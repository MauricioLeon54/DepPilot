use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::time::Duration;
use crate::package_json::DependencyKind;
use crate::package_manager::PackageManager;
use crate::update::{UpdateResult, UpdateStatus};

pub struct Output;

impl Output {
    pub fn new() -> Self {
        Self
    }

    pub fn header(&self) {
        println!();
        println!("{}", " DepPilot ".on_blue().white().bold());
        println!("{}", " Dependency Update Assistant ".dimmed());
        println!();
    }

    pub fn project_root(&self, path: &Path) {
        println!("  {}  {}", "Project".dimmed(), path.display().to_string().cyan());
    }

    pub fn package_name(&self, name: Option<&str>) {
        if let Some(n) = name {
            println!("  {}  {}", "Package".dimmed(), n.cyan().bold());
        }
    }

    pub fn package_manager_detected(&self, pm: &PackageManager) {
        let icon = match pm {
            PackageManager::Yarn => "🧶",
            PackageManager::Pnpm => "⚡",
            PackageManager::Npm => "📦",
        };
        println!("  {}  {} {}", "Manager".dimmed(), icon, pm.name().cyan().bold());
        println!();
    }

    pub fn warn_unrelated_changes(&self) {
        println!();
        println!(
            "  {}  Working tree has unrelated changes.",
            "⚠".yellow().bold()
        );
        println!(
            "     {}",
            "Committing now could stage files you did not intend to include.".dimmed()
        );
    }

    pub fn no_deps_to_update(&self) {
        println!("{}", "  No dependencies found to update.".yellow());
    }

    pub fn deps_count(&self, count: usize) {
        let word = if count == 1 { "dependency" } else { "dependencies" };
        println!(
            "  {}  {} {}",
            "Updating".dimmed(),
            count.to_string().cyan().bold(),
            word.dimmed()
        );
        println!();
    }

    pub fn updating(&self, name: &str, version: &str, kind: &DependencyKind) {
        let kind_label = match kind {
            DependencyKind::Production => "dep".normal().dimmed(),
            DependencyKind::Dev => "dev".yellow().dimmed(),
        };
        println!(
            "  {}  {}  {}  → {}",
            "↑".blue().bold(),
            name.white().bold(),
            kind_label,
            version.green()
        );
    }

    pub fn start_spinner(&self, name: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("  {spinner:.blue} {msg}")
                .unwrap(),
        );
        pb.set_message(format!("Installing {}...", name));
        pb.enable_steady_tick(Duration::from_millis(80));
        pb
    }

    pub fn update_success(&self, name: &str) {
        println!(
            "  {}  {}",
            "✓".green().bold(),
            format!("{} updated", name).dimmed()
        );
    }

    pub fn update_failed(&self, name: &str, reason: &str) {
        println!(
            "  {}  {}",
            "✗".red().bold(),
            format!("{} failed", name).white().bold()
        );
        println!("     {}", reason.dimmed());
    }

    pub fn validation_passed(&self) {
        println!("  {}  Validation passed", "✓".green());
    }

    pub fn validation_failed(&self, reason: &str) {
        println!("  {}  Validation failed", "✗".red().bold());
        if !reason.is_empty() {
            println!("     {}", reason.dimmed());
        }
    }

    pub fn commit_preview(&self, stage_cmd: &str, commit_cmd: &str) {
        println!();
        println!("  {}", "Git commands:".dimmed());
        println!("    {}", stage_cmd.cyan());
        println!("    {}", commit_cmd.cyan());
        println!();
    }

    pub fn committed(&self, message: &str) {
        println!(
            "  {}  Committed: {}",
            "✓".green().bold(),
            message.dimmed()
        );
    }

    pub fn commit_skipped(&self) {
        println!("  {}  Commit skipped", "–".dimmed());
    }

    pub fn dry_run_command(&self, pm: &str, args: &[String]) {
        println!(
            "  {}  {} {}",
            "[dry-run]".yellow(),
            pm.bold(),
            args.join(" ").cyan()
        );
    }

    pub fn aborted(&self) {
        println!();
        println!("{}", "  Aborted.".yellow());
    }

    pub fn divider(&self) {
        println!("  {}", "─".repeat(50).dimmed());
    }

    pub fn summary(&self, results: &[UpdateResult]) {
        let updated = results
            .iter()
            .filter(|r| r.status == UpdateStatus::Updated)
            .count();
        let skipped = results
            .iter()
            .filter(|r| r.status == UpdateStatus::Skipped)
            .count();
        let failed = results
            .iter()
            .filter(|r| matches!(r.status, UpdateStatus::Failed(_)))
            .count();
        let committed = results.iter().filter(|r| r.commit_created).count();

        println!();
        self.divider();
        println!("  {}", "Summary".bold());
        println!("  {}  updated", updated.to_string().green().bold());
        if skipped > 0 {
            println!("  {}  skipped", skipped.to_string().yellow());
        }
        if failed > 0 {
            println!("  {}  failed", failed.to_string().red().bold());
        }
        println!("  {}  commits created", committed.to_string().cyan());

        if failed > 0 {
            println!();
            println!("  {}", "Failed packages:".red().bold());
            for r in results {
                if let UpdateStatus::Failed(reason) = &r.status {
                    println!("    {} — {}", r.name.bold(), reason.dimmed());
                }
            }
        }

        println!();
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}
