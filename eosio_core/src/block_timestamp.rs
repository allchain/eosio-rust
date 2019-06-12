//! TODO docs
use eosio_bytes::*;
use serde::Serialize;

/// Block timestamp as milliseconds relative to the year 2000.
#[derive(
    Read,
    Write,
    NumBytes,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Clone,
    Copy,
    Hash,
    Default,
    Serialize,
)]
#[eosio_bytes_root_path = "::eosio_bytes"]
pub struct BlockTimestamp(u32);

impl BlockTimestamp {
    /// Time between blocks.
    pub const BLOCK_INTERVAL_MS: i32 = 500;
    /// Epoch is 2000-01-01T00:00.000Z.
    pub const BLOCK_TIMESTAMP_EPOCH: i64 = 946_684_800_000;

    /// Gets the milliseconds
    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

/// TODO docs
struct BlockTimestampVisitor;

impl<'de> ::serde::de::Visitor<'de> for BlockTimestampVisitor {
    type Value = BlockTimestamp;

    #[inline]
    fn expecting(
        &self,
        formatter: &mut ::std::fmt::Formatter,
    ) -> ::std::fmt::Result {
        formatter.write_str("a second timestamp as a number or string")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        match value.parse::<u32>() {
            Ok(n) => Ok(BlockTimestamp(n)),
            Err(e) => Err(::serde::de::Error::custom(e)),
        }
    }

    #[inline]
    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(BlockTimestamp(value))
    }
}

impl<'de> ::serde::de::Deserialize<'de> for BlockTimestamp {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(BlockTimestampVisitor)
    }
}

impl From<u32> for BlockTimestamp {
    #[inline]
    fn from(i: u32) -> Self {
        Self(i)
    }
}

impl From<BlockTimestamp> for u32 {
    #[inline]
    fn from(t: BlockTimestamp) -> Self {
        t.0
    }
}