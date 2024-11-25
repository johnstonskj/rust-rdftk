use crate::model::features::{Featured, FEATURE_RDF_STAR};
use crate::model::statement::{BlankNode, ObjectNode, Statement, BLANK_NODE_NAMESPACE};
use rdftk_iri::{Iri, Name};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SubjectNode {
    Blank(BlankNode),
    Resource(Iri),
    // TODO: add version of ObjectNode::Collection
    Statement(Arc<Statement>),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<&SubjectNode> for SubjectNode {
    fn from(v: &SubjectNode) -> Self {
        v.clone()
    }
}

impl From<BlankNode> for SubjectNode {
    fn from(v: BlankNode) -> Self {
        Self::Blank(v)
    }
}

impl From<&BlankNode> for SubjectNode {
    fn from(v: &BlankNode) -> Self {
        Self::Blank(v.clone())
    }
}

impl From<Name> for SubjectNode {
    fn from(v: Name) -> Self {
        Self::Blank(v.into())
    }
}

impl From<&Name> for SubjectNode {
    fn from(v: &Name) -> Self {
        Self::Blank(v.clone().into())
    }
}

impl From<Iri> for SubjectNode {
    fn from(v: Iri) -> Self {
        Self::Resource(v)
    }
}

impl From<&Iri> for SubjectNode {
    fn from(v: &Iri) -> Self {
        Self::Resource(v.clone())
    }
}

impl From<Statement> for SubjectNode {
    fn from(v: Statement) -> Self {
        Self::Statement(Arc::new(v))
    }
}

impl From<&Statement> for SubjectNode {
    fn from(v: &Statement) -> Self {
        Self::Statement(Arc::new(v.clone()))
    }
}

impl From<Arc<Statement>> for SubjectNode {
    fn from(v: Arc<Statement>) -> Self {
        Self::Statement(v)
    }
}

impl From<&Arc<Statement>> for SubjectNode {
    fn from(v: &Arc<Statement>) -> Self {
        Self::Statement(v.clone())
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SubjectNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank(node) => write!(f, "{}:{}", BLANK_NODE_NAMESPACE, node),
            Self::Resource(iri) => write!(f, "<{}>", iri),
            Self::Statement(st) => write!(f, "<< {} >>", st),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl PartialEq<BlankNode> for SubjectNode {
    fn eq(&self, other: &BlankNode) -> bool {
        match self {
            Self::Blank(value) => value == other,
            _ => false,
        }
    }
}

impl PartialEq<Iri> for SubjectNode {
    fn eq(&self, other: &Iri) -> bool {
        match self {
            Self::Resource(value) => value == other,
            _ => false,
        }
    }
}

impl PartialEq<Statement> for SubjectNode {
    fn eq(&self, other: &Statement) -> bool {
        match self {
            Self::Statement(value) => {
                <Arc<Statement> as Borrow<Statement>>::borrow(value) == other.borrow()
            }
            _ => false,
        }
    }
}

impl PartialEq<Arc<Statement>> for SubjectNode {
    fn eq(&self, other: &Arc<Statement>) -> bool {
        match self {
            Self::Statement(value) => value == other,
            _ => false,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl PartialOrd for SubjectNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SubjectNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Blank(lhs), Self::Blank(rhs)) => lhs.cmp(rhs),
            (Self::Blank(_), Self::Resource(_)) => Ordering::Less,
            (Self::Blank(_), Self::Statement(_)) => Ordering::Less,
            (Self::Resource(_), Self::Blank(_)) => Ordering::Greater,
            (Self::Resource(lhs), Self::Resource(rhs)) => lhs.cmp(rhs),
            (Self::Resource(_), Self::Statement(_)) => Ordering::Less,
            (Self::Statement(_), Self::Blank(_)) => Ordering::Greater,
            (Self::Statement(_), Self::Resource(_)) => Ordering::Greater,
            (Self::Statement(lhs), Self::Statement(rhs)) => lhs.cmp(rhs),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for SubjectNode {
    fn supports_feature(&self, feature: &Iri) -> bool {
        *feature == *FEATURE_RDF_STAR
    }
}

// ------------------------------------------------------------------------------------------------

impl SubjectNode {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------
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

    pub fn as_statement(&self) -> Option<Arc<Statement>> {
        match &self {
            Self::Statement(st) => Some(st.clone()),
            _ => None,
        }
    }
    // --------------------------------------------------------------------------------------------
    // Conversion
    // --------------------------------------------------------------------------------------------

    pub fn to_object(&self) -> ObjectNode {
        match self {
            SubjectNode::Blank(v) => v.clone().into(),
            SubjectNode::Resource(v) => v.clone().into(),
            SubjectNode::Statement(v) => v.clone().into(),
        }
    }
}
