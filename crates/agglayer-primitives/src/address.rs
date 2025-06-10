pub use alloy_primitives::Address as AlloyAddress;

/// A wrapper over [alloy_primitives::Address] that allows us to add custom
/// methods and trait implementations.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
    derive_more::Into,
    derive_more::FromStr,
    derive_more::AsRef,
    derive_more::Display,
    derive_more::LowerHex,
    derive_more::UpperHex,
)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
#[from(AlloyAddress, [u8; 20])]
#[into(AlloyAddress, [u8; 20])]
#[as_ref(AlloyAddress, [u8; 20], [u8])]
#[repr(transparent)]
#[serde(rename = "agglayer_primitives::Address")]
pub struct Address(AlloyAddress);

impl Address {
    pub const ZERO: Self = Self::from_alloy(AlloyAddress::ZERO);

    #[inline]
    pub const fn new(bytes: [u8; 20]) -> Self {
        Self::from_alloy(AlloyAddress::new(bytes))
    }

    #[inline]
    pub const fn from_alloy(address: AlloyAddress) -> Self {
        Self(address)
    }

    #[inline]
    pub const fn as_alloy(&self) -> &AlloyAddress {
        &self.0
    }

    #[inline]
    pub const fn into_alloy(self) -> AlloyAddress {
        self.0
    }

    #[inline]
    pub const fn into_array(self) -> [u8; 20] {
        self.into_alloy().into_array()
    }

    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        self.as_alloy().0.as_slice()
    }

    #[inline]
    pub fn copy_from_slice(&mut self, slice: &[u8]) {
        self.0.copy_from_slice(slice);
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = std::array::TryFromSliceError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        AlloyAddress::try_from(value).map(Self)
    }
}

#[macro_export]
macro_rules! address {
    ($($addr:literal)*) => {
        $crate::Address::from_alloy($crate::alloy_primitives::address!($($addr)*))
    };
}
