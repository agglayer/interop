use bincode::Options as _;

use super::Error;

mod types_to_v1;
mod v1_to_types;

#[inline]
fn sp1v4_bincode_options() -> impl bincode::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
}
