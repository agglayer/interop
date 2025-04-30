use agglayer_primitives::Digest;
use alloy::providers::Provider as _;
use async_trait::async_trait;

use crate::AlloyRpc;

#[async_trait]
pub trait GetBlockHash {
    type Error;

    async fn get_block_hash(&self, block_number: u64) -> Result<Digest, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum GetBlockHashError {
    #[error("Block number {block_number} not found")]
    BlockNotFound { block_number: u64 },

    #[error("Getting information for block number {block_number}")]
    GettingBlock {
        block_number: u64,
        source: anyhow::Error,
    },
}

#[async_trait]
impl<T: AlloyRpc> GetBlockHash for T {
    type Error = GetBlockHashError;

    async fn get_block_hash(&self, block_number: u64) -> Result<Digest, Self::Error> {
        let hash = self
            .alloy_rpc()
            .get_block_by_number(block_number.into())
            .await
            .map_err(|source| GetBlockHashError::GettingBlock {
                block_number,
                source: source.into(),
            })?
            .ok_or(GetBlockHashError::BlockNotFound { block_number })?
            .header
            .hash;
        Ok(Digest::from(<[u8; 32]>::from(hash)))
    }
}
