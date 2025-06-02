use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum SmtError {
    #[error("trying to insert a key already in the SMT")]
    KeyAlreadyPresent,
    #[error("trying to generate a Merkle proof for a key not in the SMT")]
    KeyNotPresent,
    #[error("trying to generate a non-inclusion proof for a key present in the SMT")]
    KeyPresent,
    #[error("depth out of bounds")]
    DepthOutOfBounds,
}
