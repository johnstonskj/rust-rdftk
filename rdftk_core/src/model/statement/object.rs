use crate::model::features::{Featured, FEATURE_RDF_STAR};
use crate::model::literal::Literal;
use crate::model::statement::{BlankNode, Statement, SubjectNode, BLANK_NODE_NAMESPACE};
use rdftk_iri::{Iri, Name};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectNode {
    Blank(BlankNode),
    Resource(Iri),
    Literal(Literal),
    Statement(Arc<Statement>),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<&ObjectNode> for ObjectNode {
    fn from(v: &ObjectNode) -> Self {
        v.clone()
    }
}

impl From<BlankNode> for ObjectNode {
    fn from(v: BlankNode) -> Self {
        Self::Blank(v)
    }
}

impl From<&BlankNode> for ObjectNode {
    fn from(v: &BlankNode) -> Self {
        Self::Blank(v.clone())
    }
}

impl From<Name> for ObjectNode {
    fn from(v: Name) -> Self {
        Self::Blank(v.into())
    }
}

impl From<&Name> for ObjectNode {
    fn from(v: &Name) -> Self {
        Self::Blank(v.clone().into())
    }
}

impl From<Iri> for ObjectNode {
    fn from(v: Iri) -> Self {
        Self::Resource(v)
    }
}

impl From<&Iri> for ObjectNode {
    fn from(v: &Iri) -> Self {
        Self::Resource(v.clone())
    }
}

impl From<Literal> for ObjectNode {
    fn from(v: Literal) -> Self {
        Self::Literal(v)
    }
}

impl From<&Literal> for ObjectNode {
    fn from(v: &Literal) -> Self {
        Self::Literal(v.clone())
    }
}

impl From<Statement> for ObjectNode {
    fn from(v: Statement) -> Self {
        Self::Statement(Arc::new(v))
    }
}

impl From<&Statement> for ObjectNode {
    fn from(v: &Statement) -> Self {
        Self::Statement(Arc::new(v.clone()))
    }
}

impl From<Arc<Statement>> for ObjectNode {
    fn from(v: Arc<Statement>) -> Self {
        Self::Statement(v)
    }
}

impl From<&Arc<Statement>> for ObjectNode {
    fn from(v: &Arc<Statement>) -> Self {
        Self::Statement(v.clone())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ObjectNode {
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

impl PartialEq<BlankNode> for ObjectNode {
    fn eq(&self, other: &BlankNode) -> bool {
        match self {
            Self::Blank(value) => value == other,
            _ => false,
        }
    }
}

impl PartialEq<Iri> for ObjectNode {
    fn eq(&self, other: &Iri) -> bool {
        match self {
            Self::Resource(value) => value == other,
            _ => false,
        }
    }
}

impl PartialEq<Literal> for ObjectNode {
    fn eq(&self, other: &Literal) -> bool {
        match self {
            Self::Literal(value) => value == other,
            _ => false,
        }
    }
}

impl PartialEq<Statement> for ObjectNode {
    fn eq(&self, other: &Statement) -> bool {
        match self {
            Self::Statement(value) => {
                <Arc<Statement> as Borrow<Statement>>::borrow(value) == other.borrow()
            }
            _ => false,
        }
    }
}

impl PartialEq<Arc<Statement>> for ObjectNode {
    fn eq(&self, other: &Arc<Statement>) -> bool {
        match self {
            Self::Statement(value) => value == other,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl PartialOrd for ObjectNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObjectNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Blank(lhs), Self::Blank(rhs)) => lhs.cmp(rhs),
            (Self::Blank(_), Self::Resource(_)) => Ordering::Less,
            (Self::Blank(_), Self::Literal(_)) => Ordering::Less,
            (Self::Blank(_), Self::Statement(_)) => Ordering::Less,
            (Self::Resource(_), Self::Blank(_)) => Ordering::Greater,
            (Self::Resource(lhs), Self::Resource(rhs)) => lhs.cmp(rhs),
            (Self::Resource(_), Self::Literal(_)) => Ordering::Less,
            (Self::Resource(_), Self::Statement(_)) => Ordering::Less,
            (Self::Literal(_), Self::Blank(_)) => Ordering::Greater,
            (Self::Literal(_), Self::Resource(_)) => Ordering::Greater,
            (Self::Literal(lhs), Self::Literal(rhs)) => lhs.cmp(rhs),
            (Self::Literal(_), Self::Statement(_)) => Ordering::Less,
            (Self::Statement(_), Self::Blank(_)) => Ordering::Greater,
            (Self::Statement(_), Self::Resource(_)) => Ordering::Greater,
            (Self::Statement(_), Self::Literal(_)) => Ordering::Greater,
            (Self::Statement(lhs), Self::Statement(rhs)) => lhs.cmp(rhs),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for ObjectNode {
    fn supports_feature(&self, feature: &Iri) -> bool {
        *feature == *FEATURE_RDF_STAR
    }
}

// ------------------------------------------------------------------------------------------------

impl ObjectNode {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------
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

    pub fn as_literal(&self) -> Option<&Literal> {
        match &self {
            Self::Literal(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_statement(&self) -> bool {
        matches!(self, Self::Statement(_))
    }

    pub fn as_statement(&self) -> Option<Arc<Statement>> {
        match &self {
            Self::Statement(v) => Some(v.clone()),
            _ => None,
        }
    }
    // --------------------------------------------------------------------------------------------
    // Conversions
    // --------------------------------------------------------------------------------------------
    pub fn to_subject(&self) -> Option<SubjectNode> {
        match self {
            ObjectNode::Blank(v) => Some(v.clone().into()),
            ObjectNode::Resource(v) => Some(v.clone().into()),
            ObjectNode::Statement(v) => Some(v.clone().into()),
            ObjectNode::Literal(_) => None,
        }
    }
}
