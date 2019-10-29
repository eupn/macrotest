use std::path::PathBuf;
use std::process::Command;

use tempdir::TempDir;
use toml::{map::Map, Value};

use crate::{Error, Expander, ExpansionOutcome, Result, Test};
use crate::message::{message_different, message_expansion_error};

use std::env;

impl Expander {
    pub fn expand(&mut self) {
        let tests = expand_globs(&self.tests)
            .into_iter()
            .filter(|t| !t.test.path.to_string_lossy().ends_with(".expanded.rs"))
            .collect::<Vec<_>>();

        let len = tests.len();
        println!("Running {} macro expansion tests", len);

        let mut failures = 0;
        for test in tests {
            let file_stem = test
                .test
                .path
                .file_stem()
                .expect("no file stem")
                .to_string_lossy();

            match test.run() {
                Ok(outcome) => match outcome {
                    ExpansionOutcome::Same => println!("{} - ok", file_stem),

                    ExpansionOutcome::Different(a, b) => {
                        message_different(&file_stem, &a, &b);
                        failures += 1;
                    }

                    ExpansionOutcome::New(_) => println!("{} - refreshed", file_stem),

                    ExpansionOutcome::ExpandError(msg) => {
                        message_expansion_error(msg);
                        failures += 1;
                    }
                },

                Err(e) => {
                    eprintln!("Error: {:#?}", e);
                    failures += 1;
                }
            }
        }

        println!("\n\n");

        if failures > 0 {
            panic!("{} or {} tests failed", failures, len);
        }
    }
}

struct ExpandedTest {
    test: Test,
    error: Option<Error>,
}

impl ExpandedTest {
    pub fn run(&self) -> Result<ExpansionOutcome> {
        let temp_crate = make_tmp_cargo_crate_for_src(&self.test.path)?;
        let (success, output) = expand_crate(&temp_crate)?;

        if !success {
            return Ok(ExpansionOutcome::ExpandError(output))
        }

        let file_stem = self
            .test
            .path
            .file_stem()
            .expect("no file stem")
            .to_string_lossy();
        let mut expanded = self.test.path.clone();
        expanded.pop();
        let expanded = expanded.join(format!("{}.expanded.rs", file_stem));

        if !expanded.exists() {
            std::fs::write(expanded, &output)?;
            std::fs::remove_dir_all(&temp_crate)?;

            return Ok(ExpansionOutcome::New(output));
        }

        let expected_expansion = std::fs::read(expanded)?;
        std::fs::remove_dir_all(&temp_crate)?;

        Ok(if output == expected_expansion {
            ExpansionOutcome::Same
        } else {
            ExpansionOutcome::Different(expected_expansion, output)
        })
    }
}

fn expand_globs(tests: &[Test]) -> Vec<ExpandedTest> {
    fn glob(pattern: &str) -> Result<Vec<PathBuf>> {
        let mut paths = glob::glob(pattern)?
            .map(|entry| entry.map_err(Error::from))
            .collect::<Result<Vec<PathBuf>>>()?;
        paths.sort();
        Ok(paths)
    }

    let mut vec = Vec::new();

    for test in tests {
        let mut expanded = ExpandedTest {
            test: test.clone(),
            error: None,
        };
        if let Some(utf8) = test.path.to_str() {
            if utf8.contains('*') {
                match glob(utf8) {
                    Ok(paths) => {
                        for path in paths {
                            vec.push(ExpandedTest {
                                test: Test {
                                    path,
                                    expected: expanded.test.expected,
                                },
                                error: None,
                            });
                        }
                        continue;
                    }
                    Err(error) => expanded.error = Some(error),
                }
            }
        }
        vec.push(expanded);
    }

    vec
}

#[allow(dead_code)]
fn fix_relative_paths_in_deps(
    root_path: &PathBuf,
    dd: &toml::value::Table,
    deps: &mut toml::map::Map<String, toml::value::Value>,
) {
    for (k, v) in dd {
        let v = match v {
            toml::value::Value::Table(map) => {
                let mut new_map = map.clone();
                if let Some(p) = new_map.get_mut("path") {
                    let path = PathBuf::from(p.as_str().expect("path as string"));
                    if path.is_relative() {
                        let new_path =
                            std::fs::canonicalize(root_path.join(path)).expect("canonicalize path");
                        *p = toml::value::Value::String(new_path.to_string_lossy().to_string());
                    }
                };

                toml::value::Value::Table(new_map)
            }
            _ => v.clone(),
        };

        deps.insert(k.clone(), v);
    }
}

pub fn make_tmp_cargo_crate_for_src(src_path: &PathBuf) -> Result<PathBuf> {
    let crate_name = env::var("CARGO_PKG_NAME").map_err(|_| Error::PkgName)?;
    let source_dir = env::var_os("CARGO_MANIFEST_DIR")
        .ok_or(Error::ManifestDirError)
        .map(PathBuf::from)?;

    /*
    // Copy dev-dependencies as dependencies for the test crate
    let manifest_file = source_dir.join("Cargo.toml");
    let manifest = std::fs::read_to_string(manifest_file)?;
    let manifest: toml::Value = toml::from_str(&manifest)?;

    let dev_dependencies = manifest.get("dev-dependencies").and_then(|dd| dd.as_table());
    if let Some(dd) = dev_dependencies {
        fix_relative_paths_in_deps(&source_dir, dd, &mut deps);
    };
    */

    let mut deps = toml::map::Map::new();
    let mut map = toml::map::Map::new();
    map.insert(
        "path".to_string(),
        toml::Value::String(source_dir.to_string_lossy().to_string()),
    );
    deps.insert(crate_name, toml::Value::Table(map));

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
    cargo_toml.insert("dependencies".into(), Value::Table(deps.clone()));

    let cargo_toml = toml::to_string_pretty(&cargo_toml)?;

    std::fs::write(dir_path.join("Cargo.toml"), cargo_toml)?;

    std::fs::create_dir(dir_path.join("src"))?;
    std::fs::copy(src_path, dir_path.join("src").join("main.rs"))?;

    Ok(dir_path)
}

pub fn expand_crate(path: &PathBuf) -> Result<(bool, Vec<u8>)> {
    let cargo_expand = Command::new("cargo")
        .arg("expand")
        .current_dir(path)
        .output()
        .map_err(|e| Error::CargoExpandExecutionError(e.to_string()))?;

    if !cargo_expand.status.success() {
        return Ok((false, cargo_expand.stderr));
    }

    Ok((true, cargo_expand.stdout))
}
