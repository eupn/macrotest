use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::cargo;
use crate::dependencies::{self, Dependency};
use crate::features;
use crate::manifest::{Bin, Build, Config, Manifest, Name, Package, Workspace};
use crate::message::{message_different, message_expansion_error};
use crate::rustflags;
use crate::{error::Error, error::Result};

/// An extension for files containing `cargo expand` result.
const EXPANDED_RS_SUFFIX: &str = ".expanded.rs";

#[derive(Debug)]
pub(crate) struct Project {
    pub dir: PathBuf,
    source_dir: PathBuf,
    pub target_dir: PathBuf,
    pub name: String,
    pub features: Option<Vec<String>>,
    workspace: PathBuf,
}

/// This `Drop` implementation will clean up the temporary crates when expansion is finished.
/// This is to prevent pollution of the filesystem with dormant files.
impl Drop for Project {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.dir) {
            eprintln!(
                "Failed to cleanup the directory `{}`: {}",
                self.dir.to_string_lossy(),
                e
            );
        }
    }
}

/// Attempts to expand macros in files that match glob pattern.
///
/// # Refresh behavior
///
/// If no matching `.expanded.rs` files present, they will be created and result of expansion
/// will be written into them.
///
/// # Panics
///
/// Will panic if matching `.expanded.rs` file is present, but has different expanded code in it.
pub fn expand(path: impl AsRef<Path>) {
    run_tests(
        path,
        ExpansionBehavior::RegenerateFiles,
        Option::<Vec<String>>::None,
    );
}

/// Same as [`expand`] but allows to pass additional arguments to `cargo-expand`.
///
/// [`expand`]: expand/fn.expand.html
pub fn expand_args<I, S>(path: impl AsRef<Path>, args: I)
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<OsStr>,
{
    run_tests(path, ExpansionBehavior::RegenerateFiles, Some(args));
}

/// Attempts to expand macros in files that match glob pattern.
/// More strict version of [`expand`] function.
///
/// # Refresh behavior
///
/// If no matching `.expanded.rs` files present, it considered a failed test.
///
/// # Panics
///
/// Will panic if no matching `.expanded.rs` file is present. Otherwise it will exhibit the same
/// behavior as in [`expand`].
///
/// [`expand`]: expand/fn.expand.html
pub fn expand_without_refresh(path: impl AsRef<Path>) {
    run_tests(
        path,
        ExpansionBehavior::ExpectFiles,
        Option::<Vec<String>>::None,
    );
}

/// Same as [`expand_without_refresh`] but allows to pass additional arguments to `cargo-expand`.
///
/// [`expand_without_refresh`]: expand/fn.expand_without_refresh.html
pub fn expand_without_refresh_args<I, S>(path: impl AsRef<Path>, args: I)
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<OsStr>,
{
    run_tests(path, ExpansionBehavior::ExpectFiles, Some(args));
}

#[derive(Debug, Copy, Clone)]
enum ExpansionBehavior {
    RegenerateFiles,
    ExpectFiles,
}

fn run_tests<I, S>(path: impl AsRef<Path>, expansion_behavior: ExpansionBehavior, args: Option<I>)
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<OsStr>,
{
    let tests = expand_globs(&path)
        .into_iter()
        .filter(|t| !t.test.to_string_lossy().ends_with(EXPANDED_RS_SUFFIX))
        .collect::<Vec<_>>();

    let len = tests.len();
    println!("Running {} macro expansion tests", len);

    let project = prepare(&tests).unwrap_or_else(|err| {
        panic!("prepare failed: {:#?}", err);
    });

    let mut failures = 0;
    for test in tests {
        let file_stem = test
            .test
            .file_stem()
            .expect("no file stem")
            .to_string_lossy()
            .into_owned();

        match test.run(&project, expansion_behavior, &args) {
            Ok(outcome) => match outcome {
                ExpansionOutcome::Same => println!("{} - ok", file_stem),

                ExpansionOutcome::Different(a, b) => {
                    message_different(&file_stem, &a, &b);
                    failures += 1;
                }

                ExpansionOutcome::New(_) => {
                    let _ = writeln!(
                        std::io::stderr(),
                        "{}{} - refreshed",
                        file_stem,
                        EXPANDED_RS_SUFFIX
                    );
                }

                ExpansionOutcome::ExpandError(msg) => {
                    message_expansion_error(msg);
                    failures += 1;
                }
                ExpansionOutcome::NoExpandedFileFound => {
                    let _ = writeln!(
                        std::io::stderr(),
                        "{}.expanded.rs is expected but not found",
                        file_stem
                    );
                    failures += 1;
                }
            },

            Err(e) => {
                eprintln!("Error: {:#?}", e);
                failures += 1;
            }
        }
    }

    if failures > 0 {
        eprintln!("\n\n");
        panic!("{} of {} tests failed", failures, len);
    }
}

fn prepare(tests: &[ExpandedTest]) -> Result<Project> {
    let metadata = cargo::metadata()?;
    let target_dir = metadata.target_directory;
    let workspace = metadata.workspace_root;

    let crate_name = env::var("CARGO_PKG_NAME").map_err(|_| Error::PkgName)?;

    let source_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .ok_or(Error::ManifestDirError)?;

    let features = features::find();

    // Use random string for the crate dir to
    // prevent conflicts when running parallel tests.
    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(42).collect();

    let mut project = Project {
        dir: path!(target_dir / "tests" / crate_name / rand_string),
        source_dir,
        target_dir,
        name: format!("{}-tests", crate_name),
        features,
        workspace,
    };

    let manifest = make_manifest(crate_name, &project, tests)?;
    let manifest_toml = toml::to_string(&manifest)?;

    let config = make_config();
    let config_toml = toml::to_string(&config)?;

    if let Some(enabled_features) = &mut project.features {
        enabled_features.retain(|feature| manifest.features.contains_key(feature));
    }

    fs::create_dir_all(path!(project.dir / ".cargo"))?;
    fs::write(path!(project.dir / ".cargo" / "config"), config_toml)?;
    fs::write(path!(project.dir / "Cargo.toml"), manifest_toml)?;
    fs::write(path!(project.dir / "main.rs"), b"fn main() {}\n")?;

    cargo::build_dependencies(&project)?;

    Ok(project)
}

fn make_manifest(
    crate_name: String,
    project: &Project,
    tests: &[ExpandedTest],
) -> Result<Manifest> {
    let source_manifest = dependencies::get_manifest(&project.source_dir);
    let workspace_manifest = dependencies::get_workspace_manifest(&project.workspace);

    let features = source_manifest
        .features
        .keys()
        .map(|feature| {
            let enable = format!("{}/{}", crate_name, feature);
            (feature.clone(), vec![enable])
        })
        .collect();

    let mut manifest = Manifest {
        package: Package {
            name: project.name.clone(),
            version: "0.0.0".to_owned(),
            edition: source_manifest.package.edition,
            publish: false,
        },
        features,
        dependencies: std::collections::BTreeMap::new(),
        bins: Vec::new(),
        workspace: Some(Workspace {}),
        // Within a workspace, only the [patch] and [replace] sections in
        // the workspace root's Cargo.toml are applied by Cargo.
        patch: workspace_manifest.patch,
        replace: workspace_manifest.replace,
    };

    manifest.dependencies.extend(source_manifest.dependencies);
    manifest
        .dependencies
        .extend(source_manifest.dev_dependencies);
    manifest.dependencies.insert(
        crate_name,
        Dependency {
            version: None,
            path: Some(project.source_dir.clone()),
            default_features: false,
            features: Vec::new(),
            rest: std::collections::BTreeMap::new(),
        },
    );

    manifest.bins.push(Bin {
        name: Name(project.name.to_owned()),
        path: Path::new("main.rs").to_owned(),
    });

    for expanded in tests {
        if expanded.error.is_none() {
            manifest.bins.push(Bin {
                name: expanded.name.clone(),
                path: project.source_dir.join(&expanded.test),
            });
        }
    }

    Ok(manifest)
}

fn make_config() -> Config {
    Config {
        build: Build {
            rustflags: rustflags::make_vec(),
        },
    }
}

#[derive(Debug)]
enum ExpansionOutcome {
    Same,
    Different(Vec<u8>, Vec<u8>),
    New(Vec<u8>),
    ExpandError(Vec<u8>),
    NoExpandedFileFound,
}

struct ExpandedTest {
    name: Name,
    test: PathBuf,
    error: Option<Error>,
}

impl ExpandedTest {
    pub fn run<I, S>(
        &self,
        project: &Project,
        expansion_behavior: ExpansionBehavior,
        args: &Option<I>,
    ) -> Result<ExpansionOutcome>
    where
        I: IntoIterator<Item = S> + Clone,
        S: AsRef<OsStr>,
    {
        let (success, output_bytes) = cargo::expand(project, &self.name, args)?;

        if !success {
            return Ok(ExpansionOutcome::ExpandError(output_bytes));
        }

        let file_stem = self
            .test
            .file_stem()
            .expect("no file stem")
            .to_string_lossy()
            .into_owned();
        let mut expanded = self.test.clone();
        expanded.pop();
        let expanded = expanded.join(format!("{}{}", file_stem, EXPANDED_RS_SUFFIX));

        let output = normalize_expansion(&output_bytes);

        if !expanded.exists() {
            if let ExpansionBehavior::ExpectFiles = expansion_behavior {
                return Ok(ExpansionOutcome::NoExpandedFileFound);
            }

            // Write a .expanded.rs file contents with an newline character at the end
            std::fs::write(expanded, &format!("{}\n", output))?;

            return Ok(ExpansionOutcome::New(output_bytes));
        }

        let expected_expansion_bytes = std::fs::read(expanded)?;
        let expected_expansion = String::from_utf8_lossy(&expected_expansion_bytes);

        let same = output.lines().eq(expected_expansion.lines());

        Ok(if same {
            ExpansionOutcome::Same
        } else {
            let output_bytes = output.into_bytes(); // Use normalized text for a message
            ExpansionOutcome::Different(expected_expansion_bytes, output_bytes)
        })
    }
}

// `cargo expand` does always produce some fixed amount of lines that should be ignored
const CARGO_EXPAND_SKIP_LINES_COUNT: usize = 5;

fn normalize_expansion(input: &[u8]) -> String {
    let code = String::from_utf8_lossy(input);
    code.lines()
        .skip(CARGO_EXPAND_SKIP_LINES_COUNT)
        .collect::<Vec<_>>()
        .join("\n")
}

fn expand_globs(path: impl AsRef<Path>) -> Vec<ExpandedTest> {
    fn glob(pattern: &str) -> Result<Vec<PathBuf>> {
        let mut paths = glob::glob(pattern)?
            .map(|entry| entry.map_err(Error::from))
            .collect::<Result<Vec<PathBuf>>>()?;
        paths.sort();
        Ok(paths)
    }

    let mut vec = Vec::new();

    let name = path
        .as_ref()
        .file_stem()
        .expect("no file stem")
        .to_string_lossy()
        .to_string();
    let mut expanded = ExpandedTest {
        name: Name(name),
        test: path.as_ref().to_path_buf(),
        error: None,
    };

    if let Some(utf8) = path.as_ref().to_str() {
        if utf8.contains('*') {
            match glob(utf8) {
                Ok(paths) => {
                    for path in paths {
                        let name = path
                            .file_stem()
                            .expect("no file stem")
                            .to_string_lossy()
                            .to_string();
                        vec.push(ExpandedTest {
                            name: Name(name),
                            test: path,
                            error: None,
                        });
                    }
                }
                Err(error) => expanded.error = Some(error),
            }
        } else {
            vec.push(expanded);
        }
    }

    vec
}
