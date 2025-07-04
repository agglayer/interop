use agglayer_interop_types::Address;
use prost::bytes::Bytes;

use super::Error;
use crate::v1;

impl TryFrom<v1::FixedBytes20> for Address {
    type Error = Error;

    #[inline]
    fn try_from(value: v1::FixedBytes20) -> Result<Self, Self::Error> {
        Ok(Address::from(<[u8; 20]>::try_from(value)?))
    }
}

impl From<Address> for v1::FixedBytes20 {
    #[inline]
    fn from(value: Address) -> Self {
        v1::FixedBytes20 {
            value: Bytes::copy_from_slice(value.as_slice()),
        }
    }
}
