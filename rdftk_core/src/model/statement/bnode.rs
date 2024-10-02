use rdftk_iri::{Name, NameParser, QName};
use std::fmt::{Display, Formatter};
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
        if BlankNode::is_valid_str(s) {
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
    pub fn is_valid_str<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        let s: &str = s.as_ref();
        NameParser::BlankNode.is_valid_str(if let Some(s) = s.strip_prefix(BLANK_NODE_PREFIX) {
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
