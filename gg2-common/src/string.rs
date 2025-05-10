use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::networking::error::{NetworkError as Error, Result};

macro_rules! string_type {
    ($name: ident, $length: literal, $length_type: ident) => {
        #[doc = concat!("A string with a max length of ", $length)]
        #[repr(transparent)]
        #[derive(Default, Clone, Hash, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct $name(String);

        impl $name {
            // Inline to avoid assert
            #[inline(always)]
            #[doc = concat!("Panics if length is greater than ", $length)]
            pub fn from_utf8(
                vec: Vec<u8>,
            ) -> std::result::Result<Self, std::string::FromUtf8Error> {
                let value = String::from_utf8(vec)?;
                let length = value.len();
                assert!(length <= $length_type::MAX as usize);
                Ok(Self(value))
            }

            pub fn len(&self) -> $length_type {
                self.0.len() as $length_type
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl TryFrom<String> for $name {
            type Error = Error;

            fn try_from(value: String) -> Result<Self> {
                match $length_type::try_from(value.len()) {
                    Ok(_) => Ok(Self(value)),
                    Err(error) => Err(Error::StringLength(error)),
                }
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                s.to_string().try_into()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl Deref for $name {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Display::fmt(&**self, f)
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Debug::fmt(&**self, f)
            }
        }
    };
}

string_type!(GGStringLong, 65535, u16);
string_type!(GGStringShort, 255, u8);
