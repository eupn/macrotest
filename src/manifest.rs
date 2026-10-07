use crate::dependencies::{Dependency, Patch, RegistryPatch};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap as Map;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Serialize, Debug)]
pub struct Manifest {
    #[serde(rename = "cargo-features")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cargo_features: Vec<String>,
    pub package: Package,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub features: Map<String, Vec<String>>,
    pub dependencies: Map<String, Dependency>,
    #[serde(rename = "dev-dependencies")]
    pub dev_dependencies: Map<String, Dependency>,
    #[serde(rename = "bin")]
    pub bins: Vec<Bin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Workspace>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub patch: Map<String, RegistryPatch>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub replace: Map<String, Patch>,
}

#[derive(Serialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub publish: bool,
    pub edition: Edition,
}

// Do not use enum for edition for future-compatibility.
#[derive(Serialize, Deserialize, Debug)]
pub struct Edition(pub Value);

#[derive(Serialize, Debug)]
pub struct Bin {
    pub name: Name,
    pub path: PathBuf,
}

#[derive(Serialize, Clone, Debug)]
pub struct Name(pub String);

#[derive(Serialize, Debug)]
pub struct Config {
    pub build: Build,
}

#[derive(Serialize, Debug)]
pub struct Build {
    pub rustflags: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Workspace {
    #[serde(skip_serializing_if = "WorkspacePackage::is_none")]
    pub package: WorkspacePackage,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub dependencies: Map<String, Dependency>,
}

#[derive(Serialize, Debug)]
pub struct WorkspacePackage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
}

impl WorkspacePackage {
    fn is_none(&self) -> bool {
        self.edition.is_none()
    }
}

impl Default for Edition {
    fn default() -> Self {
        Self("2021".into())
    }
}

impl AsRef<OsStr> for Name {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}
