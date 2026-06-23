use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use crate::errors::DepPilotError;

pub struct Validator {
    root: PathBuf,
}

impl Validator {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Run the user-supplied shell command. Returns Ok(()) on exit code 0.
    /// Returns Err with stderr/stdout on failure.
    pub fn run(&self, check_cmd: &str, package_name: &str) -> Result<()> {
        let out = Command::new("sh")
            .args(["-c", check_cmd])
            .current_dir(&self.root)
            .output()?;

        if out.status.success() {
            return Ok(());
        }

        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        let reason = if !stderr.is_empty() { stderr } else { stdout };

        Err(DepPilotError::ValidationFailed {
            package: package_name.to_string(),
            reason,
        }
        .into())
    }
}
