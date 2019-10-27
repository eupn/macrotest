use std::path::PathBuf;
use std::process::Command;

use tempdir::TempDir;
use toml::{map::Map, Value};

use crate::common::{Dependency, DependencyKind};

use crate::Result;

pub fn make_tmp_cargo_crate_for_src(dependencies: &[Dependency], src_path: &PathBuf) -> Result<PathBuf> {
    // test_case.rs -> test_case-rs
    let file_name = src_path
        .file_name()
        .expect("File name")
        .to_string_lossy()
        .replace(".", "-");

    let temp_dir = TempDir::new("macrotest")?;
    let dir_path = temp_dir.into_path();

    let mut cargo_toml = Map::new();
    let mut package = Map::new();
    package.insert("name".into(), Value::String(file_name));
    package.insert("version".into(), Value::String("0.1.0".into()));

    cargo_toml.insert("package".into(), Value::Table(package));

    let mut deps = Map::new();
    for dep in dependencies {
        let key = dep.name.clone();
        let mut values = Map::new();
        values.insert("version".into(), Value::String(dep.version.clone()));

        if let DependencyKind::Path(path) = &dep.kind {
            values.insert("path".into(), Value::String(path.to_string_lossy().into()));
        }

        deps.insert(key, Value::Table(values));
    }

    cargo_toml.insert("dependencies".into(), Value::Table(deps));

    let cargo_toml = toml::to_string_pretty(&cargo_toml)?;

    std::fs::write(dir_path.join("Cargo.toml"), cargo_toml)?;

    std::fs::create_dir(dir_path.join("src"))?;
    std::fs::copy(src_path, dir_path.join("src").join("main.rs"))?;

    Ok(dir_path)
}

pub fn expand_crate(path: &PathBuf) -> Result<Vec<u8>> {
    let cargo_expand = Command::new("cargo")
        .arg("expand")
        .current_dir(path)
        .output()?;

    if !cargo_expand.status.success() {
        return Ok(cargo_expand.stderr);
    }

    Ok(cargo_expand.stdout)
}
