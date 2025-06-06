// Helper macro used by the rest of this module
macro_rules! required_field {
    ($from:expr, $field:ident) => {
        $from
            .$field
            .ok_or(Error::missing_field(stringify!($field)))?
            .try_into()
            .map_err(|e: Error| e.inside_field(stringify!($field)))?
    };
}

mod address;
mod aggchain_data;
mod bridge_exit;
mod bytes;
mod claim;
mod digest;
mod error;
mod global_index;
mod imported_bridge_exit;
mod l1_info_tree_leaf;
mod merkle_proof;
mod token_info;
mod u256;

pub use error::{Error, ErrorKind};

mod roots;
#[cfg(test)]
pub mod tests;
