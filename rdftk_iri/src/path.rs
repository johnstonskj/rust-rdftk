/*!
Provides the `Path` component of an `IRI`.

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
    type Err = IriError;

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
    // RFC-3986§5.4
    fn normalize(self) -> IriResult<Self> {
        let mut segments = self.segments();
        let mut index: usize = 0;
        while index < segments.len() {
            let segment = segments.get(index).unwrap();
            if segment.is_empty() && index != 0 && index != segments.len() - 1 {
                segments.remove(index);
            } else if segment == "." {
                segments.remove(index);
            } else if segment == ".." {
                segments.remove(index);
                if index > 0 {
                    index -= 1;
                    segments.remove(index);
                }
            } else {
                index += 1;
            }
        }
        Ok(Self {
            inner: segments.join("/"),
        })
    }
}

impl Path {
    pub fn resolve(&self, path: &Path) -> IriResult<Self> {
        let new_path = if path.is_empty() {
            self.clone()
        } else if path.is_absolute() {
            path.clone()
        } else if self.inner.ends_with('/') {
            let mut new = self.clone();
            new.push(&path.inner)?;
            new
        } else {
            let mut new = self.clone();
            let _ = new.pop_slug();
            new.push(&path.inner)?;
            new
        }
        .normalize()?;
        Ok(new_path)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn is_absolute(&self) -> bool {
        self.inner.starts_with('/')
    }

    pub fn is_normalized(&self) -> bool {
        self.inner
            .split('/')
            .all(|segment| segment != "." && segment != "..")
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

    pub fn push(&mut self, segment: &str) -> IriResult<()> {
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

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_no_relatives() {
        let base = Path::from_str("/b/c/d;p").unwrap();
        assert_eq!(
            base.resolve(&Path::from_str("g").unwrap()).unwrap(),
            Path::from_str("/b/c/g").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("g/").unwrap()).unwrap(),
            Path::from_str("/b/c/g/").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("/g").unwrap()).unwrap(),
            Path::from_str("/g").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("").unwrap()).unwrap(),
            Path::from_str("/b/c/d;p").unwrap()
        );
    }

    #[test]
    fn test_resolve_relatives() {
        let base = Path::from_str("/b/c/d;p").unwrap();
        assert_eq!(
            base.resolve(&Path::from_str("./g").unwrap()).unwrap(),
            Path::from_str("/b/c/g").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str(".").unwrap()).unwrap(),
            Path::from_str("/b/c").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("./").unwrap()).unwrap(),
            Path::from_str("/b/c/").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("./././").unwrap()).unwrap(),
            Path::from_str("/b/c/").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("..").unwrap()).unwrap(),
            Path::from_str("/b").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("../").unwrap()).unwrap(),
            Path::from_str("/b/").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("../g").unwrap()).unwrap(),
            Path::from_str("/b/g").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("../..").unwrap()).unwrap(),
            Path::from_str("").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("../../").unwrap()).unwrap(),
            Path::from_str("/").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("../../../../").unwrap())
                .unwrap(),
            Path::from_str("").unwrap()
        );
        assert_eq!(
            base.resolve(&Path::from_str("../../g").unwrap()).unwrap(),
            Path::from_str("/g").unwrap()
        );
    }
}
