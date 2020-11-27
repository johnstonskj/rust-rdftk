/*!
A single statement (subject, predicate, object) in the RDF data mode.

# Example

TBD

*/

#![allow(clippy::module_name_repetitions)]

use crate::literal::Literal;
use rdftk_iri::IRIRef;
use rdftk_names::rdf;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use unique_id::sequence::SequenceGenerator as IDGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Subject {
    BNode(String),
    IRI(IRIRef),
    Star(Rc<Statement>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubjectNode {
    inner: Subject,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Object {
    BNode(String),
    IRI(IRIRef),
    Literal(Box<Literal>),
    Star(Rc<Statement>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectNode {
    inner: Object,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Context {
    Default,
    BNode(String),
    IRI(IRIRef),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContextNode {
    inner: Context,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Statement {
    subject: SubjectNode,
    predicate: IRIRef,
    object: ObjectNode,
    context: ContextNode,
}

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
                Subject::Star(st) => format!("<{}>", st.to_string_no_dot()),
            }
        )
    }
}

impl From<IRIRef> for SubjectNode {
    fn from(iri: IRIRef) -> Self {
        SubjectNode::named(iri)
    }
}

impl From<&IRIRef> for SubjectNode {
    fn from(iri: &IRIRef) -> Self {
        SubjectNode::named(iri.clone())
    }
}

impl From<Statement> for SubjectNode {
    fn from(st: Statement) -> Self {
        SubjectNode::about(Rc::from(st))
    }
}

impl From<&Statement> for SubjectNode {
    fn from(st: &Statement) -> Self {
        SubjectNode::about(Rc::from(st.clone()))
    }
}

impl From<Rc<Statement>> for SubjectNode {
    fn from(st: Rc<Statement>) -> Self {
        SubjectNode::about(st)
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

    pub fn named(name: IRIRef) -> Self {
        Self {
            inner: Subject::IRI(name),
        }
    }

    pub fn about(st: Rc<Statement>) -> Self {
        Self {
            inner: Subject::Star(st),
        }
    }

    pub fn is_blank(&self) -> bool {
        matches!(self.inner, Subject::BNode(_))
    }

    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Subject::BNode(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_iri(&self) -> bool {
        matches!(self.inner, Subject::IRI(_))
    }

    #[inline]
    pub fn is_named(&self) -> bool {
        self.is_iri()
    }

    pub fn as_iri(&self) -> Option<&IRIRef> {
        match &self.inner {
            Subject::IRI(u) => Some(u),
            _ => None,
        }
    }

    pub fn is_statement(&self) -> bool {
        matches!(self.inner, Subject::Star(_))
    }

    pub fn as_statement(&self) -> Option<&Rc<Statement>> {
        match &self.inner {
            Subject::Star(st) => Some(st),
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
                Object::Star(st) => format!("<{}>", st.to_string_no_dot()),
            }
        )
    }
}

impl From<IRIRef> for ObjectNode {
    fn from(iri: IRIRef) -> Self {
        ObjectNode::named(iri)
    }
}

impl From<&IRIRef> for ObjectNode {
    fn from(iri: &IRIRef) -> Self {
        ObjectNode::named(iri.clone())
    }
}

impl From<Statement> for ObjectNode {
    fn from(st: Statement) -> Self {
        ObjectNode::about(Rc::from(st))
    }
}

impl From<Rc<Statement>> for ObjectNode {
    fn from(st: Rc<Statement>) -> Self {
        ObjectNode::about(st)
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
            Subject::Star(st) => ObjectNode::about(st),
        }
    }
}

impl From<&SubjectNode> for ObjectNode {
    fn from(subject: &SubjectNode) -> Self {
        match &subject.inner {
            Subject::BNode(node) => ObjectNode::blank_named(node),
            Subject::IRI(iri) => ObjectNode::named(iri.clone()),
            Subject::Star(st) => ObjectNode::about(st.clone()),
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

    pub fn named(name: IRIRef) -> Self {
        Self {
            inner: Object::IRI(name),
        }
    }

    pub fn about(st: Rc<Statement>) -> Self {
        Self {
            inner: Object::Star(st),
        }
    }

    pub fn is_blank(&self) -> bool {
        matches!(self.inner, Object::BNode(_))
    }

    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Object::BNode(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_iri(&self) -> bool {
        matches!(self.inner, Object::IRI(_))
    }

    pub fn as_iri(&self) -> Option<&IRIRef> {
        match &self.inner {
            Object::IRI(u) => Some(u),
            _ => None,
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(self.inner, Object::Literal(_))
    }

    pub fn as_literal(&self) -> Option<&Literal> {
        match &self.inner {
            Object::Literal(l) => Some(l),
            _ => None,
        }
    }

    pub fn is_statement(&self) -> bool {
        matches!(self.inner, Object::Star(_))
    }

    pub fn as_statement(&self) -> Option<&Rc<Statement>> {
        match &self.inner {
            Object::Star(st) => Some(st),
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
}

// ------------------------------------------------------------------------------------------------

impl Default for ContextNode {
    fn default() -> Self {
        Self {
            inner: Context::Default,
        }
    }
}

impl Display for ContextNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.inner {
                Context::Default => String::new(),
                Context::BNode(node) => format!(" _:{}", node),
                Context::IRI(iri) => format!(" <{}>", iri),
            }
        )
    }
}

impl From<IRIRef> for ContextNode {
    fn from(iri: IRIRef) -> Self {
        ContextNode::named(iri)
    }
}

impl From<&IRIRef> for ContextNode {
    fn from(iri: &IRIRef) -> Self {
        ContextNode::named(iri.clone())
    }
}

impl ContextNode {
    pub fn blank() -> Self {
        Self {
            inner: Context::BNode(new_blank_node_id()),
        }
    }

    pub fn blank_named(name: &str) -> Self {
        Self {
            inner: Context::BNode(name.to_string()),
        }
    }

    pub fn named(name: IRIRef) -> Self {
        Self {
            inner: Context::IRI(name),
        }
    }

    pub fn is_blank(&self) -> bool {
        matches!(self.inner, Context::BNode(_))
    }

    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Context::BNode(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_iri(&self) -> bool {
        matches!(self.inner, Context::IRI(_))
    }

    #[inline]
    pub fn is_named(&self) -> bool {
        self.is_iri()
    }

    pub fn as_iri(&self) -> Option<&IRIRef> {
        match &self.inner {
            Context::IRI(u) => Some(u),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} .", &self.to_string_no_dot())
    }
}

impl Statement {
    pub fn new(subject: SubjectNode, predicate: IRIRef, object: ObjectNode) -> Self {
        Self {
            subject,
            predicate,
            object,
            context: ContextNode::default(),
        }
    }

    pub fn with_context(
        subject: SubjectNode,
        predicate: IRIRef,
        object: ObjectNode,
        context: ContextNode,
    ) -> Self {
        Self {
            subject,
            predicate,
            object,
            context,
        }
    }

    pub fn also(&self, predicate: IRIRef, object: ObjectNode) -> Self {
        Self {
            subject: self.subject.clone(),
            predicate,
            object,
            context: self.context().clone(),
        }
    }

    pub fn about(&self, predicate: IRIRef, object: ObjectNode) -> Self {
        Self {
            subject: self.into(),
            predicate,
            object,
            context: self.context().clone(),
        }
    }

    pub fn subject(&self) -> &SubjectNode {
        &self.subject
    }

    pub fn predicate(&self) -> &IRIRef {
        &self.predicate
    }

    pub fn object(&self) -> &ObjectNode {
        &self.object
    }

    pub fn reify(&self) -> Vec<Statement> {
        reify_statement(self).1
    }

    pub fn is_nested(&self) -> bool {
        self.subject().is_statement() || self.object().is_statement()
    }

    pub fn context(&self) -> &ContextNode {
        &self.context
    }

    pub fn to_string_no_dot(&self) -> String {
        format!(
            "{} <{}> {}{}",
            &self.subject.to_string(),
            &self.predicate.to_string(),
            &self.object.to_string(),
            &self.context.to_string(),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn new_blank_node_id() -> String {
    format!("B{}", IDGenerator::default().next_id())
}

fn reify_statement(st: &Statement) -> (SubjectNode, Vec<Statement>) {
    let mut statements = Vec::default();
    let new_subject = SubjectNode::blank();
    statements.push(Statement::new(
        new_subject.clone(),
        rdf::a_type().clone(),
        rdf::statement().into(),
    ));
    if st.subject().is_statement() {
        let nested = reify_statement(st.subject().as_statement().unwrap());
        statements.extend(nested.1);
        statements.push(Statement::new(
            new_subject.clone(),
            rdf::subject().clone(),
            nested.0.into(),
        ));
    } else {
        statements.push(Statement::new(
            new_subject.clone(),
            rdf::subject().clone(),
            st.subject().into(),
        ));
    }
    statements.push(Statement::new(
        new_subject.clone(),
        rdf::predicate().clone(),
        st.predicate().into(),
    ));
    if st.object().is_statement() {
        let nested = reify_statement(st.object().as_statement().unwrap());
        statements.extend(nested.1);
        statements.push(Statement::new(
            new_subject.clone(),
            rdf::object().clone(),
            nested.0.into(),
        ));
    } else {
        statements.push(Statement::new(
            new_subject.clone(),
            rdf::object().clone(),
            st.object().clone(),
        ));
    }
    (new_subject, statements)
}

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
            rdf::a_type().clone(),
            rdfs::class().into(),
        );
        assert_eq!(st.to_string(), "_:01 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2000/01/rdf-schema#Class> .");
    }

    #[test]
    fn test_reify_a_statement() {
        let st = Statement::new(
            SubjectNode::blank_named("01"),
            rdf::a_type().clone(),
            rdfs::class().into(),
        );
        let sts = st.reify();
        assert_eq!(sts.len(), 4);
    }

    #[test]
    fn test_nested_statement() {
        let st = Statement::new(
            SubjectNode::blank_named("01"),
            rdf::a_type().clone(),
            rdfs::class().into(),
        );
        let st = Statement::new(st.into(), rdf::a_type().clone(), rdf::statement().into());
        println!("{}", st);
        let sts = st.reify();
        for st in &sts {
            println!("{}", st);
        }
        assert_eq!(sts.len(), 8);
    }

    #[test]
    fn test_make_literal_statement() {
        let st = Statement::new(
            SubjectNode::blank_named("01"),
            rdfs::label().clone(),
            Literal::new("some thing").into(),
        );
        assert_eq!(
            st.to_string(),
            "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"some thing\" ."
        );
    }

    #[test]
    fn test_make_quad() {
        let st = Statement::with_context(
            SubjectNode::blank_named("01"),
            rdfs::label().clone(),
            Literal::new("some thing").into(),
            ContextNode::blank_named("05"),
        );
        assert_eq!(
            st.to_string(),
            "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"some thing\" _:05 ."
        );
    }

    #[test]
    fn test_make_typed_literal_statement() {
        let st = Statement::new(
            SubjectNode::blank_named("01"),
            rdfs::label().clone(),
            Literal::with_type("2020", DataType::Int).into(),
        );
        assert_eq!(
            st.to_string(),
            "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"2020\"^^<http://www.w3.org/2001/XMLSchema#int> ."
        );
    }
}
