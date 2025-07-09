use alloy_primitives::Signature as AlloySignature;
use k256::ecdsa;
use rkyv::{Archive, Deserialize, Serialize};

use super::{Address, SignatureError, B256, U256};

#[derive(
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    Archive,
    Serialize,
    Deserialize,
)]
#[rkyv(remote = AlloySignature)]
struct AlloySignatureDef {
    #[rkyv(getter = AlloySignature::v)]
    y_parity: bool,
    #[rkyv(getter = AlloySignature::r, with = U256Def)]
    r: U256,
    #[rkyv(getter = AlloySignature::s, with = U256Def)]
    s: U256,
}

impl From<AlloySignatureDef> for AlloySignature {
    #[inline]
    fn from(value: AlloySignatureDef) -> Self {
        AlloySignature::new(value.r, value.s, value.y_parity)
    }
}

#[derive(Archive, Serialize, Deserialize)]
#[rkyv(remote = U256)]
pub struct U256Def {
    #[rkyv(getter = U256::as_limbs)]
    limbs: [u64; 4],
}

impl From<U256Def> for U256 {
    #[inline]
    fn from(value: U256Def) -> Self {
        U256::from_limbs(value.limbs)
    }
}

/// A wrapper over [AlloySignature] with custom serialization.
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
#[serde(from = "compat::Signature", into = "compat::Signature")]
pub struct Signature {
    #[serde(with = "AlloySignatureDef")]
    #[rkyv(with = AlloySignatureDef)]
    inner: AlloySignature,
}

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
        let signature: AlloySignature = self.inner.into();
        signature.recover_address_from_prehash(prehash)
    }

    #[inline]
    pub fn as_primitive_signature(&self) -> &AlloySignature {
        &self.inner
    }

    #[inline]
    pub fn as_bytes(&self) -> [u8; 65] {
        self.inner.as_bytes()
    }

    #[inline]
    pub fn r(&self) -> U256 {
        self.inner.r()
    }

    #[inline]
    pub fn s(&self) -> U256 {
        self.inner.s()
    }

    #[inline]
    pub fn v(&self) -> bool {
        self.inner.v()
    }
}

impl From<AlloySignature> for Signature {
    #[inline]
    fn from(ps: AlloySignature) -> Self {
        Self { inner: ps.into() }
    }
}

impl From<Signature> for AlloySignature {
    #[inline]
    fn from(value: Signature) -> Self {
        value.inner
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = SignatureError;

    #[inline]
    fn try_from(sig: &[u8]) -> Result<Self, Self::Error> {
        sig.try_into().map(|inner| Self { inner })
    }
}

impl std::str::FromStr for Signature {
    type Err = <AlloySignature as std::str::FromStr>::Err;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|inner| Self { inner })
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
            Self::new(sig.inner.r(), sig.inner.s(), sig.inner.v())
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
