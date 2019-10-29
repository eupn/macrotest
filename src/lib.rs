#![crate_type = "lib"]

use derive_more::From;
use failure::Fail;

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::thread;

pub mod common;
mod expand;
mod message;

#[derive(Debug, Fail, From)]
pub enum Error {
    #[fail(display = "Failed to execute `cargo expand`: {}", _0)]
    CargoExpandExecutionError(String),

    #[fail(display = "I/O error: {}", _0)]
    IoError(#[cause] std::io::Error),

    #[fail(display = "TOML serialization error: {}", _0)]
    TomlSerError(#[cause] toml::ser::Error),

    #[fail(display = "TOML deserialization error: {}", _0)]
    TomlDeError(#[cause] toml::de::Error),

    #[fail(display = "Glob error: {}", _0)]
    GlobError(#[cause] glob::GlobError),

    #[fail(display = "Glob pattern error: {}", _0)]
    GlobPatternError(#[cause] glob::PatternError),

    #[fail(display = "No CARGO_MANIFEST_DIR env var")]
    ManifestDirError,

    #[fail(display = "No CARGO_PKG_NAME env var")]
    PkgName,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum ExpansionOutcome {
    Same,
    Different(Vec<u8>, Vec<u8>),
    New(Vec<u8>),
    ExpandError(Vec<u8>),
}

#[derive(Debug)]
pub struct TestCases {
    inner: RefCell<Expander>,
}

#[derive(Debug)]
struct Expander {
    tests: Vec<Test>,
}

#[derive(Debug, Copy, Clone)]
enum Expected {
    Pass,

    #[allow(dead_code)]
    CompileFail,
}

#[derive(Clone, Debug)]
struct Test {
    path: PathBuf,
    expected: Expected,
}

impl TestCases {
    pub fn new() -> Self {
        TestCases {
            inner: RefCell::new(Expander { tests: Vec::new() }),
        }
    }

    pub fn pass<P: AsRef<Path>>(&self, path: P) {
        self.inner.borrow_mut().tests.push(Test {
            path: path.as_ref().to_owned(),
            expected: Expected::Pass,
        });
    }
}

#[doc(hidden)]
impl Drop for TestCases {
    fn drop(&mut self) {
        if !thread::panicking() {
            self.inner.borrow_mut().expand();
        }
    }
}
