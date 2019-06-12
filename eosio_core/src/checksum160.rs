//! TODO docs
use eosio_bytes::{NumBytes, Read, Write};

/// TODO docs
#[derive(
    Read,
    Write,
    NumBytes,
    Default,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct Checksum160([u8; 20]);

impl Checksum160 {
    /// TODO docs.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// TODO docs.
    pub const fn to_bytes(&self) -> [u8; 20] {
        self.0
    }
}

impl From<[u8; 20]> for Checksum160 {
    #[inline]
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl From<Checksum160> for [u8; 20] {
    #[inline]
    fn from(value: Checksum160) -> Self {
        value.0
    }
}