/*!
The query component of an `IRI` is preceded by a question mark (?) and contains a query string of
non-hierarchical data. Its syntax is not well defined, but by convention is most often a sequence
of attribute–value pairs separated bya delimiter (&).

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
/// This type holds the query component of the IRI. While it is common in URLs to see queries of
/// the form `key=value&key=value...` this is not part of the specification which explicitly makes
/// the format of queries opaque:
///
/// > The query component is a string of information to be interpreted by the resource.
///
/// # Example
///
/// ```rust
/// use rdftk_iri::Query;
/// use std::str::FromStr;
///
/// let query = Query::from_str("page1").unwrap();
/// println!("'{}'", query); // prints '?page1'
///
/// let query = Query::from_str("page=1&size=20").unwrap();
/// println!("'{}'", query); // prints '?page=1&size=20'
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Query(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Query {
    fn default() -> Self {
        Self(String::new())
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.0)
    }
}

impl FromStr for Query {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ErrorKind::InvalidChar(Component::Query).into())
        }
    }
}

impl ValidateStr for Query {
    fn is_valid(s: &str) -> bool {
        parse::is_iquery(s)
    }
}

impl Normalize for Query {
    fn normalize(self) -> IriResult<Self> {
        unimplemented!()
    }
}

impl Query {
    /// Return `true` if the path is the empty string `""` (which is a legal value), else `false`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the current value of this fragment as a String.
    pub fn value(&self) -> &String {
        &self.0
    }
}
