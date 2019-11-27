//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/core/eosio/name.hpp#L28-L269>
mod name_type;

pub use eosio_numstr::{ParseNameError, NAME_LEN_MAX, NAME_UTF8_CHARS};

use crate::bytes::{NumBytes, Read, Write};
use alloc::string::{String, ToString};
use core::convert::TryFrom;
use core::fmt;
use core::marker::PhantomData;
use core::str::FromStr;
use eosio_numstr::{name_from_str, name_to_string};

/// TODO docs
/// TODO use `NonZeroU64`
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Hash,
    PartialOrd,
    Ord,
    Read,
    Write,
    NumBytes,
)]
#[eosio(crate_path = "crate::bytes")]
pub struct Name(u64);

impl Name {
    /// Creates a new name
    #[inline]
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// TODO docs
    #[inline]
    #[must_use]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Name {
    #[inline]
    #[must_use]
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<Name> for u64 {
    #[inline]
    #[must_use]
    fn from(i: Name) -> Self {
        i.0
    }
}

impl FromStr for Name {
    type Err = ParseNameError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = name_from_str(s)?;
        Ok(name.into())
    }
}

impl TryFrom<&str> for Name {
    type Error = ParseNameError;
    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for Name {
    type Error = ParseNameError;
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl fmt::Display for Name {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = name_to_string(self.0);
        write!(f, "{}", s)
    }
}

impl From<Name> for String {
    #[inline]
    #[must_use]
    fn from(i: Name) -> Self {
        i.to_string()
    }
}

impl PartialEq<Name> for String {
    #[inline]
    #[must_use]
    fn eq(&self, other: &Name) -> bool {
        self.as_str() == other.to_string().as_str()
    }
}

impl PartialEq<String> for Name {
    #[inline]
    #[must_use]
    fn eq(&self, other: &String) -> bool {
        self.to_string().as_str() == other.as_str()
    }
}

/// TODO docs
#[cfg(feature = "serde")]
struct NameVisitor<T: FromStr<Err = ParseNameError> + From<u64> + fmt::Display>(
    PhantomData<T>,
);

#[cfg(feature = "serde")]
impl<'de, T> serde::de::Visitor<'de> for NameVisitor<T>
where
    T: FromStr<Err = ParseNameError> + From<u64> + fmt::Display,
{
    type Value = T;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an EOSIO name string or number")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        value.parse::<T>().map_err(serde::de::Error::custom)
    }

    #[inline]
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.into())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Name {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(NameVisitor(PhantomData::<Self>))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
