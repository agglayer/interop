pub fn main() -> agglayer_build::Result<()> {
    agglayer_build::build_program("crates/agglayer-build/sample-program/zkvm").map(drop)
}
