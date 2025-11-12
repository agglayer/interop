use std::{env, fs, path::Path};

use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Metadata, MetadataCommand,
};
use eyre::{eyre, Context, OptionExt};
use sp1_build::BuildArgs;

use super::Mode;

pub struct ProgramBuilder {
    /// Directory with the zkvm program source.
    program_dir: Utf8PathBuf,

    /// The path to the cached zkvm ELF binary.
    cached_elf_path: Utf8PathBuf,

    /// SP1 build arguments.
    build_args: BuildArgs,

    /// Zkvm program Cargo metadata.
    program_metadata: Metadata,
}

impl ProgramBuilder {
    /// New zkvm ELF builder.
    pub fn new(relative_program_dir: impl AsRef<Utf8Path>) -> eyre::Result<Self> {
        let host_metadata = MetadataCommand::new()
            .no_deps()
            .exec()
            .context("Host workspace metadata extraction failed")?;

        let program_dir = host_metadata.workspace_root.join(relative_program_dir);

        let program_dir_meta = fs::metadata(&program_dir)
            .with_context(|| format!("Checking the program dir ({program_dir})"))?;
        eyre::ensure!(
            program_dir_meta.is_dir(),
            "Program path is not a directory ({program_dir})",
        );

        let manifest_dir = env::var("CARGO_MANIFEST_DIR").context("Cannot obtain manifest dir")?;
        let cached_elf_path = Utf8Path::new(&manifest_dir).join("elf/riscv32im-succinct-zkvm-elf");

        let program_metadata = MetadataCommand::new()
            .no_deps()
            .current_dir(&program_dir)
            .exec()
            .context("Program workspace metadata extraction failed")?;

        let build_args = BuildArgs {
            docker: true,
            tag: String::from(super::DEFAULT_DOCKER_TAG),
            workspace_directory: Some(host_metadata.workspace_root.to_string()),
            ..Default::default()
        };

        Ok(Self {
            program_dir,
            cached_elf_path,
            build_args,
            program_metadata,
        })
    }

    /// Use the specified docker tag for the toolchain image instead of the
    /// default one.
    pub fn docker_tag(mut self, docker_tag: impl Into<String>) -> Self {
        self.build_args.tag = docker_tag.into();
        self
    }

    /// Specify the path where the cached zkvm ELF binary lives.
    pub fn cached_elf_path(mut self, cached_elf_path: impl Into<Utf8PathBuf>) -> Self {
        self.cached_elf_path = cached_elf_path.into();
        self
    }

    /// Add build arguments. Same as passed to `cargo prove`.
    pub fn add_args(mut self, extra_args: impl IntoIterator<Item = String>) -> Self {
        clap::Parser::update_from(&mut self.build_args, extra_args);
        self
    }

    /// Add arguments passed through to `rustc`.
    pub fn add_rustflags(mut self, extra_args: impl IntoIterator<Item = String>) -> Self {
        self.build_args.rustflags.extend(extra_args);
        self
    }

    /// Get the path to the produced zkvm ELF in the build folder.
    fn built_elf_path(&self) -> eyre::Result<Utf8PathBuf> {
        let args = &self.build_args;
        let mut paths_iter = sp1_build::generate_elf_paths(&self.program_metadata, Some(args))
            .map_err(|e| eyre!(e))
            .context("Failed to extract zkvm ELF paths")?
            .into_iter();

        let (_package, path) = paths_iter.next().ok_or_eyre("No zkvm ELF paths")?;
        eyre::ensure!(paths_iter.next().is_none(), "Too many zkvm ELF paths");

        Ok(path)
    }

    fn copy_elf(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> eyre::Result<()> {
        // Create all the necessary directories first
        if let Some(cached_elf_dir) = destination.as_ref().parent() {
            fs::create_dir_all(cached_elf_dir)
                .context("Failed to create directory for zkvm ELF")?;
        }

        // Copy to a temporary and move so it's less sensitive to partial writes when
        // the build is interrupted.
        let source_tmp = Path::new(&env::var("OUT_DIR").context("Getting build OUT_DIR")?)
            .join(source.as_ref().file_name().unwrap_or("zkvm-elf".as_ref()))
            .with_extension(".temporary");
        fs::copy(source, source_tmp.as_path()).context("Failed to copy zkvm ELF")?;
        fs::rename(source_tmp, destination).context("Failed to move zkvm ELF")?;

        Ok(())
    }

    fn build_program(self) -> eyre::Result<Utf8PathBuf> {
        eprintln!("Program dir: {}", self.program_dir.as_str());

        let elf_path = self.built_elf_path()?;
        sp1_build::build_program_with_args(self.program_dir.as_str(), self.build_args);
        println!("cargo::rustc-env=AGGLAYER_ELF_PATH={elf_path}");

        Ok(elf_path)
    }

    fn build_and_refresh(self) -> eyre::Result<Utf8PathBuf> {
        let cached_elf_path = self.cached_elf_path.clone();

        let elf_path = self.build_program()?;
        Self::copy_elf(elf_path.as_path(), cached_elf_path).context("Copying zkvm ELF to cache")?;

        Ok(elf_path)
    }

    fn take_from_cache(self) -> eyre::Result<Utf8PathBuf> {
        let cached_elf_path = self.cached_elf_path;

        println!("cargo::rerun-if-changed={cached_elf_path}");
        println!("cargo::rustc-env=AGGLAYER_ELF_PATH={cached_elf_path}");

        Ok(cached_elf_path)
    }

    /// Run with given mode or config.
    pub fn run_mode(self, mode: Mode) -> eyre::Result<Utf8PathBuf> {
        match mode {
            Mode::Build => self.build_program(),
            Mode::Refresh => self.build_and_refresh(),
            Mode::Cached => self.take_from_cache(),
        }
    }

    /// Run the zkvm ELF builder.
    pub fn run(self) -> eyre::Result<Utf8PathBuf> {
        self.run_mode(Mode::from_env()?)
    }
}
