/*!
Provides the `Query` component of an `IRI`.

# Example

TBD

*/

#![allow(clippy::module_name_repetitions)]

use crate::error::{Component, Error as UriError, ErrorKind, Result as UriResult};
use crate::parse;
use crate::Normalize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Query {
    inner: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QueryPart {
    key: String,
    value: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.inner)
    }
}

impl FromStr for Query {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if parse::is_iquery(s) {
            Ok(Self {
                inner: s.to_string(),
            })
        } else {
            Err(ErrorKind::InvalidChar(Component::Query).into())
        }
    }
}

impl Normalize for Query {
    fn normalize(self) -> UriResult<Self> {
        unimplemented!()
    }
}

impl Query {
    pub fn new(part: &QueryPart) -> Self {
        Self {
            inner: match &part.value {
                None => part.key.clone(),
                Some(value) => format!("{}={}", part.key, value),
            },
        }
    }

    pub fn push(&mut self, part: &QueryPart) {
        self.inner = match &part.value {
            None => part.key.clone(),
            Some(value) => format!("{}={}", part.key, value),
        }
    }

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

impl QueryPart {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            value: None,
        }
    }
    pub fn with_value(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: Some(value.to_string()),
        }
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
