/*!
Provides the `Path` component of an `IRI`.

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
pub struct Path {
    inner: String,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Path {
    fn default() -> Self {
        Self {
            inner: String::new(),
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FromStr for Path {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if parse::is_path(s) {
            Ok(Self {
                inner: s.to_string(),
            })
        } else {
            Err(ErrorKind::InvalidChar(Component::Path).into())
        }
    }
}

impl Normalize for Path {
    fn normalize(self) -> UriResult<Self> {
        unimplemented!()
    }
}

impl Path {
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn value(&self) -> &String {
        &self.inner
    }

    pub fn clear(&mut self) {
        self.inner = String::new();
    }

    pub fn segments(&self) -> Vec<String> {
        self.inner.split('/').map(|s| s.to_string()).collect()
    }

    pub fn push(&mut self, segment: &str) -> UriResult<()> {
        if parse::is_path(segment) {
            if self.inner.ends_with('/') {
                self.inner = format!("{}{}", self.inner, segment);
            } else {
                self.inner = format!("{}/{}", self.inner, segment);
            }
            Ok(())
        } else {
            Err(ErrorKind::InvalidChar(Component::Path).into())
        }
    }

    pub fn pop(&mut self) -> Option<String> {
        let mut segments = self.segments();
        let last = segments.pop();
        self.inner = segments.join("/");
        last
    }

    pub fn has_slug(&self) -> bool {
        !self.inner.is_empty() && !self.inner.ends_with('/')
    }

    pub fn slug(&mut self) -> Option<String> {
        if self.has_slug() {
            let segments = self.segments();
            segments.last().cloned()
        } else {
            None
        }
    }

    pub fn pop_slug(&mut self) -> Option<String> {
        let mut segments = self.segments();
        let last = segments.pop();
        self.inner = segments.join("/");
        if !self.inner.is_empty() {
            self.inner = format!("{}/", self.inner);
        }
        last
    }

    pub fn merge(&mut self, _path: &Path) -> UriResult<()> {
        unimplemented!()
    }

    pub fn with_merged(&self, _path: &Path) -> UriResult<Path> {
        unimplemented!()
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
