pub fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    agglayer_elf_build::build_program("crates/agglayer-elf-build/sample-program/zkvm")?;
    Ok(())
}
