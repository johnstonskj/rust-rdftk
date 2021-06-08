/*!
* This module provides types for the RDF Statement (triple) concept.
*
* 1. A **statement** comprises a subject, a predicate, and an object.
* 1. A **subject** may be a blank (unnamed) node, an IRI (named node), or a statement reference
*    according to RDF-star.
* 1. A **predicate** is an IRI.
* 1. An **object** may be a blank (unnamed) node, an IRI (named node), a literal value, or a statement
*    reference according to RDF-star.
* 1. A **literal** has a string-like *lexical form* and may have an asserted data type or a language
*    identifier.
*
* # Example
*
*
* ```rust
* use rdftk_core::model::statement::{Statement, StatementList, ObjectNode};
* use rdftk_iri::IRI;
* use std::rc::Rc;
* use std::str::FromStr;use rdftk_core::simple::statement::statement_factory;use rdftk_core::simple::literal::literal_factory;
*
* let factory = statement_factory();
* let mut statements: StatementList = Default::default();
*
* statements.push(factory.statement(
*     factory.named_subject(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap().into()),
*     IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap().into(),
*     factory.literal_object(literal_factory().literal("Tony Benn")),
* ).unwrap());
* ```
*
*
*/

#![allow(clippy::module_name_repetitions)]

use crate::error::Result;
use crate::model::features::Featured;
use crate::model::literal::LiteralFactoryRef;
use rdftk_iri::IRIRef;
use rdftk_names::rdf;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait models an RDF statement.
///
pub trait Statement: Debug + Featured {
    ///
    /// Return the subject of this statement.
    ///
    fn subject(&self) -> &SubjectNodeRef;

    ///
    /// Set the value of this statement's subject.
    ///
    fn set_subject(&mut self, subject: SubjectNodeRef);

    ///
    /// Return the predicate of this statement.
    ///
    fn predicate(&self) -> &IRIRef;

    ///
    /// Set the value of this statement's predicate.
    ///
    fn set_predicate(&mut self, predicate: IRIRef);

    ///
    /// Return the object of this statement.
    ///
    fn object(&self) -> &ObjectNodeRef;

    ///
    /// Set the value of this statement's object.
    ///
    fn set_object(&mut self, object: ObjectNodeRef);

    // --------------------------------------------------------------------------------------------
    // Factories
    // --------------------------------------------------------------------------------------------

    ///
    /// Return the factory that creates statements using the same provider as `self`.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn factory(&self) -> StatementFactoryRef;

    ///
    /// Return the factory that creates literals using the same provider as `self`.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn literal_factory(&self) -> LiteralFactoryRef;

    // --------------------------------------------------------------------------------------------

    ///
    /// This statement is considered nested if *either* subject or object is itself a statement
    /// ([RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html))
    ///
    fn is_nested(&self) -> bool {
        self.subject().is_statement() || self.object().is_statement()
    }
}

///
/// The actual statement storage type, reference counted for memory management.
///
pub type StatementRef = Rc<dyn Statement>;

///
/// A list of statements, this can be used to pass non-model.graph sets of statements.
///
pub type StatementList = Vec<StatementRef>;

///
/// The reserved namespace value used to identify a serialized blank node.
///
pub const BLANK_NODE_NAMESPACE: &str = "_";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Reify a single statement, returning the list of resulting statements.
///
pub fn reify_statement(
    st: &StatementRef,
    factory: &StatementFactoryRef,
) -> Result<(SubjectNodeRef, Vec<StatementRef>)> {
    let mut statements: Vec<StatementRef> = Default::default();
    let new_subject = factory.blank_subject();
    statements.push(factory.statement(
        new_subject.clone(),
        rdf::a_type().clone(),
        factory.named_object(rdf::statement().clone()),
    )?);
    if st.subject().is_statement() {
        let nested = reify_statement(st.subject().as_statement().unwrap(), factory)?;
        statements.extend(nested.1);
        statements.push(factory.statement(
            new_subject.clone(),
            rdf::subject().clone(),
            factory.subject_as_object(nested.0),
        )?);
    } else {
        statements.push(factory.statement(
            new_subject.clone(),
            rdf::subject().clone(),
            factory.subject_as_object(st.subject().clone()),
        )?);
    }
    statements.push(factory.statement(
        new_subject.clone(),
        rdf::predicate().clone(),
        factory.named_object(st.predicate().clone()),
    )?);
    if st.object().is_statement() {
        let nested = reify_statement(st.object().as_statement().unwrap(), factory)?;
        statements.extend(nested.1);
        statements.push(factory.statement(
            new_subject.clone(),
            rdf::object().clone(),
            factory.subject_as_object(nested.0),
        )?);
    } else {
        statements.push(factory.statement(
            new_subject.clone(),
            rdf::object().clone(),
            st.object().clone(),
        )?);
    }
    Ok((new_subject, statements))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PartialEq<dyn Statement> for dyn Statement {
    fn eq(&self, other: &dyn Statement) -> bool {
        self.subject() == other.subject()
            && self.predicate() == other.predicate()
            && self.object() == other.object()
    }
}

impl Eq for dyn Statement {}

impl Display for dyn Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <{}> {}",
            &self.subject().to_string(),
            &self.predicate().to_string(),
            &self.object().to_string(),
        )
    }
}

impl Hash for dyn Statement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.subject().hash(state);
        self.predicate().hash(state);
        self.object().hash(state);
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod factory;
pub use factory::*;

#[doc(hidden)]
mod subject;
pub use subject::*;

#[doc(hidden)]
mod predicate;
pub use predicate::*;

#[doc(hidden)]
mod object;
pub use object::*;
