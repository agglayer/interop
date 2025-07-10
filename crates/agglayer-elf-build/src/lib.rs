pub use anyhow::{Error, Result};

pub mod zkvm_build;

pub use zkvm_build::{build_program, ProgramBuilder};
