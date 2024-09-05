use crate::model::literal::LiteralRef;
use crate::model::statement::{BlankNode, StatementRef, SubjectNode, BLANK_NODE_NAMESPACE};
use crate::model::Equiv;
use rdftk_iri::IriRef;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait models the *object* component of an RDF statement.
///
pub trait ObjectNode: SubjectNode {
    ///
    /// Return `true` if this object is a literal value, else `false`.
    ///
    fn is_literal(&self) -> bool {
        self.as_literal().is_some()
    }

    ///
    /// Return a literal value, if `self.is_literal()`, else `None`.
    ///
    fn as_literal(&self) -> Option<&LiteralRef>;
}

///
/// A reference counted wrapper around a [`ObjectNode`] instance.
///
pub type ObjectNodeRef = Rc<dyn ObjectNode>;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PartialEq<dyn ObjectNode> for dyn ObjectNode {
    fn eq(&self, other: &dyn ObjectNode) -> bool {
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
        } else if self.is_literal() && other.is_literal() {
            let lhs = self.as_literal().unwrap();
            let rhs = other.as_literal().unwrap();
            lhs == rhs
        } else {
            false
        }
    }
}

impl Eq for dyn ObjectNode {}

impl Hash for dyn ObjectNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_blank().hash(state);
        self.as_iri().hash(state);
        self.as_statement().hash(state);
        self.as_literal().hash(state);
    }
}

impl Display for dyn ObjectNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = self.as_blank() {
            write!(f, "{}:{}", BLANK_NODE_NAMESPACE, node)
        } else if let Some(iri) = self.as_iri() {
            write!(f, "<{}>", iri)
        } else if let Some(literal) = self.as_literal() {
            write!(f, "{}", literal)
        } else if let Some(st) = self.as_statement() {
            write!(f, "<< {} >>", st.deref())
        } else {
            unreachable!()
        }
    }
}

impl Equiv<BlankNode> for dyn ObjectNode {
    fn eqv(&self, other: &BlankNode) -> bool {
        if let Some(value) = self.as_blank() {
            value == other
        } else {
            false
        }
    }
}

impl Equiv<IriRef> for dyn ObjectNode {
    fn eqv(&self, other: &IriRef) -> bool {
        if let Some(value) = self.as_iri() {
            value == other
        } else {
            false
        }
    }
}

impl Equiv<StatementRef> for dyn ObjectNode {
    fn eqv(&self, other: &StatementRef) -> bool {
        if let Some(value) = self.as_statement() {
            value == other
        } else {
            false
        }
    }
}

impl Equiv<LiteralRef> for dyn ObjectNode {
    fn eqv(&self, other: &LiteralRef) -> bool {
        if let Some(value) = self.as_literal() {
            value == other
        } else {
            false
        }
    }
}
