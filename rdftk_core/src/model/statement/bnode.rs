use crate::error::invalid_from_str;
use crate::model::qname::{is_xml_name, QName};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::rc::Rc;
use std::str::FromStr;
use unique_id::sequence::SequenceGenerator as IDGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A String wrapper for blank nodes.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlankNode(String);

///
/// A reference counted wrapper around a [`ObjectNode`] instance.
///
pub type BlankNodeRef = Rc<BlankNode>;

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

///
/// The reserved namespace value used to identify a serialized blank node.
///
pub const BLANK_NODE_NAMESPACE: &str = "_";

///
/// The reserved prefix value used to identify a serialized blank node.
///
pub const BLANK_NODE_PREFIX: &str = "_:";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl AsRef<str> for BlankNode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for BlankNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for BlankNode {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(invalid_from_str(s, "BlankNode"))
        }
    }
}

impl From<BlankNode> for String {
    fn from(v: BlankNode) -> Self {
        v.0
    }
}

impl BlankNode {
    ///
    /// Construct a new blank node with a generated identifier.
    ///
    pub fn generate() -> Self {
        Self(format!("B{}", IDGenerator.next_id()))
    }

    ///
    /// Returns `true` if the string is a valid blank node identifier, else
    /// `false`. Note that this function will accept simple names, or those
    /// with the reserved prefix `"_:"`.
    ///
    pub fn is_valid(s: &str) -> bool {
        is_xml_name(if let Some(s) = s.strip_prefix(BLANK_NODE_PREFIX) {
            s
        } else {
            s
        })
    }

    ///
    /// Return a qualified version of the blank node, i.e. with the reserved
    /// namespace value `"_"`.
    ///
    pub fn to_qname(&self) -> QName {
        QName::new_unchecked(Some(BLANK_NODE_NAMESPACE), &self.0)
    }
}
