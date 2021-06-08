use crate::model::literal::LiteralRef;
use crate::model::statement::{ObjectNode, StatementRef};
use crate::model::Provided;
use rdftk_iri::IRIRef;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `ObjectNode` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleObjectNode {
    pub(crate) inner: Object,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub(crate) enum Object {
    BNode(String),
    IRI(IRIRef),
    Literal(LiteralRef),
    Star(StatementRef),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleObjectNode {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl ObjectNode for SimpleObjectNode {
    fn is_blank(&self) -> bool {
        matches!(self.inner, Object::BNode(_))
    }

    fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Object::BNode(s) => Some(s),
            _ => None,
        }
    }

    fn is_iri(&self) -> bool {
        matches!(self.inner, Object::IRI(_))
    }

    fn as_iri(&self) -> Option<&IRIRef> {
        match &self.inner {
            Object::IRI(u) => Some(u),
            _ => None,
        }
    }

    fn is_literal(&self) -> bool {
        matches!(self.inner, Object::Literal(_))
    }

    fn as_literal(&self) -> Option<&LiteralRef> {
        match &self.inner {
            Object::Literal(l) => Some(l),
            _ => None,
        }
    }

    fn is_statement(&self) -> bool {
        matches!(self.inner, Object::Star(_))
    }

    fn as_statement(&self) -> Option<&StatementRef> {
        match &self.inner {
            Object::Star(st) => Some(st),
            _ => None,
        }
    }
}
