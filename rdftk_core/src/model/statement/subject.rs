use crate::model::statement::BlankNode;
use crate::model::statement::{StatementRef, BLANK_NODE_NAMESPACE};
use crate::model::{Equiv, Provided};
use rdftk_iri::IriRef;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait models the *subject* component of an RDF statement.
///
pub trait SubjectNode: Debug + Provided {
    // --------------------------------------------------------------------------------------------
    // Inner type checks/accessors
    // --------------------------------------------------------------------------------------------

    ///
    /// Return `true` if this subject is a blank node, else `false`.
    ///
    fn is_blank(&self) -> bool {
        self.as_blank().is_some()
    }

    ///
    /// Return a blank node string, if `self.is_blank()`, else `None`.
    ///
    fn as_blank(&self) -> Option<&BlankNode>;

    ///
    /// Return `true` if this subject is an Iri, else `false`.
    ///
    fn is_iri(&self) -> bool {
        self.as_iri().is_some()
    }

    ///
    /// Return a named node Iri, if `self.is_iri()`, else `None`.
    ///
    fn as_iri(&self) -> Option<&IriRef>;

    ///
    /// Return `true` if this subject is an [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html) statement, else `false`.
    ///
    fn is_statement(&self) -> bool {
        self.as_statement().is_some()
    }

    ///
    /// Return a statement reference, if `self.is_statement()`, else `None`.
    ///
    fn as_statement(&self) -> Option<&StatementRef>;
}

///
/// A reference counted wrapper around a [`SubjectNode`] instance.
///
pub type SubjectNodeRef = Rc<dyn SubjectNode>;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PartialEq<dyn SubjectNode> for dyn SubjectNode {
    fn eq(&self, other: &dyn SubjectNode) -> bool {
        if self.is_blank() && other.is_blank() {
            let lhs = self.as_blank().unwrap();
            let rhs = other.as_blank().unwrap();
            lhs == rhs
        } else if self.is_iri() && other.is_iri() {
            let lhs = self.as_iri().unwrap();
            let rhs = other.as_iri().unwrap();
            lhs == rhs
        } else if self.is_statement() && other.is_statement() {
            let lhs = self.as_statement().unwrap();
            let rhs = other.as_statement().unwrap();
            lhs == rhs
        } else {
            false
        }
    }
}

impl Eq for dyn SubjectNode {}

impl Hash for dyn SubjectNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_blank().hash(state);
        self.as_iri().hash(state);
        self.as_statement().hash(state);
    }
}

impl Display for dyn SubjectNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = self.as_blank() {
            write!(f, "{}:{}", BLANK_NODE_NAMESPACE, node)
        } else if let Some(iri) = self.as_iri() {
            write!(f, "<{}>", iri)
        } else if let Some(st) = self.as_statement() {
            write!(f, "<< {} >>", st.deref())
        } else {
            unreachable!()
        }
    }
}

impl Equiv<BlankNode> for dyn SubjectNode {
    fn eqv(&self, other: &BlankNode) -> bool {
        if let Some(value) = self.as_blank() {
            value == other
        } else {
            false
        }
    }
}

impl Equiv<IriRef> for dyn SubjectNode {
    fn eqv(&self, other: &IriRef) -> bool {
        if let Some(value) = self.as_iri() {
            value == other
        } else {
            false
        }
    }
}

impl Equiv<StatementRef> for dyn SubjectNode {
    fn eqv(&self, other: &StatementRef) -> bool {
        if let Some(value) = self.as_statement() {
            value == other
        } else {
            false
        }
    }
}

impl PartialOrd for dyn SubjectNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dyn SubjectNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_iri() && other.is_iri() {
            if let Some(iri) = self.as_iri() {
                if let Some(other_iri) = other.as_iri() {
                    return iri.cmp(other_iri);
                }
            }
        }
        if self.is_blank() && other.is_blank() {
            if let Some(blank) = self.as_blank() {
                if let Some(other_blank) = other.as_blank() {
                    return blank.cmp(other_blank);
                }
            }
        }
        if self.is_statement() && other.is_statement() {
            todo!("sorting rdf-star statements is not yet supported");
        }
        if self.is_iri() {
            Ordering::Less
        } else if self.is_blank() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}
