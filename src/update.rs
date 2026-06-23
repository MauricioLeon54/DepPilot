use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

use crate::cli::UpdateArgs;
use crate::git::{self, Git};
use crate::output::Output;
use crate::package_json::{Dependency, DependencyKind, PackageJson};
use crate::package_manager::{self, PackageManager};
use crate::project::ProjectRoot;
use crate::prompt::Prompt;
use crate::validation::Validator;

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateStatus {
    Updated,
    Skipped,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct UpdateResult {
    pub name: String,
    pub kind: DependencyKind,
    pub status: UpdateStatus,
    pub commit_created: bool,
}

/// Parsed representation of a user-supplied package spec such as "axios@^1.0.0".
#[derive(Debug, PartialEq)]
pub struct UpdateSpec {
    pub name: String,
    pub version: String,
}

impl UpdateSpec {
    /// Parse a package spec string. Handles:
    ///   - "axios"            → name=axios,      version=latest
    ///   - "axios@^1.0.0"    → name=axios,       version=^1.0.0
    ///   - "@scope/pkg"      → name=@scope/pkg,  version=latest
    ///   - "@scope/pkg@1.0"  → name=@scope/pkg,  version=1.0
    pub fn parse(input: &str) -> Self {
        // Skip the leading '@' of scoped packages when searching for the version separator.
        let search_from = if input.starts_with('@') { 1 } else { 0 };
        if let Some(at_offset) = input[search_from..].find('@') {
            let at_pos = search_from + at_offset;
            return UpdateSpec {
                name: input[..at_pos].to_string(),
                version: input[at_pos + 1..].to_string(),
            };
        }
        UpdateSpec {
            name: input.to_string(),
            version: "latest".to_string(),
        }
    }
}

/// Entry point called from main.
pub fn run(args: &UpdateArgs) -> Result<()> {
    let out = Output::new();
    out.header();

    // ── 1. Project root ──────────────────────────────────────────────────────
    let root = ProjectRoot::detect()?;
    out.project_root(&root.path);

    // ── 2. Package.json ──────────────────────────────────────────────────────
    let pkg = PackageJson::load(&root.package_json_path())?;
    out.package_name(pkg.name.as_deref());

    // ── 3. Package manager ───────────────────────────────────────────────────
    let pm = resolve_package_manager(&root, args)?;
    out.package_manager_detected(&pm);

    // ── 4. Verify PM binary is on PATH ───────────────────────────────────────
    package_manager::verify_installed(&pm)?;

    // ── 5. Git working tree safety check ────────────────────────────────────
    let git = Git::new(root.path.clone());
    if git.is_repo() && !args.no_commit && !args.dry_run {
        let safe = [&"package.json" as &str, pm.lock_file()];
        if git.has_unrelated_changes(&safe)? && !args.force {
            out.warn_unrelated_changes();
            if !args.yes && !Prompt::confirm("Continue anyway?")? {
                out.aborted();
                return Ok(());
            }
        }
    }

    // ── 6. Resolve which dependencies to update ───────────────────────────────
    let deps = resolve_deps(&pkg, args);
    if deps.is_empty() {
        out.no_deps_to_update();
        return Ok(());
    }
    out.deps_count(deps.len());

    // ── 7. Update loop ───────────────────────────────────────────────────────
    let mut results: Vec<UpdateResult> = Vec::new();
    for dep in &deps {
        let result = update_one(dep, &pm, &root, &git, args, &out)?;
        results.push(result);
    }

    // ── 8. Summary ───────────────────────────────────────────────────────────
    out.summary(&results);
    Ok(())
}

// ── Private helpers ───────────────────────────────────────────────────────────

fn resolve_package_manager(root: &ProjectRoot, args: &UpdateArgs) -> Result<PackageManager> {
    match package_manager::detect(root, args.package_manager.as_deref()) {
        Ok(pm) => Ok(pm),
        Err(e) if e.to_string().contains("Multiple lock files") => {
            eprintln!("\n  ⚠  {}", e);
            let items = ["yarn", "pnpm", "npm"];
            let idx = Prompt::select("Which package manager should DepPilot use?", &items)?;
            Ok(PackageManager::from_str(items[idx]).unwrap())
        }
        Err(e) => Err(e),
    }
}

fn resolve_deps(pkg: &PackageJson, args: &UpdateArgs) -> Vec<Dependency> {
    let only_dev = args.only.as_deref() == Some("dev");
    let only_prod = args.only.as_deref() == Some("deps");

    if args.packages.is_empty() {
        return pkg
            .all_dependencies()
            .into_iter()
            .filter(|d| {
                if only_prod {
                    d.kind == DependencyKind::Production
                } else if only_dev {
                    d.kind == DependencyKind::Dev
                } else {
                    true
                }
            })
            .collect();
    }

    args.packages
        .iter()
        .map(|spec_str| {
            let spec = UpdateSpec::parse(spec_str);
            match pkg.find(&spec.name) {
                Some(mut d) => {
                    d.version = spec.version;
                    d
                }
                None => Dependency {
                    name: spec.name,
                    version: spec.version,
                    kind: if only_dev {
                        DependencyKind::Dev
                    } else {
                        DependencyKind::Production
                    },
                },
            }
        })
        .collect()
}

fn update_one(
    dep: &Dependency,
    pm: &PackageManager,
    root: &ProjectRoot,
    git: &Git,
    args: &UpdateArgs,
    out: &Output,
) -> Result<UpdateResult> {
    out.updating(&dep.name, &dep.version, &dep.kind);

    if args.dry_run {
        let install_args = pm.install_args(&dep.name, &dep.version, &dep.kind);
        out.dry_run_command(pm.name(), &install_args);
        return Ok(UpdateResult {
            name: dep.name.clone(),
            kind: dep.kind.clone(),
            status: UpdateStatus::Updated,
            commit_created: false,
        });
    }

    // ── Run package manager ──────────────────────────────────────────────────
    let spinner = out.start_spinner(&dep.name);
    let install_args = pm.install_args(&dep.name, &dep.version, &dep.kind);
    let ok = run_pm(pm.name(), &install_args, &root.path, args.force)?;
    spinner.finish_and_clear();

    if !ok {
        let reason = "Package manager exited with a non-zero code".to_string();
        out.update_failed(&dep.name, &reason);
        return Ok(UpdateResult {
            name: dep.name.clone(),
            kind: dep.kind.clone(),
            status: UpdateStatus::Failed(reason),
            commit_created: false,
        });
    }

    out.update_success(&dep.name);

    // ── Validation ───────────────────────────────────────────────────────────
    if let Some(check_cmd) = &args.check {
        let validator = Validator::new(root.path.clone());
        match validator.run(check_cmd, &dep.name) {
            Ok(()) => out.validation_passed(),
            Err(e) => {
                let msg = e.to_string();
                out.validation_failed(&msg);
                if !args.continue_on_error {
                    return Ok(UpdateResult {
                        name: dep.name.clone(),
                        kind: dep.kind.clone(),
                        status: UpdateStatus::Failed(format!("Validation: {}", msg)),
                        commit_created: false,
                    });
                }
            }
        }
    }

    // ── Commit ───────────────────────────────────────────────────────────────
    if args.no_commit || !git.is_repo() {
        return Ok(UpdateResult {
            name: dep.name.clone(),
            kind: dep.kind.clone(),
            status: UpdateStatus::Updated,
            commit_created: false,
        });
    }

    let default_msg = git::build_commit_message(&args.commit_template, &dep.name, &dep.version);
    let files = ["package.json", pm.lock_file()];
    let (stage_cmd, commit_cmd) = git::preview_commands(&files, &default_msg);
    out.commit_preview(&stage_cmd, &commit_cmd);

    let final_msg = if args.yes {
        Some(default_msg)
    } else {
        Prompt::edit_commit_message(&default_msg)?
    };

    match final_msg {
        None => {
            out.commit_skipped();
            Ok(UpdateResult {
                name: dep.name.clone(),
                kind: dep.kind.clone(),
                status: UpdateStatus::Updated,
                commit_created: false,
            })
        }
        Some(msg) => {
            git.stage_files(&files)?;
            git.commit(&msg)?;
            out.committed(&msg);
            Ok(UpdateResult {
                name: dep.name.clone(),
                kind: dep.kind.clone(),
                status: UpdateStatus::Updated,
                commit_created: true,
            })
        }
    }
}

fn run_pm(pm_name: &str, args: &[String], cwd: &PathBuf, force: bool) -> Result<bool> {
    let status = Command::new(pm_name).args(args).current_dir(cwd).status()?;
    Ok(status.success() || force)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name_only() {
        let s = UpdateSpec::parse("axios");
        assert_eq!(s.name, "axios");
        assert_eq!(s.version, "latest");
    }

    #[test]
    fn parse_name_at_semver() {
        let s = UpdateSpec::parse("axios@^1.18.1");
        assert_eq!(s.name, "axios");
        assert_eq!(s.version, "^1.18.1");
    }

    #[test]
    fn parse_name_at_latest() {
        let s = UpdateSpec::parse("vue@latest");
        assert_eq!(s.name, "vue");
        assert_eq!(s.version, "latest");
    }

    #[test]
    fn parse_scoped_package_no_version() {
        let s = UpdateSpec::parse("@scope/pkg");
        assert_eq!(s.name, "@scope/pkg");
        assert_eq!(s.version, "latest");
    }

    #[test]
    fn parse_scoped_package_with_version() {
        let s = UpdateSpec::parse("@scope/pkg@1.2.3");
        assert_eq!(s.name, "@scope/pkg");
        assert_eq!(s.version, "1.2.3");
    }

    #[test]
    fn parse_tilde_version() {
        let s = UpdateSpec::parse("lodash@~4.17.0");
        assert_eq!(s.name, "lodash");
        assert_eq!(s.version, "~4.17.0");
    }
}
