use crate::errors::DepPilotError;
use crate::package_json::DependencyKind;
use crate::project::ProjectRoot;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum PackageManager {
    Yarn,
    Pnpm,
    Npm,
}

impl PackageManager {
    pub fn name(&self) -> &str {
        match self {
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Npm => "npm",
        }
    }

    pub fn lock_file(&self) -> &str {
        match self {
            PackageManager::Yarn => "yarn.lock",
            PackageManager::Pnpm => "pnpm-lock.yaml",
            PackageManager::Npm => "package-lock.json",
        }
    }

    /// Build the install command arguments for a given package and dependency kind.
    pub fn install_args(&self, name: &str, version: &str, kind: &DependencyKind) -> Vec<String> {
        let versioned = format_versioned(name, version);
        match self {
            PackageManager::Yarn => {
                let mut args = vec!["add".to_string(), versioned];
                if *kind == DependencyKind::Dev {
                    args.push("--dev".to_string());
                }
                args
            }
            PackageManager::Pnpm => {
                let mut args = vec!["add".to_string(), versioned];
                if *kind == DependencyKind::Dev {
                    args.push("-D".to_string());
                }
                args
            }
            PackageManager::Npm => {
                let flag = match kind {
                    DependencyKind::Production => "--save",
                    DependencyKind::Dev => "--save-dev",
                };
                vec!["install".to_string(), versioned, flag.to_string()]
            }
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "yarn" => Some(PackageManager::Yarn),
            "pnpm" => Some(PackageManager::Pnpm),
            "npm" => Some(PackageManager::Npm),
            _ => None,
        }
    }
}

fn format_versioned(name: &str, version: &str) -> String {
    if version.is_empty() || version == "latest" {
        format!("{}@latest", name)
    } else {
        format!("{}@{}", name, version)
    }
}

/// Detect which package manager to use for the project.
/// Returns Err(MultipleLockFiles) when multiple lock files are present and no override is given.
pub fn detect(root: &ProjectRoot, override_pm: Option<&str>) -> Result<PackageManager> {
    if let Some(pm_str) = override_pm {
        return PackageManager::from_str(pm_str).ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown package manager '{}'. Valid options: yarn, pnpm, npm.",
                pm_str
            )
        });
    }

    let found: Vec<PackageManager> = [
        ("yarn.lock", PackageManager::Yarn),
        ("pnpm-lock.yaml", PackageManager::Pnpm),
        ("package-lock.json", PackageManager::Npm),
    ]
    .into_iter()
    .filter(|(lock, _)| root.has_file(lock))
    .map(|(_, pm)| pm)
    .collect();

    match found.len() {
        0 => Err(DepPilotError::NoLockFile.into()),
        1 => Ok(found.into_iter().next().unwrap()),
        _ => Err(DepPilotError::MultipleLockFiles.into()),
    }
}

/// Return all lock files present in the project root (used for conflict detection).
pub fn detected_lock_files(root: &ProjectRoot) -> Vec<(&'static str, PackageManager)> {
    [
        ("yarn.lock", PackageManager::Yarn),
        ("pnpm-lock.yaml", PackageManager::Pnpm),
        ("package-lock.json", PackageManager::Npm),
    ]
    .into_iter()
    .filter(|(lock, _)| root.has_file(lock))
    .collect()
}

/// Verify that the package manager binary is present in PATH.
pub fn verify_installed(pm: &PackageManager) -> Result<()> {
    which::which(pm.name())
        .map(|_| ())
        .map_err(|_| DepPilotError::PackageManagerNotFound(pm.name().to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_root(files: &[&str]) -> (TempDir, ProjectRoot) {
        let dir = TempDir::new().unwrap();
        for f in files {
            fs::write(dir.path().join(f), "").unwrap();
        }
        let root = ProjectRoot {
            path: dir.path().to_path_buf(),
        };
        (dir, root)
    }

    #[test]
    fn detects_yarn() {
        let (_dir, root) = make_root(&["yarn.lock"]);
        assert_eq!(detect(&root, None).unwrap(), PackageManager::Yarn);
    }

    #[test]
    fn detects_pnpm() {
        let (_dir, root) = make_root(&["pnpm-lock.yaml"]);
        assert_eq!(detect(&root, None).unwrap(), PackageManager::Pnpm);
    }

    #[test]
    fn detects_npm() {
        let (_dir, root) = make_root(&["package-lock.json"]);
        assert_eq!(detect(&root, None).unwrap(), PackageManager::Npm);
    }

    #[test]
    fn errors_on_no_lock_file() {
        let (_dir, root) = make_root(&[]);
        let err = detect(&root, None).unwrap_err();
        assert!(err.to_string().contains("No lock file"));
    }

    #[test]
    fn errors_on_multiple_lock_files() {
        let (_dir, root) = make_root(&["yarn.lock", "package-lock.json"]);
        let err = detect(&root, None).unwrap_err();
        assert!(err.to_string().contains("Multiple lock files"));
    }

    #[test]
    fn override_bypasses_detection() {
        let (_dir, root) = make_root(&["yarn.lock", "package-lock.json"]);
        assert_eq!(detect(&root, Some("npm")).unwrap(), PackageManager::Npm);
    }

    #[test]
    fn yarn_prod_install_args() {
        let args =
            PackageManager::Yarn.install_args("axios", "^1.0.0", &DependencyKind::Production);
        assert_eq!(args, vec!["add", "axios@^1.0.0"]);
    }

    #[test]
    fn yarn_dev_install_args() {
        let args = PackageManager::Yarn.install_args("jest", "latest", &DependencyKind::Dev);
        assert_eq!(args, vec!["add", "jest@latest", "--dev"]);
    }

    #[test]
    fn pnpm_dev_install_args() {
        let args = PackageManager::Pnpm.install_args("vitest", "^1.0.0", &DependencyKind::Dev);
        assert_eq!(args, vec!["add", "vitest@^1.0.0", "-D"]);
    }

    #[test]
    fn npm_prod_install_args() {
        let args =
            PackageManager::Npm.install_args("react", "^18.0.0", &DependencyKind::Production);
        assert_eq!(args, vec!["install", "react@^18.0.0", "--save"]);
    }

    #[test]
    fn npm_dev_install_args() {
        let args = PackageManager::Npm.install_args("eslint", "latest", &DependencyKind::Dev);
        assert_eq!(args, vec!["install", "eslint@latest", "--save-dev"]);
    }

    #[test]
    fn scoped_package_formatted_correctly() {
        let args = PackageManager::Npm.install_args("@types/node", "^20.0.0", &DependencyKind::Dev);
        assert_eq!(args, vec!["install", "@types/node@^20.0.0", "--save-dev"]);
    }
}
