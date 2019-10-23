//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L249-L274>
use crate::account::AccountName;
use crate::bytes::{NumBytes, Read, Write};
use crate::name::ParseNameError;
use crate::name_type;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

name_type!(ActionName);
name_type!(PermissionName);

/// This is the packed representation of an action along with meta-data about
/// the authorization levels.
#[derive(
    Clone, Debug, Serialize, Deserialize, Read, Write, NumBytes, Default,
)]
#[__eosio_path = "crate::bytes"]
pub struct Action<T> {
    /// Name of the account the action is intended for
    pub account: AccountName,
    /// Name of the action
    pub name: ActionName,
    /// List of permissions that authorize this action
    pub authorization: Vec<PermissionLevel>,
    /// Payload data
    pub data: T,
}

/// TODO docs
pub trait ToAction: Write + NumBytes {
    /// TODO docs
    const NAME: ActionName;

    /// TODO docs
    #[inline]
    fn to_action(
        &self,
        account: AccountName,
        authorization: Vec<PermissionLevel>,
    ) -> Action<Vec<u8>> {
        let mut data = vec![0_u8; self.num_bytes()];
        self.write(&mut data, &mut 0).expect("write");

        Action {
            account,
            name: Self::NAME,
            authorization,
            data,
        }
    }
}

/// TODO docs.
pub trait ActionFn: ToAction + Read + Write + NumBytes + Clone {
    /// TODO docs.
    fn call(self);
}

/// A permission
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Default,
    Read,
    Write,
    NumBytes,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[__eosio_path = "crate::bytes"]
pub struct PermissionLevel {
    /// TODO docs
    pub actor: AccountName,
    /// TODO docs
    pub permission: PermissionName,
}

/// TODO docs
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParsePermissionLevelError {
    /// TODO docs
    InvalidFormat,
    /// TODO docs
    InvalidActor(ParseNameError),
    /// TODO docs
    InvalidPermission(ParseNameError),
}

impl Error for ParsePermissionLevelError {}

impl fmt::Display for ParsePermissionLevelError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(
                f,
                "invalid format, must be in the format 'actor@permission'"
            ),
            Self::InvalidActor(err) => write!(f, "invalid actor name: {}", err),
            Self::InvalidPermission(err) => {
                write!(f, "invalid permission name: {}", err)
            }
        }
    }
}

impl FromStr for PermissionLevel {
    type Err = ParsePermissionLevelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('@');
        let actor = parts
            .next()
            .ok_or(ParsePermissionLevelError::InvalidFormat)?;
        let actor = actor
            .parse::<AccountName>()
            .map_err(ParsePermissionLevelError::InvalidActor)?;
        let permission = parts
            .next()
            .ok_or(ParsePermissionLevelError::InvalidFormat)?;
        let permission = permission
            .parse::<PermissionName>()
            .map_err(ParsePermissionLevelError::InvalidPermission)?;
        if parts.next().is_none() {
            Ok(Self { actor, permission })
        } else {
            Err(ParsePermissionLevelError::InvalidFormat)
        }
    }
}

impl TryFrom<&str> for PermissionLevel {
    type Error = ParsePermissionLevelError;
    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for PermissionLevel {
    type Error = ParsePermissionLevelError;
    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl fmt::Display for PermissionLevel {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.actor.as_name(), self.permission.as_name())
    }
}

#[cfg(test)]
mod permission_level_tests {
    use super::*;
    use eosio_macros::n;

    #[test]
    fn test_from_str_ok() {
        let ok = Ok(PermissionLevel {
            actor: AccountName::new(n!(hello)),
            permission: PermissionName::new(n!(world)),
        });
        assert_eq!(PermissionLevel::from_str("hello@world"), ok);
        assert_eq!(PermissionLevel::try_from("hello@world"), ok);
    }

    #[test]
    fn test_from_str_invalid_format() {
        for input in &["hello", "hello@world@"] {
            assert_eq!(
                PermissionLevel::from_str(input),
                Err(ParsePermissionLevelError::InvalidFormat)
            );
        }
    }

    #[test]
    fn test_from_str_invalid_actor() {
        for (input, expected) in &[
            ("", ParseNameError::Empty),
            (" ", ParseNameError::BadChar(' ')),
            ("@world", ParseNameError::Empty),
            ("hello6", ParseNameError::BadChar('6')),
            ("HELLO", ParseNameError::BadChar('H')),
            ("hellohellohello", ParseNameError::TooLong),
        ] {
            assert_eq!(
                PermissionLevel::from_str(input),
                Err(ParsePermissionLevelError::InvalidActor(*expected))
            );
        }
    }

    #[test]
    fn test_from_str_invalid_permission() {
        for (input, expected) in &[
            ("hello@", ParseNameError::Empty),
            ("hello@ ", ParseNameError::BadChar(' ')),
            ("hello@world6", ParseNameError::BadChar('6')),
            ("hello@WORLD", ParseNameError::BadChar('W')),
            ("hello@worldworldworld", ParseNameError::TooLong),
        ] {
            assert_eq!(
                PermissionLevel::from_str(input),
                Err(ParsePermissionLevelError::InvalidPermission(*expected))
            );
        }
    }

    #[test]
    fn test_display() {
        let p = PermissionLevel {
            actor: AccountName::new(n!(hello)),
            permission: PermissionName::new(n!(world)),
        };
        assert_eq!("hello@world", p.to_string());
    }
}