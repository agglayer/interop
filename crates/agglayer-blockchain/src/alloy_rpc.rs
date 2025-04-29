use alloy::{
    network::Ethereum,
    providers::{
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        Identity, RootProvider,
    },
};

pub type AlloyFillProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
    Ethereum,
>;

// Send + Sync bound required in order to use `AlloyRpc` in async contexts.
pub trait AlloyRpc: Send + Sync {
    fn alloy_rpc(&self) -> &AlloyFillProvider;
}
