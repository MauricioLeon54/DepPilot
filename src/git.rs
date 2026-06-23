use crate::errors::DepPilotError;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub struct Git {
    root: PathBuf,
}

impl Git {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn is_repo(&self) -> bool {
        Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .current_dir(&self.root)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Returns true if the working tree contains changes to files outside of `safe_files`.
    pub fn has_unrelated_changes(&self, safe_files: &[&str]) -> Result<bool> {
        let out = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&self.root)
            .output()?;

        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if line.len() < 3 {
                continue;
            }
            let file = line[3..].trim();
            let is_safe = safe_files.iter().any(|&s| file == s || file.ends_with(s));
            if !is_safe {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn stage_files(&self, files: &[&str]) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.arg("add");
        for f in files {
            cmd.arg(f);
        }
        let out = cmd.current_dir(&self.root).output()?;
        if !out.status.success() {
            return Err(DepPilotError::GitError(
                String::from_utf8_lossy(&out.stderr).trim().to_string(),
            )
            .into());
        }
        Ok(())
    }

    pub fn commit(&self, message: &str) -> Result<()> {
        let out = Command::new("git")
            .args(["commit", "-m", message])
            .current_dir(&self.root)
            .output()?;
        if !out.status.success() {
            return Err(DepPilotError::GitError(
                String::from_utf8_lossy(&out.stderr).trim().to_string(),
            )
            .into());
        }
        Ok(())
    }
}

/// Build a commit message from a template.
/// Supported placeholders: {name}, {version}
pub fn build_commit_message(template: &str, name: &str, version: &str) -> String {
    template
        .replace("{name}", name)
        .replace("{version}", version)
}

/// Format the git commands that will be previewed to the user before execution.
pub fn preview_commands(files: &[&str], message: &str) -> (String, String) {
    let stage = format!("git add {}", files.join(" "));
    let commit = format!("git commit -m \"{}\"", message);
    (stage, commit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_template_substitution() {
        let msg = build_commit_message("chore: update {name}", "axios", "^1.0.0");
        assert_eq!(msg, "chore: update axios");
    }

    #[test]
    fn template_with_version_placeholder() {
        let msg = build_commit_message(
            "chore(deps): update {name} to {version}",
            "react",
            "^18.2.0",
        );
        assert_eq!(msg, "chore(deps): update react to ^18.2.0");
    }

    #[test]
    fn preview_commands_format() {
        let (stage, commit) =
            preview_commands(&["package.json", "yarn.lock"], "chore: update axios");
        assert_eq!(stage, "git add package.json yarn.lock");
        assert_eq!(commit, r#"git commit -m "chore: update axios""#);
    }

    #[test]
    fn template_with_no_placeholders() {
        let msg = build_commit_message("chore: bump deps", "axios", "latest");
        assert_eq!(msg, "chore: bump deps");
    }
}
