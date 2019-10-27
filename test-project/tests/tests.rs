use macrotest::{run_tests, common::{Config, Dependency, DependencyKind}};
use std::env::current_dir;

#[test]
pub fn run() {
    let curr_dir = current_dir().expect("curr dir env");
    let config = Config {
        dependencies: vec![
            Dependency {
                name: "test-project".to_string(),
                kind: DependencyKind::Path(curr_dir),
                version: "0.1.0".to_string()
            }
        ],
        src_base: std::path::PathBuf::from("tests/expand"),
    };

    run_tests(&config);
}