use crate::model::literal::LiteralRef;
use crate::model::statement::{BlankNodeRef, ObjectNode, StatementRef, SubjectNode};
use crate::model::Provided;
use rdftk_iri::IriRef;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `ObjectNode` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleObjectNode(Object);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
enum Object {
    BNode(BlankNodeRef),
    Iri(IriRef),
    Literal(LiteralRef),
    Star(StatementRef),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<BlankNodeRef> for SimpleObjectNode {
    fn from(v: BlankNodeRef) -> Self {
        Self(Object::BNode(v))
    }
}

impl From<IriRef> for SimpleObjectNode {
    fn from(v: IriRef) -> Self {
        Self(Object::Iri(v))
    }
}

impl From<LiteralRef> for SimpleObjectNode {
    fn from(v: LiteralRef) -> Self {
        Self(Object::Literal(v))
    }
}

impl From<StatementRef> for SimpleObjectNode {
    fn from(v: StatementRef) -> Self {
        Self(Object::Star(v))
    }
}

impl Provided for SimpleObjectNode {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl SubjectNode for SimpleObjectNode {
    fn is_blank(&self) -> bool {
        matches!(self.0, Object::BNode(_))
    }

    fn as_blank(&self) -> Option<&BlankNodeRef> {
        match &self.0 {
            Object::BNode(s) => Some(s),
            _ => None,
        }
    }

    fn is_iri(&self) -> bool {
        matches!(self.0, Object::Iri(_))
    }

    fn as_iri(&self) -> Option<&IriRef> {
        match &self.0 {
            Object::Iri(u) => Some(u),
            _ => None,
        }
    }

    fn is_statement(&self) -> bool {
        matches!(self.0, Object::Star(_))
    }

    fn as_statement(&self) -> Option<&StatementRef> {
        match &self.0 {
            Object::Star(st) => Some(st),
            _ => None,
        }
    }
}

impl ObjectNode for SimpleObjectNode {
    fn is_literal(&self) -> bool {
        matches!(self.0, Object::Literal(_))
    }

    fn as_literal(&self) -> Option<&LiteralRef> {
        match &self.0 {
            Object::Literal(l) => Some(l),
            _ => None,
        }
    }
}
