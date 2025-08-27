use agglayer_primitives::Digest;
use async_trait::async_trait;
use mockall::mock;

use crate::{GetBlockHash, GetBlockNumber};

mock! {
    pub Rpc {}

    #[async_trait]
    impl GetBlockHash for Rpc {
        type Error = eyre::Error;
        async fn get_block_hash(&self, block_number: u64) -> eyre::Result<Digest>;
    }

    #[async_trait]
    impl GetBlockNumber for Rpc {
        type Error = eyre::Error;
        async fn get_block_number(&self, block_hash: Digest) -> eyre::Result<u64>;
    }
}
