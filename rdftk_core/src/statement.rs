/*!
A single statement (subject, predicate, object) in the RDF data mode.

# Example

TBD

*/

#![allow(clippy::module_name_repetitions)]

use crate::literal::Literal;
use rdftk_iri::IRI;
use rdftk_names::rdf;
use std::fmt::{Display, Formatter};
use unique_id::sequence::SequenceGenerator as IDGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Subject {
    BNode(String),
    IRI(IRI),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubjectNode {
    inner: Subject,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Object {
    BNode(String),
    IRI(IRI),
    Literal(Box<Literal>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectNode {
    inner: Object,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Statement {
    subject: SubjectNode,
    predicate: IRI,
    object: ObjectNode,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SubjectNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.inner {
                Subject::BNode(node) => format!("_:{}", node),
                Subject::IRI(iri) => format!("<{}>", iri),
            }
        )
    }
}

impl From<IRI> for SubjectNode {
    fn from(iri: IRI) -> Self {
        SubjectNode::named(iri)
    }
}

impl From<&IRI> for SubjectNode {
    fn from(iri: &IRI) -> Self {
        SubjectNode::named(iri.clone())
    }
}

impl SubjectNode {
    pub fn blank() -> Self {
        Self {
            inner: Subject::BNode(new_blank_node_id()),
        }
    }

    pub fn blank_named(name: &str) -> Self {
        Self {
            inner: Subject::BNode(name.to_string()),
        }
    }

    pub fn named(name: IRI) -> Self {
        Self {
            inner: Subject::IRI(name),
        }
    }

    pub fn is_blank(&self) -> bool {
        match self.inner {
            Subject::BNode(_) => true,
            _ => false,
        }
    }

    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Subject::BNode(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_iri(&self) -> bool {
        match self.inner {
            Subject::IRI(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_named(&self) -> bool {
        self.is_iri()
    }

    pub fn as_iri(&self) -> Option<&IRI> {
        match &self.inner {
            Subject::IRI(u) => Some(u),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ObjectNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.inner {
                Object::BNode(node) => format!("_:{}", node),
                Object::IRI(iri) => format!("<{}>", iri),
                Object::Literal(literal) => literal.to_string(),
            }
        )
    }
}

impl From<IRI> for ObjectNode {
    fn from(iri: IRI) -> Self {
        ObjectNode::named(iri)
    }
}

impl From<&IRI> for ObjectNode {
    fn from(iri: &IRI) -> Self {
        ObjectNode::named(iri.clone())
    }
}

impl From<Literal> for ObjectNode {
    fn from(literal: Literal) -> Self {
        Self {
            inner: Object::Literal(Box::new(literal)),
        }
    }
}

impl From<SubjectNode> for ObjectNode {
    fn from(subject: SubjectNode) -> Self {
        match subject.inner {
            Subject::BNode(node) => ObjectNode::blank_named(&node),
            Subject::IRI(iri) => ObjectNode::named(iri),
        }
    }
}

impl From<&SubjectNode> for ObjectNode {
    fn from(subject: &SubjectNode) -> Self {
        match &subject.inner {
            Subject::BNode(node) => ObjectNode::blank_named(node),
            Subject::IRI(iri) => ObjectNode::named(iri.clone()),
        }
    }
}

impl ObjectNode {
    pub fn blank() -> Self {
        Self {
            inner: Object::BNode(new_blank_node_id()),
        }
    }

    pub fn blank_named(name: &str) -> Self {
        Self {
            inner: Object::BNode(name.to_string()),
        }
    }

    pub fn named(name: IRI) -> Self {
        Self {
            inner: Object::IRI(name),
        }
    }

    pub fn is_blank(&self) -> bool {
        match self.inner {
            Object::BNode(_) => true,
            _ => false,
        }
    }

    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Object::BNode(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_iri(&self) -> bool {
        match self.inner {
            Object::IRI(_) => true,
            _ => false,
        }
    }

    pub fn as_iri(&self) -> Option<&IRI> {
        match &self.inner {
            Object::IRI(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_subject(&self) -> Option<SubjectNode> {
        match &self.inner {
            Object::IRI(iri) => Some(SubjectNode::named(iri.clone())),
            Object::BNode(b) => Some(SubjectNode::blank_named(b)),
            _ => None,
        }
    }
    pub fn is_literal(&self) -> bool {
        match self.inner {
            Object::Literal(_) => true,
            _ => false,
        }
    }

    pub fn as_literal(&self) -> Option<&Literal> {
        match &self.inner {
            Object::Literal(l) => Some(l),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <{}> {} .",
            &self.subject.to_string(),
            &self.predicate.to_string(),
            &self.object.to_string()
        )
    }
}

impl Statement {
    pub fn new(subject: SubjectNode, predicate: IRI, object: ObjectNode) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }

    pub fn subject(&self) -> &SubjectNode {
        &self.subject
    }

    pub fn predicate(&self) -> &IRI {
        &self.predicate
    }

    pub fn object(&self) -> &ObjectNode {
        &self.object
    }

    pub fn reify(&self) -> Vec<Statement> {
        let mut statements = Vec::default();
        let new_subject = SubjectNode::blank();
        statements.push(Statement::new(
            new_subject.clone(),
            rdf::a_type(),
            rdf::statement().into(),
        ));
        statements.push(Statement::new(
            new_subject.clone(),
            rdf::subject(),
            self.subject().into(),
        ));
        statements.push(Statement::new(
            new_subject.clone(),
            rdf::predicate(),
            self.predicate().into(),
        ));
        statements.push(Statement::new(
            new_subject,
            rdf::object(),
            self.object().clone(),
        ));
        statements
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn new_blank_node_id() -> String {
    format!("B{}", IDGenerator::default().next_id())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataType;
    use rdftk_names::{rdf, rdfs};

    #[test]
    fn test_make_a_statement() {
        let st = Statement::new(
            SubjectNode::blank_named("01"),
            rdf::a_type(),
            rdfs::class().into(),
        );
        assert_eq!(st.to_string(), "_:01 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2000/01/rdf-schema#class> .");
    }

    #[test]
    fn test_make_literal_statement() {
        let st = Statement::new(
            SubjectNode::blank_named("01"),
            rdfs::label(),
            Literal::new("some thing").into(),
        );
        assert_eq!(
            st.to_string(),
            "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"some thing\" ."
        );
    }

    #[test]
    fn test_make_typed_literal_statement() {
        let st = Statement::new(
            SubjectNode::blank_named("01"),
            rdfs::label(),
            Literal::with_type("2020", DataType::Int).into(),
        );
        assert_eq!(
            st.to_string(),
            "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"2020\"^^<http://www.w3.org/2001/XMLSchema#int> ."
        );
    }
}
