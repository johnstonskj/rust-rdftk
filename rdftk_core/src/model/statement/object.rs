use crate::model::literal::Literal;
use crate::model::statement::{BlankNode, Statement};
use rdftk_iri::Iri;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use super::BLANK_NODE_NAMESPACE;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectNode<L, T>
where
    L: Literal,
    T: Statement<Literal = L>,
{
    Blank(BlankNode),
    Resource(Iri),
    Literal(L),
    Statement(Arc<T>),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<L: Literal, T: Statement<Literal = L>> From<BlankNode> for ObjectNode<L, T> {
    fn from(v: BlankNode) -> Self {
        Self::Blank(v)
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<&BlankNode> for ObjectNode<L, T> {
    fn from(v: &BlankNode) -> Self {
        Self::Blank(v.clone())
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<Iri> for ObjectNode<L, T> {
    fn from(v: Iri) -> Self {
        Self::Resource(v)
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<&Iri> for ObjectNode<L, T> {
    fn from(v: &Iri) -> Self {
        Self::Resource(v.clone())
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<L> for ObjectNode<L, T> {
    fn from(v: L) -> Self {
        Self::Literal(v)
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<Arc<T>> for ObjectNode<L, T> {
    fn from(v: Arc<T>) -> Self {
        Self::Statement(v)
    }
}

// ------------------------------------------------------------------------------------------------

impl<L: Literal + Display, T: Statement<Literal = L> + Display> Display for ObjectNode<L, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank(node) => write!(f, "{}:{}", BLANK_NODE_NAMESPACE, node),
            Self::Resource(iri) => write!(f, "<{}>", iri),
            Self::Literal(lit) => write!(f, "{}", lit),
            Self::Statement(st) => write!(f, "<< {} >>", st),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<L: Literal, T: Statement<Literal = L>> PartialEq<BlankNode> for ObjectNode<L, T> {
    fn eq(&self, other: &BlankNode) -> bool {
        match self {
            Self::Blank(value) => value == other,
            _ => false,
        }
    }
}

impl<L: Literal, T: Statement<Literal = L>> PartialEq<Iri> for ObjectNode<L, T> {
    fn eq(&self, other: &Iri) -> bool {
        match self {
            Self::Resource(value) => value == other,
            _ => false,
        }
    }
}

impl<L: Literal + PartialEq, T: Statement<Literal = L>> PartialEq<L> for ObjectNode<L, T> {
    fn eq(&self, other: &L) -> bool {
        match self {
            Self::Literal(value) => value == other,
            _ => false,
        }
    }
}

//
// This results in an error due to duplication with impl above, but not sure why.
//
//impl<L: Literal, T: Statement<Literal = L> + PartialEq> PartialEq<T> for ObjectNode<L, T> {
//    fn eq(&self, other: &T) -> bool {
//        match self {
//            Self::Statement(value) => <Arc<T> as Borrow<T>>::borrow(value) == other.borrow(),
//            _ => false,
//        }
//    }
//}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl<L: Literal, T: Statement<Literal = L>> ObjectNode<L, T> {
    pub fn is_blank(&self) -> bool {
        matches!(self, Self::Blank(_))
    }

    pub fn as_blank(&self) -> Option<&BlankNode> {
        match &self {
            Self::Blank(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_resource(&self) -> bool {
        matches!(self, Self::Resource(_))
    }

    pub fn as_resource(&self) -> Option<&Iri> {
        match &self {
            Self::Resource(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(_))
    }

    pub fn as_literal(&self) -> Option<&L> {
        match &self {
            Self::Literal(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_statement(&self) -> bool {
        matches!(self, Self::Statement(_))
    }

    pub fn as_statement(&self) -> Option<Arc<T>> {
        match &self {
            Self::Statement(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn provider_id(&self) -> Option<&'static str> {
        match self {
            Self::Literal(v) => Some(v.provider_id()),
            Self::Statement(v) => Some(v.provider_id()),
            _ => None,
        }
    }
}
