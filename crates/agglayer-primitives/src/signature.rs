use alloy_primitives::Signature as AlloySignature;
use k256::ecdsa;

use super::{Address, SignatureError, B256, U256};

/// A wrapper over [AlloySignature] with custom serialization.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
#[serde(from = "compat::Signature", into = "compat::Signature")]
pub struct Signature(AlloySignature);

impl Signature {
    #[inline]
    pub fn from_signature_and_parity(sig: ecdsa::Signature, v: bool) -> Self {
        AlloySignature::from_signature_and_parity(sig, v).into()
    }

    #[inline]
    pub fn new(r: U256, s: U256, v: bool) -> Self {
        AlloySignature::new(r, s, v).into()
    }

    #[inline]
    pub fn recover_address_from_prehash(&self, prehash: &B256) -> Result<Address, SignatureError> {
        self.0.recover_address_from_prehash(prehash).map(Address::from)
    }

    #[inline]
    pub fn as_primitive_signature(&self) -> &AlloySignature {
        &self.0
    }

    #[inline]
    pub fn as_bytes(&self) -> [u8; 65] {
        self.0.as_bytes()
    }

    #[inline]
    pub fn r(&self) -> U256 {
        self.0.r()
    }

    #[inline]
    pub fn s(&self) -> U256 {
        self.0.s()
    }

    #[inline]
    pub fn v(&self) -> bool {
        self.0.v()
    }
}

impl From<AlloySignature> for Signature {
    #[inline]
    fn from(ps: AlloySignature) -> Self {
        Self(ps)
    }
}

impl From<Signature> for AlloySignature {
    #[inline]
    fn from(value: Signature) -> Self {
        value.0
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = SignatureError;

    #[inline]
    fn try_from(sig: &[u8]) -> Result<Self, Self::Error> {
        sig.try_into().map(Self)
    }
}

impl std::str::FromStr for Signature {
    type Err = <AlloySignature as std::str::FromStr>::Err;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

/// Helpers for serialization / deserialization format compatibility.
mod compat {
    use super::U256;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Signature {
        r: U256,
        s: U256,
        odd_y_parity: bool,
    }

    impl Signature {
        #[inline]
        fn new(r: U256, s: U256, odd_y_parity: bool) -> Self {
            Self { r, s, odd_y_parity }
        }
    }

    impl From<super::Signature> for Signature {
        #[inline]
        fn from(sig: super::Signature) -> Self {
            Self::new(sig.0.r(), sig.0.s(), sig.0.v())
        }
    }

    impl From<Signature> for super::Signature {
        #[inline]
        fn from(sig: Signature) -> Self {
            let Signature { r, s, odd_y_parity } = sig;
            Self::new(r, s, odd_y_parity)
        }
    }
}
