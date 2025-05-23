pub use bincode::{Error, Options, Result};

/// Bincode configuration. Deliberately inaccessible from the outside.
mod options {
    use bincode::config::{
        AllowTrailing, BigEndian, Bounded, DefaultOptions as BincodeDefaultOptions, FixintEncoding,
        Options as _, WithOtherEndian, WithOtherIntEncoding, WithOtherLimit, WithOtherTrailing,
    };

    pub type Default =
        WithOtherIntEncoding<WithOtherEndian<BincodeDefaultOptions, BigEndian>, FixintEncoding>;

    #[inline]
    pub fn default() -> Default {
        bincode::options().with_big_endian().with_fixint_encoding()
    }

    pub type SP1v4 = WithOtherTrailing<
        WithOtherIntEncoding<BincodeDefaultOptions, FixintEncoding>,
        AllowTrailing,
    >;

    #[inline]
    pub fn sp1v4() -> SP1v4 {
        bincode::options()
            .with_fixint_encoding()
            .allow_trailing_bytes()
    }

    pub type Limited<T> = WithOtherLimit<T, Bounded>;
}

/// Bincode codec with opinionated settings.
#[derive(Clone, Debug)]
pub struct Codec<Opts>(Opts);

/// Create a bincode codec with default agglayer settings.
#[inline]
pub fn default() -> Codec<options::Default> {
    Codec(options::default())
}

/// Create a bincode codec with settings used by `sp1`.
#[inline]
pub fn sp1v4() -> Codec<options::SP1v4> {
    Codec(options::sp1v4())
}

/// Create a bincode coded with settings used by smart contract verifiers.
///
/// This happens to be the same as [default] but with a more accurate name.
#[inline]
pub fn contracts() -> Codec<options::Default> {
    default()
}

impl<Opts: Options> Codec<Opts> {
    /// Impose a limit on encoding / decoding size.
    #[inline]
    pub fn with_limit(self, max: u64) -> Codec<options::Limited<Opts>> {
        Codec(self.0.with_limit(max))
    }

    /// Encode an object into a byte vector.
    #[inline]
    pub fn serialize<T>(self, item: &T) -> Result<Vec<u8>>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.serialize(item)
    }

    /// Encode an object into a writer.
    #[inline]
    pub fn serialize_into<W, T>(self, writer: W, item: &T) -> Result<()>
    where
        W: std::io::Write,
        T: ?Sized + serde::Serialize,
    {
        self.0.serialize_into(writer, item)
    }

    /// Decode an object from a slice.
    #[inline]
    pub fn deserialize<'a, T>(self, bytes: &'a [u8]) -> Result<T>
    where
        T: serde::Deserialize<'a>,
    {
        self.0.deserialize(bytes)
    }

    /// Decode an object from a reader.
    #[inline]
    pub fn deserialize_from<T, R>(self, reader: R) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        R: std::io::Read,
    {
        self.0.deserialize_from(reader)
    }
}

#[cfg(test)]
mod test {
    use unified_bridge::NetworkId;

    #[test]
    fn sp1_endians() {
        let network_id = NetworkId::new(0x00112233);
        let network_id_enc = super::sp1v4().serialize(&network_id).unwrap();

        let mut stdin0 = sp1_sdk::SP1Stdin::new();
        stdin0.write_slice(&network_id_enc);

        let mut stdin1 = sp1_sdk::SP1Stdin::new();
        stdin1.write(&network_id);

        assert_eq!(&stdin1.buffer[0], &[0x33, 0x22, 0x11, 0x00]);
        assert_eq!(stdin0.buffer, stdin1.buffer);
        assert_eq!(stdin0.read::<NetworkId>(), stdin1.read::<NetworkId>());
    }
}
