use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

pub type HashU32 = [u32; 8];

/// SP1 verifying key hash.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(from = "B256", into = "B256")]
pub struct VKeyHash(HashU32);

impl VKeyHash {
    pub const fn from_hash_u32(hash: HashU32) -> Self {
        Self(hash)
    }

    pub const fn from_bytes(bytes: B256) -> Self {
        let bytes = bytes.0;
        let mut hash_u32 = HashU32::default();

        for (w, c) in bytes.chunks(4).enumerate() {
            hash_u32[w] = u32::from_be_bytes(c.try_into().unwrap());
        }

        Self(hash_u32)
    }

    #[cfg(feature = "sp1")]
    pub fn from_vkey<K: sp1_sdk::HashableKey>(vkey: &K) -> Self {
        Self::from_hash_u32(vkey.hash_u32())
    }

    pub const fn to_bytes(&self) -> B256 {
        let mut bytes = [0_u8; 32];

        for w in 0..8 {
            bytes[4 * w..4 * w + 4].copy_from_slice(self.0[w].to_be_bytes());
        }

        B256::new(bytes)
    }

    pub const fn to_hash_u32(&self) -> HashU32 {
        self.0
    }
}

impl From<B256> for VKeyHash {
    fn from(bytes: B256) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<VKeyHash> for B256 {
    fn from(hash: VKeyHash) -> Self {
        hash.to_bytes()
    }
}

impl std::str::FromStr for VKeyHash {
    type Err = <B256 as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self::from_bytes)
    }
}

impl std::fmt::Debug for VKeyHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_bytes().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use alloy_primitives::b256;

    use super::*;

    #[test]
    fn constructors_consistently_be() {
        let from_hash_u32 = VKeyHash::from_hash_u32([
            0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f, 0x10111213, 0x14151617, 0x18191a1b,
            0x1c1d1e1f,
        ]);

        let from_hex = VKeyHash::from_bytes(b256!(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        ));

        assert_eq!(from_hash_u32, from_hex);

        let roundtrip = VKeyHash::from_bytes(from_hash_u32.to_bytes());
        assert_eq!(from_hash_u32, roundtrip);
    }
}
