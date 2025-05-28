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
)]
#[cfg_attr(feature = "testutils", derive(arbitrary::Arbitrary))]
pub struct Address(AlloyAddress);

impl Address {
    pub const ZERO: Self = Self::from_alloy(AlloyAddress::ZERO);

    pub const fn new(bytes: [u8; 20]) -> Self {
        Self::from_alloy(AlloyAddress::new(bytes))
    }

    pub const fn from_alloy(address: AlloyAddress) -> Self {
        Self(address)
    }

    pub const fn as_slice(&self) -> &[u8] {
        self.as_alloy().0.as_slice()
    }

    pub const fn as_alloy(&self) -> &AlloyAddress {
        &self.0
    }

    pub fn copy_from_slice(&mut self, slice: &[u8]) {
        self.0.copy_from_slice(slice);
    }
}

impl From<AlloyAddress> for Address {
    fn from(value: AlloyAddress) -> Self {
        Self::from_alloy(value)
    }
}

#[macro_export]
macro_rules! address {
    ($($addr:literal)*) => {
        $crate::Address::from_alloy($crate::alloy_primitives::address!($($addr)*))
    };
}
