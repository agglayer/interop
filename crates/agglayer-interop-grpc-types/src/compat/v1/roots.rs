use agglayer_interop_types::roots::LocalExitRoot;
use prost::bytes::Bytes;

use super::Error;
use crate::v1;

impl TryFrom<v1::FixedBytes32> for LocalExitRoot {
    type Error = Error;

    #[inline]
    fn try_from(value: v1::FixedBytes32) -> Result<Self, Self::Error> {
        Ok(LocalExitRoot::from(<[u8; 32]>::try_from(value)?))
    }
}

impl From<LocalExitRoot> for v1::FixedBytes32 {
    #[inline]
    fn from(value: LocalExitRoot) -> Self {
        v1::FixedBytes32 {
            value: Bytes::copy_from_slice(value.as_ref()),
        }
    }
}
