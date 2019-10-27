use std::path::PathBuf;

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub kind: DependencyKind,
    pub version: String,
}

#[derive(Debug)]
pub enum DependencyKind {
    CratesIo,
    Path(PathBuf),
}

#[derive(Debug)]
pub struct Config {
    pub dependencies: Vec<Dependency>,
    pub src_base: PathBuf,
}
