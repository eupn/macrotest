#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate test_procmacro_project;
struct Test;
struct Hello;
