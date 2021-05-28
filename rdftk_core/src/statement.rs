/*!
This module provides types for the RDF Statement (triple) concept.

1. A **statement** comprises a subject, a predicate, and an object.
1. A **subject** may be a blank (unnamed) node, an IRI (named node), or a statement reference
   according to RDF-star.
1. A **predicate** is an IRI.
1. An **object** may be a blank (unnamed) node, an IRI (named node), a literal value, or a statement
   reference according to RDF-star.
1. A **literal** has a string-like *lexical form* and may have an asserted data type or a language
   identifier.

# Example


```rust
use rdftk_core::{Statement, SubjectNode, ObjectNode};
use rdftk_core::statement::StatementList;
use rdftk_iri::IRI;
use std::rc::Rc;
use std::str::FromStr;

let mut statements: StatementList = Default::default();

statements.push(Statement::new_ref(
    SubjectNode::from(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()).into(),
    IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap().into(),
    ObjectNode::literal_str("Tony Benn").into(),
));
```


*/

#![allow(clippy::module_name_repetitions)]

use crate::literal::Literal;
use crate::DataType;
use rdftk_iri::{IRIRef, IRI};
use rdftk_names::rdf;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use unique_id::sequence::SequenceGenerator as IDGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A subject node anchors one or more statements in a graph such that all statements with a common
/// subject are considered to be statements about the same thing.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubjectNode {
    inner: Subject,
}

///
/// The actual subject storage type, reference counted for memory management.
///
pub type SubjectNodeRef = Rc<SubjectNode>;

///
/// An object node may be another resource, or a literal value. In this way graphs can easily be
/// made using resource links.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectNode {
    inner: Object,
}

///
/// The actual object storage type, reference counted for memory management.
///
pub type ObjectNodeRef = Rc<ObjectNode>;

///
/// A statement comprises a subject, a predicate, and an object.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Statement {
    subject: SubjectNodeRef,
    predicate: IRIRef,
    object: ObjectNodeRef,
}

///
/// The actual statement storage type, reference counted for memory management.
///
pub type StatementRef = Rc<Statement>;

///
/// A list of statements, this can be used to pass non-graph sets of statements.
///
pub type StatementList = Vec<StatementRef>;

///
/// The reserved namespace value used to identify a serialized blank node.
///
pub const BLANK_NODE_NAMESPACE: &str = "_";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Subject {
    BNode(String),
    IRI(IRIRef),
    Star(StatementRef),
    //    Formulae(FormulaRef),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Object {
    BNode(String),
    IRI(IRIRef),
    Literal(Literal),
    Star(StatementRef),
    //    Formulae(FormulaRef),
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
                Subject::BNode(node) => format!("{}:{}", BLANK_NODE_NAMESPACE, node),
                Subject::IRI(iri) => format!("<{}>", iri),
                Subject::Star(st) => format!("<< {} >>", st.to_string_no_dot()),
                //                Subject::Formulae(f) => f.to_string(),
            }
        )
    }
}

impl From<IRI> for SubjectNode {
    fn from(iri: IRI) -> Self {
        SubjectNode::named(iri.into())
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

impl From<&StatementRef> for SubjectNode {
    fn from(st: &StatementRef) -> Self {
        SubjectNode::about(st.clone())
    }
}

impl From<StatementRef> for SubjectNode {
    fn from(st: StatementRef) -> Self {
        SubjectNode::about(st)
    }
}

impl SubjectNode {
    ///
    /// Construct a new subject node, as a blank node with a randomly assigned name.
    ///
    pub fn blank() -> Self {
        Self::blank_named(&new_blank_node_id())
    }

    ///
    /// Construct a new subject node reference, as a blank node with a randomly assigned name.
    ///
    pub fn blank_ref() -> SubjectNodeRef {
        Rc::from(Self::blank())
    }

    ///
    /// Construct a new subject node, as a blank node with the specified name.
    ///
    pub fn blank_named(name: &str) -> Self {
        Self {
            inner: Subject::BNode(name.to_string()),
        }
    }

    /// Is this object a blank node, with the provided name value?
    pub fn eq_blank(&self, other: &str) -> bool {
        if let Some(value) = self.as_blank() {
            value == other
        } else {
            false
        }
    }

    ///
    /// Construct a new subject node, with an IRI naming a resource.
    ///
    pub fn named(name: IRIRef) -> Self {
        Self {
            inner: Subject::IRI(name),
        }
    }

    ///
    /// Construct a new subject node reference, with an IRI naming a resource.
    ///
    pub fn named_ref(name: IRIRef) -> SubjectNodeRef {
        Rc::from(Self::named(name))
    }

    /// Is this object a named node, with the provided IRI value?
    pub fn eq_iri(&self, other: &IRIRef) -> bool {
        if let Some(value) = self.as_iri() {
            value == other
        } else {
            false
        }
    }

    ///
    /// Construct a new subject node, where the subject **is an** existing statement. This is
    /// an extension specified by [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    pub fn about(st: StatementRef) -> Self {
        Self {
            inner: Subject::Star(st),
        }
    }

    ///
    /// Construct a new subject node reference, where the subject **is an** existing statement.
    /// This is an extension specified by [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    pub fn about_ref(st: StatementRef) -> SubjectNodeRef {
        Rc::from(Self::about(st))
    }

    // ///
    // /// Construct a new subject node, where the subject is a formula (quoted graph).
    // ///
    // pub fn formula(f: FormulaRef) -> Self {
    //     Self {
    //         inner: Subject::Formulae(f),
    //     }
    // }
    //
    // ///
    // /// Construct a new subject node reference, where the subject is a formula (quoted graph).
    // ///
    // pub fn formula_ref(f: FormulaRef) -> SubjectNodeRef {
    //     Rc::from(Self::formula(f))
    // }

    /// Is this object a literal, with the provided value?
    pub fn eq_statement(&self, other: &StatementRef) -> bool {
        if let Some(value) = self.as_statement() {
            value == other
        } else {
            false
        }
    }

    ///
    /// Return `true` if this subject is a blank node, else `false`.
    ///
    pub fn is_blank(&self) -> bool {
        matches!(self.inner, Subject::BNode(_))
    }

    ///
    /// Return a blank node string, if `self.is_blank()`, else `None`.
    ///
    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Subject::BNode(s) => Some(s),
            _ => None,
        }
    }

    ///
    /// Return `true` if this subject is an IRI, else `false`.
    ///
    pub fn is_iri(&self) -> bool {
        matches!(self.inner, Subject::IRI(_))
    }

    ///
    /// Return `true` if this subject is an IRI, else `false`.
    ///
    #[inline]
    pub fn is_named(&self) -> bool {
        self.is_iri()
    }

    ///
    /// Return a named node IRI, if `self.is_iri()`, else `None`.
    ///
    pub fn as_iri(&self) -> Option<&IRIRef> {
        match &self.inner {
            Subject::IRI(u) => Some(u),
            _ => None,
        }
    }

    ///
    /// Return `true` if this subject is an [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html) statement, else `false`.
    ///
    pub fn is_statement(&self) -> bool {
        matches!(self.inner, Subject::Star(_))
    }

    ///
    /// Return a statement reference, if `self.is_statement()`, else `None`.
    ///
    pub fn as_statement(&self) -> Option<&StatementRef> {
        match &self.inner {
            Subject::Star(st) => Some(st),
            _ => None,
        }
    }

    // ///
    // /// Return `true` if this subject is a [N3 Formula](https://www.w3.org/TeamSubmission/n3/#Quoting) statement, else `false`.
    // ///
    // pub fn is_formula(&self) -> bool {
    //     matches!(self.inner, Subject::Formulae(_))
    // }
    //
    // ///
    // /// Return a formula reference, if `self.is_formula()`, else `None`.
    // ///
    // pub fn as_formula(&self) -> Option<&FormulaRef> {
    //     match &self.inner {
    //         Subject::Formulae(f) => Some(f),
    //         _ => None,
    //     }
    // }

    ///
    /// Return a new object node reference, which refers to this subject.
    ///
    pub fn as_object(&self) -> ObjectNodeRef {
        match &self.inner {
            Subject::BNode(node) => ObjectNode::blank_named(node),
            Subject::IRI(iri) => ObjectNode::named(iri.clone()),
            Subject::Star(st) => ObjectNode::about(st.clone()),
            //            Subject::Formulae(f) => ObjectNode::formula(f.clone()),
        }
        .into()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ObjectNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.inner {
                Object::BNode(node) => format!("{}:{}", BLANK_NODE_NAMESPACE, node),
                Object::IRI(iri) => format!("<{}>", iri),
                Object::Literal(literal) => literal.to_string(),
                Object::Star(st) => format!("<< {} >>", st.to_string_no_dot()),
                //                Object::Formulae(f) => f.to_string(),
            }
        )
    }
}

impl From<IRI> for ObjectNode {
    fn from(iri: IRI) -> Self {
        ObjectNode::named(iri.into())
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

impl From<Statement> for ObjectNodeRef {
    fn from(st: Statement) -> Self {
        ObjectNode::about_ref(Rc::from(st))
    }
}

impl From<Literal> for ObjectNodeRef {
    fn from(v: Literal) -> Self {
        ObjectNode::literal_ref(v)
    }
}

impl ObjectNode {
    ///
    /// Construct a new object node, as a blank node with a randomly assigned name.
    ///
    pub fn blank() -> Self {
        Self::blank_named(&new_blank_node_id())
    }

    ///
    /// Construct a new object node reference, as a blank node with a randomly assigned name.
    ///
    pub fn blank_ref() -> ObjectNodeRef {
        Rc::from(Self::blank())
    }

    ///
    /// Construct a new object node, as a blank node with the specified name.
    ///
    pub fn blank_named(name: &str) -> Self {
        Self {
            inner: Object::BNode(name.to_string()),
        }
    }

    /// Is this object a blank node, with the provided name value?
    pub fn eq_blank(&self, other: &str) -> bool {
        if let Some(value) = self.as_blank() {
            value == other
        } else {
            false
        }
    }

    ///
    /// Construct a new object node, with an IRI naming a resource.
    ///
    pub fn named(name: IRIRef) -> Self {
        Self {
            inner: Object::IRI(name),
        }
    }

    ///
    /// Construct a new object node reference, with an IRI naming a resource.
    ///
    pub fn named_ref(name: IRIRef) -> ObjectNodeRef {
        Rc::from(Self::named(name))
    }

    /// Is this object a named node, with the provided IRI value?
    pub fn eq_iri(&self, other: &IRIRef) -> bool {
        if let Some(value) = self.as_iri() {
            value == other
        } else {
            false
        }
    }

    ///
    /// Construct a new object node, with a literal value.
    ///
    pub fn literal(literal: Literal) -> Self {
        Self {
            inner: Object::Literal(literal),
        }
    }

    ///
    /// Construct a new object node reference, with a literal value.
    ///
    pub fn literal_ref(literal: Literal) -> ObjectNodeRef {
        Rc::from(Self::literal(literal))
    }

    ///
    /// Construct a new object node, with an untyped literal value.
    ///
    pub fn literal_str(value: &str) -> Self {
        Self::literal(Literal::new(value))
    }

    ///
    /// Construct a new object node, with an typed literal value.
    ///
    pub fn literal_with_type(value: &str, data_type: DataType) -> Self {
        Self::literal(Literal::with_type(value, data_type))
    }

    ///
    /// Construct a new object node, with an untyped literal value with language identifier.
    ///
    pub fn literal_with_language(value: &str, language: &str) -> Self {
        Self::literal(Literal::with_language(value, language))
    }

    /// Is this object a literal, with the provided value?
    pub fn eq_literal(&self, other: &Literal) -> bool {
        if let Some(value) = self.as_literal() {
            value == other
        } else {
            false
        }
    }

    ///
    /// Construct a new object node, where the object **is an** existing statement. This is
    /// an extension specified by [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    pub fn about(st: StatementRef) -> Self {
        Self {
            inner: Object::Star(st),
        }
    }

    ///
    /// Construct a new object node reference, where the object **is an** existing statement. This is
    /// an extension specified by [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    pub fn about_ref(st: StatementRef) -> ObjectNodeRef {
        Rc::from(Self::about(st))
    }

    // ///
    // /// Construct a new object node, where the subject is a formula (quoted graph).
    // ///
    // pub fn formula(f: FormulaRef) -> Self {
    //     Self {
    //         inner: Object::Formulae(f),
    //     }
    // }
    //
    // ///
    // /// Construct a new object node reference, where the subject is a formula (quoted graph).
    // ///
    // pub fn formula_ref(f: FormulaRef) -> ObjectNodeRef {
    //     Rc::from(Self::formula(f))
    // }

    /// Is this object a literal, with the provided value?
    pub fn eq_statement(&self, other: &StatementRef) -> bool {
        if let Some(value) = self.as_statement() {
            value == other
        } else {
            false
        }
    }

    ///
    /// Return `true` if this object is a blank node, else `false`.
    ///
    pub fn is_blank(&self) -> bool {
        matches!(self.inner, Object::BNode(_))
    }

    ///
    /// Return a blank node string, if `self.is_blank()`, else `None`.
    ///
    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Object::BNode(s) => Some(s),
            _ => None,
        }
    }

    ///
    /// Return `true` if this object is an IRI, else `false`.
    ///
    pub fn is_iri(&self) -> bool {
        matches!(self.inner, Object::IRI(_))
    }

    ///
    /// Return a named node IRI, if `self.is_iri()`, else `None`.
    ///
    pub fn as_iri(&self) -> Option<&IRIRef> {
        match &self.inner {
            Object::IRI(u) => Some(u),
            _ => None,
        }
    }

    ///
    /// Return `true` if this object is a literal value, else `false`.
    ///
    pub fn is_literal(&self) -> bool {
        matches!(self.inner, Object::Literal(_))
    }

    ///
    /// Return a literal value, if `self.is_literal()`, else `None`.
    ///
    pub fn as_literal(&self) -> Option<&Literal> {
        match &self.inner {
            Object::Literal(l) => Some(l),
            _ => None,
        }
    }

    ///
    /// Return `true` if this object is an [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html) statement, else `false`.
    ///
    pub fn is_statement(&self) -> bool {
        matches!(self.inner, Object::Star(_))
    }

    ///
    /// Return a statement reference, if `self.is_statement()`, else `None`.
    ///
    pub fn as_statement(&self) -> Option<&StatementRef> {
        match &self.inner {
            Object::Star(st) => Some(st),
            _ => None,
        }
    }

    // ///
    // /// Return `true` if this object is a [N3 Formula](https://www.w3.org/TeamSubmission/n3/#Quoting) statement, else `false`.
    // ///
    // pub fn is_formula(&self) -> bool {
    //     matches!(self.inner, Object::Formulae(_))
    // }
    //
    // ///
    // /// Return a formula reference, if `self.is_formula()`, else `None`.
    // ///
    // pub fn as_formula(&self) -> Option<&FormulaRef> {
    //     match &self.inner {
    //         Object::Formulae(f) => Some(f),
    //         _ => None,
    //     }
    // }

    ///
    /// Return a new subject node reference, which refers to this object, if it is not
    /// a literal.
    ///
    pub fn as_subject(&self) -> Option<SubjectNode> {
        match &self.inner {
            Object::IRI(iri) => Some(SubjectNode::named(iri.clone())),
            Object::BNode(b) => Some(SubjectNode::blank_named(b)),
            Object::Star(st) => Some(SubjectNode::about(st.clone())),
            //            Object::Formulae(f) => Some(SubjectNode::formula(f.clone())),
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
    ///
    /// Construct a new statement from the provided subject, predicate, and object.
    ///
    pub fn new(subject: SubjectNodeRef, predicate: IRIRef, object: ObjectNodeRef) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }

    ///
    /// Construct a new statement reference from the provided subject, predicate, and object.
    ///
    pub fn new_ref(
        subject: SubjectNodeRef,
        predicate: IRIRef,
        object: ObjectNodeRef,
    ) -> StatementRef {
        Rc::from(Self::new(subject, predicate, object))
    }

    ///
    /// Construct a new statement with the provided predicate and object but using self's
    /// subject.
    ///
    pub fn also(&self, predicate: IRIRef, object: ObjectNodeRef) -> Self {
        Self {
            subject: self.subject.clone(),
            predicate,
            object,
        }
    }

    ///
    /// Construct a new statement from the provided predicate, and object, but using self
    /// as the subject ([RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html)).
    ///
    pub fn about(&self, predicate: IRIRef, object: ObjectNodeRef) -> Self {
        Self {
            subject: SubjectNode::about(self.clone().into()).into(),
            predicate,
            object,
        }
    }

    ///
    /// Return the subject of this statement.
    ///
    pub fn subject(&self) -> &SubjectNodeRef {
        &self.subject
    }

    ///
    /// Set the value of this statement's subject.
    ///
    pub fn set_subject(&mut self, subject: SubjectNodeRef) {
        self.subject = subject;
    }

    ///
    /// Return the predicate of this statement.
    ///
    pub fn predicate(&self) -> &IRIRef {
        &self.predicate
    }

    ///
    /// Set the value of this statement's predicate.
    ///
    pub fn set_predicate(&mut self, predicate: IRIRef) {
        self.predicate = predicate;
    }

    ///
    /// Return the object of this statement.
    ///
    pub fn object(&self) -> &ObjectNodeRef {
        &self.object
    }

    ///
    /// Set the value of this statement's object.
    ///
    pub fn set_object(&mut self, object: ObjectNodeRef) {
        self.object = object;
    }

    ///
    /// RDF Reification is the process of turning a single statement into a set of statements, and
    /// more importantly giving an identity (in the form of a blank node) to the statement.
    ///
    pub fn reify(&self) -> Vec<StatementRef> {
        reify_statement(self).1
    }

    ///
    /// This statement is considered nested if *either* subject or object is itself a statement
    /// ([RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html))
    ///
    pub fn is_nested(&self) -> bool {
        self.subject().is_statement() || self.object().is_statement()
    }

    ///
    /// Return a string form of a statement, in this case it does not terminate with a "."
    /// character in the usual style.
    ///
    pub fn to_string_no_dot(&self) -> String {
        format!(
            "{} <{}> {}",
            &self.subject.to_string(),
            &self.predicate.to_string(),
            &self.object.to_string(),
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn new_blank_node_id() -> String {
    format!("B{}", IDGenerator::default().next_id())
}

fn reify_statement(st: &Statement) -> (SubjectNodeRef, Vec<StatementRef>) {
    let mut statements: Vec<StatementRef> = Default::default();
    let new_subject = Rc::from(SubjectNode::blank());
    statements.push(
        Statement::new(
            new_subject.clone(),
            rdf::a_type().clone(),
            ObjectNode::named(rdf::statement().clone()).into(),
        )
        .into(),
    );
    if st.subject().is_statement() {
        let nested = reify_statement(st.subject().as_statement().unwrap());
        statements.extend(nested.1);
        statements.push(
            Statement::new(
                new_subject.clone(),
                rdf::subject().clone(),
                nested.0.as_object(),
            )
            .into(),
        );
    } else {
        statements.push(
            Statement::new(
                new_subject.clone(),
                rdf::subject().clone(),
                st.subject().as_object(),
            )
            .into(),
        );
    }
    statements.push(
        Statement::new(
            new_subject.clone(),
            rdf::predicate().clone(),
            ObjectNode::named(st.predicate().clone()).into(),
        )
        .into(),
    );
    if st.object().is_statement() {
        let nested = reify_statement(st.object().as_statement().unwrap());
        statements.extend(nested.1);
        statements.push(
            Statement::new(
                new_subject.clone(),
                rdf::object().clone(),
                nested.0.as_object(),
            )
            .into(),
        );
    } else {
        statements.push(
            Statement::new(
                new_subject.clone(),
                rdf::object().clone(),
                st.object().clone(),
            )
            .into(),
        );
    }
    (new_subject, statements)
}
