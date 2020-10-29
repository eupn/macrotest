use std::convert::From;

#[derive(Debug)]
pub(crate) enum Error {
    Cargo(std::io::Error),
    CargoExpandExecutionError(String),
    CargoFail,
    CargoMetadata(serde_json::error::Error),
    IoError(std::io::Error),
    TomlSerError(toml::ser::Error),
    TomlDeError(toml::de::Error),
    GlobError(glob::GlobError),
    GlobPatternError(glob::PatternError),
    ManifestDirError,
    PkgName,
    UnrecognizedEnv(std::ffi::OsString),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Error::*;

        match self {
            Cargo(e) => write!(f, "{}", e),
            CargoExpandExecutionError(e) => write!(f, "Failed to execute cargo command: {}", e),
            CargoFail => write!(f, "cargo reported an error"),
            CargoMetadata(e) => write!(f, "{}", e),
            IoError(e) => write!(f, "{}", e),
            TomlSerError(e) => write!(f, "{}", e),
            TomlDeError(e) => write!(f, "{}", e),
            GlobError(e) => write!(f, "{}", e),
            GlobPatternError(e) => write!(f, "{}", e),
            ManifestDirError => write!(f, "could not find CARGO_MANIFEST_DIR env var"),
            PkgName => write!(f, "could not find CARGO_PKG_NAME env var"),
            UnrecognizedEnv(e) => write!(
                f,
                "unrecognized value of MACROTEST: \"{}\"",
                e.to_string_lossy()
            ),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::TomlSerError(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlDeError(e)
    }
}

impl From<glob::GlobError> for Error {
    fn from(e: glob::GlobError) -> Self {
        Error::GlobError(e)
    }
}

impl From<glob::PatternError> for Error {
    fn from(e: glob::PatternError) -> Self {
        Error::GlobPatternError(e)
    }
}
