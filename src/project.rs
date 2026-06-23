use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::errors::DepPilotError;

pub struct ProjectRoot {
    pub path: PathBuf,
}

impl ProjectRoot {
    /// Walk up from cwd to find the nearest directory containing package.json.
    pub fn detect() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        find_project_root(&cwd)
            .ok_or_else(|| DepPilotError::NoPackageJson(cwd.display().to_string()).into())
    }

    pub fn package_json_path(&self) -> PathBuf {
        self.path.join("package.json")
    }

    pub fn has_file(&self, name: &str) -> bool {
        self.path.join(name).exists()
    }
}

fn find_project_root(start: &Path) -> Option<ProjectRoot> {
    let mut current = start.to_path_buf();
    loop {
        if current.join("package.json").exists() {
            return Some(ProjectRoot { path: current });
        }
        if !current.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn finds_package_json_in_current_dir() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        let root = find_project_root(dir.path());
        assert!(root.is_some());
        assert_eq!(root.unwrap().path, dir.path());
    }

    #[test]
    fn finds_package_json_in_parent() {
        let dir = TempDir::new().unwrap();
        let sub = dir.path().join("packages").join("app");
        fs::create_dir_all(&sub).unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        let root = find_project_root(&sub);
        assert!(root.is_some());
        assert_eq!(root.unwrap().path, dir.path());
    }

    #[test]
    fn returns_none_when_no_package_json() {
        let dir = TempDir::new().unwrap();
        let root = find_project_root(dir.path());
        assert!(root.is_none());
    }
}
