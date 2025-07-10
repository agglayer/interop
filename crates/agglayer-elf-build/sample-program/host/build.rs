pub fn main() -> agglayer_elf_build::Result<()> {
    agglayer_elf_build::build_program("crates/agglayer-elf-build/sample-program/zkvm").map(drop)
}
