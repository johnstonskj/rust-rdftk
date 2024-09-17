use crate::model::statement::{BlankNodeRef, StatementRef, SubjectNode};
use crate::model::Provided;
use rdftk_iri::IriRef;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `SubjectNode` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleSubjectNode(Subject);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
enum Subject {
    BNode(BlankNodeRef),
    Iri(IriRef),
    Star(StatementRef),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<BlankNodeRef> for SimpleSubjectNode {
    fn from(v: BlankNodeRef) -> Self {
        Subject::BNode(v).into()
    }
}

impl From<IriRef> for SimpleSubjectNode {
    fn from(v: IriRef) -> Self {
        Subject::Iri(v).into()
    }
}

impl From<StatementRef> for SimpleSubjectNode {
    fn from(v: StatementRef) -> Self {
        Subject::Star(v).into()
    }
}

impl From<Subject> for SimpleSubjectNode {
    fn from(value: Subject) -> Self {
        Self(value)
    }
}

impl Provided for SimpleSubjectNode {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl SubjectNode for SimpleSubjectNode {
    fn is_blank(&self) -> bool {
        matches!(self.0, Subject::BNode(_))
    }

    fn as_blank(&self) -> Option<&BlankNodeRef> {
        match &self.0 {
            Subject::BNode(s) => Some(s),
            _ => None,
        }
    }

    fn is_iri(&self) -> bool {
        matches!(self.0, Subject::Iri(_))
    }

    fn as_iri(&self) -> Option<&IriRef> {
        match &self.0 {
            Subject::Iri(u) => Some(u),
            _ => None,
        }
    }

    fn is_statement(&self) -> bool {
        matches!(self.0, Subject::Star(_))
    }

    fn as_statement(&self) -> Option<&StatementRef> {
        match &self.0 {
            Subject::Star(st) => Some(st),
            _ => None,
        }
    }
}
