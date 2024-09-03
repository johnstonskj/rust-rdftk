use crate::model::statement::{BlankNode, StatementRef, SubjectNode};
use crate::model::Provided;
use rdftk_iri::IriRef;

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
    BNode(BlankNode),
    Iri(IriRef),
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

    fn as_blank(&self) -> Option<&BlankNode> {
        match &self.inner {
            Subject::BNode(s) => Some(s),
            _ => None,
        }
    }

    fn is_iri(&self) -> bool {
        matches!(self.inner, Subject::Iri(_))
    }

    fn as_iri(&self) -> Option<&IriRef> {
        match &self.inner {
            Subject::Iri(u) => Some(u),
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
