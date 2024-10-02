use super::BLANK_NODE_NAMESPACE;
use crate::model::literal::Literal;
use crate::model::statement::{BlankNode, Statement};
use rdftk_iri::Iri;
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SubjectNode<L, T>
where
    L: Literal,
    T: Statement<Literal = L> + Clone,
{
    Blank(BlankNode),
    Resource(Iri),
    Statement(Arc<T>),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<L: Literal, T: Statement<Literal = L>> From<BlankNode> for SubjectNode<L, T> {
    fn from(v: BlankNode) -> Self {
        Self::Blank(v)
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<&BlankNode> for SubjectNode<L, T> {
    fn from(v: &BlankNode) -> Self {
        Self::Blank(v.clone())
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<Iri> for SubjectNode<L, T> {
    fn from(v: Iri) -> Self {
        Self::Resource(v)
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<&Iri> for SubjectNode<L, T> {
    fn from(v: &Iri) -> Self {
        Self::Resource(v.clone())
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<Arc<T>> for SubjectNode<L, T> {
    fn from(v: Arc<T>) -> Self {
        Self::Statement(v)
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<T> for SubjectNode<L, T> {
    fn from(v: T) -> Self {
        Self::Statement(Arc::new(v))
    }
}

// ------------------------------------------------------------------------------------------------

impl<L: Literal, T: Statement<Literal = L> + Display> Display for SubjectNode<L, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank(node) => write!(f, "{}:{}", BLANK_NODE_NAMESPACE, node),
            Self::Resource(iri) => write!(f, "<{}>", iri),
            Self::Statement(st) => write!(f, "<< {} >>", st),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<L: Literal, T: Statement<Literal = L>> PartialEq<BlankNode> for SubjectNode<L, T> {
    fn eq(&self, other: &BlankNode) -> bool {
        match self {
            Self::Blank(value) => value == other,
            _ => false,
        }
    }
}

impl<L: Literal, T: Statement<Literal = L>> PartialEq<Iri> for SubjectNode<L, T> {
    fn eq(&self, other: &Iri) -> bool {
        match self {
            Self::Resource(value) => value == other,
            _ => false,
        }
    }
}

impl<L: Literal, T: Statement<Literal = L> + PartialEq> PartialEq<T> for SubjectNode<L, T> {
    fn eq(&self, other: &T) -> bool {
        match self {
            Self::Statement(value) => <Arc<T> as Borrow<T>>::borrow(value) == other.borrow(),
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl<L: Literal, T: Statement<Literal = L>> SubjectNode<L, T> {
    pub fn is_blank(&self) -> bool {
        matches!(self, Self::Blank(_))
    }

    pub fn as_blank(&self) -> Option<&BlankNode> {
        match &self {
            Self::Blank(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_resource(&self) -> bool {
        matches!(self, Self::Resource(_))
    }

    pub fn as_resource(&self) -> Option<&Iri> {
        match &self {
            Self::Resource(u) => Some(u),
            _ => None,
        }
    }

    pub fn is_statement(&self) -> bool {
        matches!(self, Self::Statement(_))
    }

    pub fn as_statement(&self) -> Option<Arc<T>> {
        match &self {
            Self::Statement(st) => Some(st.clone()),
            _ => None,
        }
    }

    pub fn provider_id(&self) -> Option<&'static str> {
        match self {
            Self::Statement(v) => Some(v.provider_id()),
            _ => None,
        }
    }
}
