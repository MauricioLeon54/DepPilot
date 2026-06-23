use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use anyhow::Result;
use crate::errors::DepPilotError;

#[derive(Debug, Clone, PartialEq)]
pub enum DependencyKind {
    Production,
    Dev,
}

impl DependencyKind {
    pub fn label(&self) -> &str {
        match self {
            DependencyKind::Production => "dep",
            DependencyKind::Dev => "dev",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub kind: DependencyKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, String>,
    #[serde(rename = "devDependencies", default)]
    pub dev_dependencies: BTreeMap<String, String>,
}

impl PackageJson {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| DepPilotError::ParseError(e.to_string()))?;
        serde_json::from_str(&raw)
            .map_err(|e| DepPilotError::ParseError(format!("Invalid JSON: {}", e)).into())
    }

    /// All dependencies combined: production first, then dev.
    pub fn all_dependencies(&self) -> Vec<Dependency> {
        let mut deps: Vec<Dependency> = self
            .dependencies
            .iter()
            .map(|(name, version)| Dependency {
                name: name.clone(),
                version: version.clone(),
                kind: DependencyKind::Production,
            })
            .collect();

        let dev: Vec<Dependency> = self
            .dev_dependencies
            .iter()
            .map(|(name, version)| Dependency {
                name: name.clone(),
                version: version.clone(),
                kind: DependencyKind::Dev,
            })
            .collect();

        deps.extend(dev);
        deps
    }

    /// Look up a dependency by name, checking both sections.
    pub fn find(&self, name: &str) -> Option<Dependency> {
        if let Some(v) = self.dependencies.get(name) {
            return Some(Dependency {
                name: name.to_string(),
                version: v.clone(),
                kind: DependencyKind::Production,
            });
        }
        self.dev_dependencies.get(name).map(|v| Dependency {
            name: name.to_string(),
            version: v.clone(),
            kind: DependencyKind::Dev,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_pkg(content: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "{}", content).unwrap();
        f
    }

    #[test]
    fn parses_both_sections() {
        let f = write_pkg(r#"{"dependencies":{"axios":"^1.0.0"},"devDependencies":{"jest":"^29.0.0"}}"#);
        let pkg = PackageJson::load(f.path()).unwrap();
        assert_eq!(pkg.dependencies["axios"], "^1.0.0");
        assert_eq!(pkg.dev_dependencies["jest"], "^29.0.0");
    }

    #[test]
    fn all_dependencies_preserves_kind() {
        let f = write_pkg(r#"{"dependencies":{"axios":"^1.0.0"},"devDependencies":{"jest":"^29.0.0"}}"#);
        let pkg = PackageJson::load(f.path()).unwrap();
        let all = pkg.all_dependencies();
        let axios = all.iter().find(|d| d.name == "axios").unwrap();
        let jest = all.iter().find(|d| d.name == "jest").unwrap();
        assert_eq!(axios.kind, DependencyKind::Production);
        assert_eq!(jest.kind, DependencyKind::Dev);
    }

    #[test]
    fn find_returns_correct_kind() {
        let f = write_pkg(r#"{"dependencies":{"react":"^18.0.0"},"devDependencies":{"vitest":"^1.0.0"}}"#);
        let pkg = PackageJson::load(f.path()).unwrap();
        assert_eq!(pkg.find("react").unwrap().kind, DependencyKind::Production);
        assert_eq!(pkg.find("vitest").unwrap().kind, DependencyKind::Dev);
        assert!(pkg.find("nonexistent").is_none());
    }

    #[test]
    fn handles_empty_sections() {
        let f = write_pkg(r#"{"name":"my-app"}"#);
        let pkg = PackageJson::load(f.path()).unwrap();
        assert!(pkg.all_dependencies().is_empty());
    }
}
