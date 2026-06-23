use thiserror::Error;

#[derive(Error, Debug)]
pub enum DepPilotError {
    #[error("No package.json found starting from '{0}'")]
    NoPackageJson(String),

    #[error("Failed to parse package.json: {0}")]
    ParseError(String),

    #[error("Multiple lock files detected. Use --package-manager to specify one.")]
    MultipleLockFiles,

    #[error("No lock file found. Cannot detect package manager. Use --package-manager to override.")]
    NoLockFile,

    #[error("Package manager '{0}' not found in PATH")]
    PackageManagerNotFound(String),

    #[error("Update failed for '{package}': {reason}")]
    UpdateFailed { package: String, reason: String },

    #[error("Validation failed for '{package}': {reason}")]
    ValidationFailed { package: String, reason: String },

    #[error("Git error: {0}")]
    GitError(String),

    #[error("Working tree has unrelated changes. Use --force to continue anyway.")]
    UnrelatedChanges,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
