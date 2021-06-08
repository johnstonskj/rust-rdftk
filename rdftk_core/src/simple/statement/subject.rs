use crate::model::statement::{StatementRef, SubjectNode};
use crate::model::Provided;
use rdftk_iri::IRIRef;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `SubjectNode` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleSubjectNode {
    pub(crate) inner: Subject,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub(crate) enum Subject {
    BNode(String),
    IRI(IRIRef),
    Star(StatementRef),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleSubjectNode {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl SubjectNode for SimpleSubjectNode {
    fn is_blank(&self) -> bool {
        matches!(self.inner, Subject::BNode(_))
    }

    fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Subject::BNode(s) => Some(s),
            _ => None,
        }
    }

    fn is_iri(&self) -> bool {
        matches!(self.inner, Subject::IRI(_))
    }

    fn as_iri(&self) -> Option<&IRIRef> {
        match &self.inner {
            Subject::IRI(u) => Some(u),
            _ => None,
        }
    }

    fn is_statement(&self) -> bool {
        matches!(self.inner, Subject::Star(_))
    }

    fn as_statement(&self) -> Option<&StatementRef> {
        match &self.inner {
            Subject::Star(st) => Some(st),
            _ => None,
        }
    }
}
