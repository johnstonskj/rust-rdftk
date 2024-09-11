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
pub struct BlankNode(Name);

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

impl AsRef<Name> for BlankNode {
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for BlankNode {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
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
        Ok(Self(Name::from_str(s)?))
    }
}

impl From<BlankNode> for Name {
    fn from(v: BlankNode) -> Self {
        v.0
    }
}

impl From<BlankNode> for String {
    fn from(v: BlankNode) -> Self {
        v.0.into()
    }
}

impl From<&BlankNode> for Name {
    fn from(v: &BlankNode) -> Self {
        v.0.clone()
    }
}

impl From<&BlankNode> for String {
    fn from(v: &BlankNode) -> Self {
        (&v.0).into()
    }
}

impl From<Name> for BlankNode {
    fn from(value: Name) -> Self {
        Self(value)
    }
}

impl From<&Name> for BlankNode {
    fn from(value: &Name) -> Self {
        Self(value.clone())
    }
}

impl BlankNode {
    ///
    /// Construct a new blank node with a generated identifier.
    ///
    pub fn generate() -> Self {
        Self(Name::new_unchecked(format!("B{}", IDGenerator.next_id())))
    }

    ///
    /// Returns `true` if the string is a valid blank node identifier, else
    /// `false`. Note that this function will accept simple names, or those
    /// with the reserved prefix `"_:"`.
    ///
    pub fn is_valid(s: &str) -> bool {
        Name::is_valid_str(
            if let Some(s) = s.strip_prefix(BLANK_NODE_PREFIX) {
                s
            } else {
                s
            },
            Default::default(),
        )
    }

    ///
    /// Return a qualified version of the blank node, i.e. with the reserved
    /// namespace value `"_"`.
    ///
    pub fn to_qname(&self) -> QName {
        QName::new_blank(self.0.clone()).unwrap()
    }
}
