/*!
Provides the `Fragment` component of an `IRI`.

# Example

TBD

*/

#![allow(clippy::module_name_repetitions)]

use crate::error::{Component, Error as IriError, ErrorKind, Result as IriResult};
use crate::parse;
use crate::Normalize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fragment {
    inner: String,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Fragment {
    fn default() -> Self {
        Self {
            inner: "".to_string(),
        }
    }
}

impl Display for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.inner)
    }
}

impl FromStr for Fragment {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if parse::is_ifragment(s) {
            Ok(Self {
                inner: s.to_string(),
            })
        } else {
            Err(ErrorKind::InvalidChar(Component::Fragment).into())
        }
    }
}

impl Normalize for Fragment {
    fn normalize(self) -> IriResult<Self> {
        unimplemented!()
    }
}

impl Fragment {
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn value(&self) -> &String {
        &self.inner
    }

    pub fn clear(&mut self) {
        self.inner = String::new();
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
