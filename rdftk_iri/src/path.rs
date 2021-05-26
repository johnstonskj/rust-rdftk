/*!
A path is always defined for a URI, though the defined path may be empty (zero length). A
segment may also be empty, resulting in two consecutive slashes (//) in the path component. A
path component may resemble or map exactly to a file system path, but does not always imply a
relation to one. If an authority component is present, then the path component must either be
empty or begin with a slash (/). If an authority component is absent, then the path cannot
begin with an empty segment, that is with two slashes (//), as the following characters would
be interpreted as an authority component. The final segment of the path may be referred to as
a 'slug'.

# Example

TBD

*/

#![allow(clippy::module_name_repetitions)]

use crate::error::{Component, Error as IriError, ErrorKind, Result as IriResult};
use crate::Normalize;
use crate::{parse, ValidateStr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The path is a component of the _"generic URI"_, per[RFC 3296](https://tools.ietf.org/html/rfc2396)
/// §3:
///
/// > URI that are hierarchical in nature use the slash "/" character for
/// > separating hierarchical components.  For some file systems, a "/"
/// > character (used to denote the hierarchical structure of a URI) is the
/// > delimiter used to construct a file name hierarchy, and thus the URI
/// > path will look similar to a file pathname.  This does NOT imply that
/// > the resource is a file or that the URI maps to an actual filesystem
/// > pathname.
///
/// > URI that do not make use of the slash "/" character for separating
/// > hierarchical components are considered opaque by the generic URI
/// > parser.
///
/// Specifically, any absolute URI, that is one having a specified scheme, whose path portion
/// **does not** start with a slash "/" character should be considered opaque.
///
///
/// # Example
///
/// ```rust
/// use rdftk_iri::Path;
/// use std::str::FromStr;
///
/// let path = Path::from_str("foo").unwrap();
/// println!("'{}'", path); // prints 'foo'
///
/// let path = Path::from_str("/foo/bar").unwrap();
/// println!("'{}'", path); // prints '/foo/bar'
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Path(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const PATH_SEP: &str = "/";
const DOT: &str = ".";
const DOT_DOT: &str = "..";
const WELL_KNOWN: &str = "/.well-known/";

impl Default for Path {
    fn default() -> Self {
        Self(String::new())
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Path {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ErrorKind::InvalidChar(Component::Path).into())
        }
    }
}

impl ValidateStr for Path {
    fn is_valid(s: &str) -> bool {
        parse::is_path(s)
    }
}

impl Normalize for Path {
    // SPEC: RFC-3986 §5.4
    fn normalize(self) -> IriResult<Self> {
        let mut segments = self.hierarchical_segments();
        let mut index: usize = 0;
        while index < segments.len() {
            let segment = segments.get(index).unwrap();
            if (segment.is_empty() && index != 0 && index != segments.len() - 1) || segment == DOT {
                let _ = segments.remove(index);
            } else if segment == DOT_DOT {
                let _ = segments.remove(index);
                if index > 0 {
                    index -= 1;
                    let _ = segments.remove(index);
                }
            } else {
                index += 1;
            }
        }
        Ok(Self(segments.join(PATH_SEP)))
    }
}

impl Path {
    ///
    /// The root of a path is the path separator character "/", this will return a new path
    /// consisting of only this character.
    ///
    pub fn root() -> Self {
        Self(PATH_SEP.to_string())
    }

    ///
    /// Constructs a new *well-known* path, i.e. it contains the prefix specified in
    /// [RFC-8615: Well-Known Uniform Resource Identifiers (URIs)](https://datatracker.ietf.org/doc/html/rfc8615).
    ///
    pub fn well_known() -> Self {
        Self(WELL_KNOWN.to_string())
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Return `true` if the path is the empty string `""` (which is a legal value), else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// Returns `true` if this path is an absolute path, else `false`.
    ///
    pub fn is_absolute(&self) -> bool {
        self.0.starts_with(PATH_SEP)
    }

    /// Returns the current value of the path as a String.
    pub fn value(&self) -> &String {
        &self.0
    }

    // --------------------------------------------------------------------------------------------

    /// self = base
    pub fn resolve(&self, relative_path: &Path) -> IriResult<Self> {
        let new_path = if relative_path.is_empty() {
            self.clone()
        } else if relative_path.is_absolute() {
            relative_path.clone()
        } else if self.0.ends_with(PATH_SEP) {
            let mut new = self.clone();
            new.push(&relative_path.0)?;
            new
        } else {
            let mut new = self.clone();
            let _ = new.pop_slug();
            new.push(&relative_path.0)?;
            new
        }
        .normalize()?;
        Ok(new_path)
    }

    ///
    /// Returns `true` if this path is fully normalized, else `false`.
    ///
    pub fn is_normalized(&self) -> bool {
        self.0
            .split(PATH_SEP)
            .all(|segment| segment != DOT && segment != DOT_DOT)
    }

    ///
    /// Returns true if this path starts with the well-known prefix defined in
    /// [RFC-8615: Well-Known Uniform Resource Identifiers (URIs)](https://datatracker.ietf.org/doc/html/rfc8615).
    ///
    pub fn is_well_known(&self) -> bool {
        self.0.starts_with(WELL_KNOWN)
    }
    // --------------------------------------------------------------------------------------------

    fn hierarchical_segments(&self) -> Vec<String> {
        self.0.split(PATH_SEP).map(|s| s.to_string()).collect()
    }

    /// Push a new segment onto the end of the path.
    pub fn push(&mut self, segment: &str) -> IriResult<()> {
        if parse::is_path(segment) {
            if self.0.ends_with(PATH_SEP) {
                self.0 = format!("{}{}", self.0, segment);
            } else {
                self.0 = format!("{}/{}", self.0, segment);
            }
            Ok(())
        } else {
            Err(ErrorKind::InvalidChar(Component::Path).into())
        }
    }

    /// Pop the last segment from the end of the path, if present.
    pub fn pop(&mut self) -> Option<String> {
        let mut segments = self.hierarchical_segments();
        let last = segments.pop();
        self.0 = segments.join(PATH_SEP);
        last
    }

    /// Returns `true` if this path ends in a _slug_, else `false`.
    pub fn has_slug(&self) -> bool {
        !self.0.is_empty() && !self.0.ends_with(PATH_SEP)
    }

    /// Return the slug from the end of the path, if present.
    pub fn slug(&mut self) -> Option<String> {
        if self.has_slug() {
            let segments = self.hierarchical_segments();
            segments.last().cloned()
        } else {
            None
        }
    }

    /// Pop the slug from the end of the path, if present.
    pub fn pop_slug(&mut self) -> Option<String> {
        let mut segments = self.hierarchical_segments();
        let last = segments.pop();
        self.0 = segments.join(PATH_SEP);
        if !self.0.is_empty() {
            self.0 = format!("{}{}", self.0, PATH_SEP);
        }
        last
    }
}
