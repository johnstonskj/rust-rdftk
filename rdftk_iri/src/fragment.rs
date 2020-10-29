#![allow(clippy::module_name_repetitions)]

use crate::error::{Error as IriError, ErrorKind};
use crate::Normalize;
use crate::{parse, ValidateStr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The fragment component of an `IRI` contains a fragment identifier providing direction to a
/// secondary resource, such as a section heading in an article identified by the remainder of the
/// URI. When the primary resource is an HTML document, the fragment is often an id attribute of a
/// specific element, and web browsers will scroll this element into view.
///
/// # Example
///
/// ```rust
/// use rdftk_iri::Fragment;
/// use std::str::FromStr;
///
/// let heading = Fragment::from_str("heading-one").unwrap();
/// println!("'{}'", heading); // prints '#heading-one'
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fragment(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Fragment {
    fn default() -> Self {
        Self(String::default())
    }
}

impl Display for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl FromStr for Fragment {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ErrorKind::ParseFragmentError(s.to_string()).into())
        }
    }
}

impl ValidateStr for Fragment {
    fn is_valid(s: &str) -> bool {
        parse::is_ifragment(s)
    }
}

impl Normalize for Fragment {}

impl Fragment {
    /// Return `true` if the fragment is the empty string `""` (which is a legal value), else `false`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the current value of this fragment as a String.
    pub fn value(&self) -> &String {
        &self.0
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(Fragment::is_valid(""));
        assert!(Fragment::is_valid("aaa"));
        assert!(Fragment::is_valid("aaa123"));
        assert!(Fragment::is_valid("aaa-123"));
        assert!(Fragment::is_valid("aaa_123"));
        assert!(Fragment::is_valid("aaa(123)"));
    }

    #[test]
    fn test_is_not_valid() {
        assert!(!Fragment::is_valid(" "));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Fragment::from_str("").unwrap(), Fragment("".to_string()));
        assert_eq!(
            Fragment::from_str("aaa").unwrap(),
            Fragment("aaa".to_string())
        );
        assert_eq!(
            Fragment::from_str("aaa123").unwrap(),
            Fragment("aaa123".to_string())
        );
        assert_eq!(
            Fragment::from_str("aaa-123").unwrap(),
            Fragment("aaa-123".to_string())
        );
        assert_eq!(
            Fragment::from_str("aaa_123").unwrap(),
            Fragment("aaa_123".to_string())
        );
        assert_eq!(
            Fragment::from_str("aaa(123)").unwrap(),
            Fragment("aaa(123)".to_string())
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(Fragment::from_str("").unwrap().to_string(), "#".to_string());
        assert_eq!(
            Fragment::from_str("aaa").unwrap().to_string(),
            "#aaa".to_string()
        );
        assert_eq!(
            Fragment::from_str("aaa123").unwrap().to_string(),
            "#aaa123".to_string()
        );
        assert_eq!(
            Fragment::from_str("aaa-123").unwrap().to_string(),
            "#aaa-123".to_string()
        );
        assert_eq!(
            Fragment::from_str("aaa_123").unwrap().to_string(),
            "#aaa_123".to_string()
        );
        assert_eq!(
            Fragment::from_str("aaa(123)").unwrap().to_string(),
            "#aaa(123)".to_string()
        );
    }
}
