#![crate_type = "lib"]

use failure::Fail;
use derive_more::From;

use crate::common::Config;
use std::path::PathBuf;

pub mod common;
mod expand;

#[derive(Debug, Fail, From)]
pub enum Error {
    #[fail(display = "I/O error: {}", _0)]
    IoError(#[cause] std::io::Error),

    #[fail(display = "TOML serialization error: {}", _0)]
    TomlSerError(#[cause] toml::ser::Error)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum ExpansionOutcome {
    Same,
    Different,
    New,
}

fn expand_and_compare(config: &Config, src: &PathBuf, expanded: &PathBuf) -> Result<ExpansionOutcome> {
    let tmp_crate = expand::make_tmp_cargo_crate_for_src(&config.dependencies, src)?;
    let expansion = expand::expand_crate(&tmp_crate)?;

    if !expanded.exists() {
        std::fs::write(expanded, &expansion)?;
        std::fs::remove_dir_all(&tmp_crate)?;

        return Ok(ExpansionOutcome::New);
    }

    let expected_expansion = std::fs::read(expanded)?;
    std::fs::remove_dir_all(&tmp_crate)?;

    Ok(if expansion == expected_expansion {
        ExpansionOutcome::Same
    } else {
        ExpansionOutcome::Different
    })
}

pub fn run_tests(config: &Config) -> Result<()> {
    let dir = std::fs::read_dir(&config.src_base)?;

    let files = dir
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let files = files.into_iter()
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().to_string_lossy().ends_with(".rs"))
        .filter(|entry| !entry.path().to_string_lossy().ends_with(".expanded.rs"))
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    for file in files {
        let file_stem = file.file_stem().expect("no file stem").to_string_lossy();
        let mut expanded = file.clone();
        expanded.pop();
        let expanded = expanded.join(format!("{}.expanded.rs", file_stem));

        match expand_and_compare(config, &file, &expanded)? {
            ExpansionOutcome::Same => println!("{} - ok", file_stem),
            ExpansionOutcome::Different => println!("{} - different!", file_stem),
            ExpansionOutcome::New => println!("{} - refreshed", file_stem),
        }
    }

    Ok(())
}
