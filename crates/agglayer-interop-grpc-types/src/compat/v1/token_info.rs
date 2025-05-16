use agglayer_interop_types::TokenInfo;

use super::Error;
use crate::v1;

impl TryFrom<v1::TokenInfo> for TokenInfo {
    type Error = Error;

    #[inline]
    fn try_from(value: v1::TokenInfo) -> Result<Self, Self::Error> {
        Ok(TokenInfo {
            origin_network: value.origin_network.into(),
            origin_token_address: required_field!(value, origin_token_address),
        })
    }
}

impl From<TokenInfo> for v1::TokenInfo {
    #[inline]
    fn from(value: TokenInfo) -> Self {
        v1::TokenInfo {
            origin_network: value.origin_network.to_u32(),
            origin_token_address: Some(value.origin_token_address.into()),
        }
    }
}
