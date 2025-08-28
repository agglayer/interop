use agglayer_primitives::Digest;
use alloy::providers::Provider as _;
use async_trait::async_trait;

use crate::AlloyRpc;

#[async_trait]
pub trait GetBlockNumber {
    type Error;

    async fn get_block_number(&self, block_hash: Digest) -> Result<u64, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum GetBlockNumberError {
    #[error("Block hash {block_hash} not found")]
    BlockNotFound { block_hash: Digest },

    #[error("Getting information for block hash {block_hash}")]
    GettingBlock {
        block_hash: Digest,
        source: eyre::Error,
    },
}

#[async_trait]
impl<T: AlloyRpc> GetBlockNumber for T {
    type Error = GetBlockNumberError;

    async fn get_block_number(&self, block_hash: Digest) -> Result<u64, Self::Error> {
        let number = self
            .alloy_rpc()
            .get_block_by_hash(block_hash.0.into())
            .await
            .map_err(|source| GetBlockNumberError::GettingBlock {
                block_hash,
                source: source.into(),
            })?
            .ok_or(GetBlockNumberError::BlockNotFound { block_hash })?
            .header
            .number;
        Ok(number)
    }
}
