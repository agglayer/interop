use agglayer_interop_types::L1InfoTreeLeaf;

use super::Error;
use crate::v1;

impl TryFrom<v1::L1InfoTreeLeafWithContext> for L1InfoTreeLeaf {
    type Error = Error;

    #[inline]
    fn try_from(value: v1::L1InfoTreeLeafWithContext) -> Result<Self, Self::Error> {
        Ok(L1InfoTreeLeaf {
            l1_info_tree_index: value.l1_info_tree_index,
            rer: required_field!(value, rer),
            mer: required_field!(value, mer),
            block_hash: required_field!(value, block_hash),
            timestamp: value.timestamp,
        })
    }
}

impl From<L1InfoTreeLeaf> for v1::L1InfoTreeLeafWithContext {
    #[inline]
    fn from(value: L1InfoTreeLeaf) -> Self {
        v1::L1InfoTreeLeafWithContext {
            l1_info_tree_index: value.l1_info_tree_index,
            rer: Some(value.rer.into()),
            mer: Some(value.mer.into()),
            block_hash: Some(value.block_hash.into()),
            timestamp: value.timestamp,
        }
    }
}
