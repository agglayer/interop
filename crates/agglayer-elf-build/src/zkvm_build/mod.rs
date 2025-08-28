//! Building zkvm ELF binaries, Agglayer-style.
//!
//! This module provides a build script library that can be used to build
//! zkvm ELF binaries. It is based on [sp1_build] but there is a couple
//! differences to capture the build patterns used in Agglayer.
//!
//! * The docker-based deterministic build is always used.
//! * The path to the zkvm program crate is specified relative to the host
//!   workspace root.
//! * The ELF binary is also emitted into the source tree so it can be checked
//!   into the source tree.
//! * By default, the checked in ELF is used so the project can be built without
//!   the zkvm toolcahin.
//!
//! ## Controlling the build process using `AGGLAYER_ELF_BUILD`
//!
//! The build mode is specified using the `AGGLAYER_ELF_BUILD` environment
//! variable.
//!
//! * Set it to 'build' to rebuild the program from source (requires docker).
//! * Set it to 'refresh' to rebuild the program from source and update the
//!   checked in binary (requires docker).
//! * Set it to 'cached' to use the cached checked-in binary (default).
//!
//! After the build mode, the `AGGLAYER_ELF_BUILD` variable may contain extra
//! arguments to be passed to the build. The accepted arguments are the same as
//! what `cargo prove build` accepts.
//!
//! Example: `AGGLAYER_ELF_BUILD="refresh --warning-level=minimal" cargo build`.
//!
//! ## Building a zkvm ELF binary
//!
//! Decide which crate will expose the ELF binary.
//!
//! Inside that crate, use [build_program] and point it to the zkvm program
//! crate. Note the path is relative to the workspace root:
//!
//! ```ignore
//! fn main() -> eyre::Result<()> {
//!     color_eyre::install()?;
//!     agglayer_elf_build::build_program("crates/whatever-proof-program")?;
//!     Ok(())
//! }
//! ```
//!
//! If more customization is required, [ProgramBuilder] may be used instead.
//!
//! In the crate source, include the resulting binary like this:
//!
//! ```ignore
//! pub const ELF: &[u8] = include_bytes!(env!("AGGLAYER_ELF_PATH"));
//! ```
//!
//! ## Caveats
//!
//! (1) Due to limitations of Rust build scripts, it is not possible to depend
//! the cached zkvm ELF. Something like
//! `include_bytes!("path/to/cached/elf/riscv32im-succinct-zkvm-elf")` is prone
//! to build time race conditions. Instead, depend on the crate and use the
//! `elf_crate::ELF` constant defined as suggested above.
//!
//! (2) By default, `cargo build` and similar commands do not emit output coming
//! from build scipts. In this case, the build script builds the whole proof
//! program including its dependencies. It may appear that the build is stuck
//! for some time. Passing `-vv` to cargo makes it emit the build script output
//! but it also makes the whole output much noisier.
//!
//! (3) The build script does not currently make any attempt to communicate the
//! number of CPU cores taken up by currently running host build to the cargo
//! command in the Docker container. This may result in too much concurrency.
//!
//! Note that (2) and (3) also apply to upstream [sp1_build].

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};

pub use self::{mode::Mode, program_builder::ProgramBuilder};

mod mode;
mod program_builder;

pub const DEFAULT_DOCKER_TAG: &str =
    "v5.0.0@sha256:52d9e522d1dcbc4062edee950391173aed752793c33361fb2cad44272027a68c";

/// Path to the cached zkvm ELF binary, relative to `build.rs`.
pub const CACHED_ELF_PATH: &str = "elf/riscv32im-succinct-zkvm-elf";

/// Convenience function to build the zkvm ELF if no customization is needed.
pub fn build_program(program_dir: impl AsRef<Utf8Path>) -> eyre::Result<Utf8PathBuf> {
    ProgramBuilder::new(program_dir)?.run()
}

#[macro_export]
macro_rules! elf_bytes {
    () => {
        ::std::include_bytes!(::std::env!("AGGLAYER_ELF_PATH"))
    };
}
