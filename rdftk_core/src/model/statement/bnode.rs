use crate::model::qname::QName;
use rdftk_iri::Name;
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
/// A reference counted wrapper around a [`BlankNode`] instance.
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

impl Display for BlankNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for BlankNode {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if Self::is_valid_str(s) {
            Ok(Self(s.into()))
        } else {
            Err(crate::error::Error::InvalidBlankNodeName { name: s.into() })
        }
    }
}

impl From<BlankNode> for String {
    fn from(v: BlankNode) -> Self {
        v.0
    }
}

impl From<Name> for BlankNode {
    fn from(v: Name) -> Self {
        Self(v.into())
    }
}

impl From<&Name> for BlankNode {
    fn from(v: &Name) -> Self {
        Self(v.into())
    }
}

impl From<&BlankNode> for String {
    fn from(v: &BlankNode) -> Self {
        v.0.clone()
    }
}

impl AsRef<str> for BlankNode {
    fn as_ref(&self) -> &str {
        self.0.as_str()
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
    pub fn is_valid_str(s: &str) -> bool {
        is_bnode_name(if let Some(s) = s.strip_prefix(BLANK_NODE_PREFIX) {
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
        QName::new_blank(Name::new_unchecked(&self.0)).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn is_bnode_name_start_char(c: char) -> bool {
    c == ':'
        || c.is_ascii_digit()
        || c.is_ascii_uppercase()
        || c == '_'
        || c.is_ascii_lowercase()
        || ('\u{C0}'..='\u{D6}').contains(&c)
        || ('\u{D8}'..='\u{F6}').contains(&c)
        || ('\u{0F8}'..='\u{2FF}').contains(&c)
        || ('\u{370}'..='\u{37D}').contains(&c)
        || ('\u{037F}'..='\u{1FFF}').contains(&c)
        || ('\u{200C}'..='\u{200D}').contains(&c)
        || ('\u{2070}'..='\u{218F}').contains(&c)
        || ('\u{2C00}'..='\u{2FEF}').contains(&c)
        || ('\u{3001}'..='\u{D7FF}').contains(&c)
        || ('\u{F900}'..='\u{FDCF}').contains(&c)
        || ('\u{FDF0}'..='\u{FFFD}').contains(&c)
        || ('\u{10000}'..='\u{EFFFF}').contains(&c)
}

pub(crate) fn is_bnode_name_char(c: char) -> bool {
    is_bnode_name_start_char(c)
        || c == '-'
        || c == '.'
        || c == '\u{B7}'
        || ('\u{0300}'..='\u{036F}').contains(&c)
        || ('\u{203F}'..='\u{2040}').contains(&c)
}

pub(crate) fn is_bnode_name(s: &str) -> bool {
    !s.is_empty()
        && s.starts_with(is_bnode_name_start_char)
        && s[1..].chars().all(is_bnode_name_char)
}
