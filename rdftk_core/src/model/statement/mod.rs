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
* use rdftk_core::model::Implementation;
* use rdftk_core::model::literal::LiteralFactory;
* use rdftk_core::model::statement::{Statement, StatementFactory};
* use rdftk_core::simple::Implementation as SimpleImplementation;
* use rdftk_core::simple::statement::SimpleStatement;
* use rdftk_iri::Iri;
* use std::rc::Rc;
* use std::str::FromStr;
*
* let factories = SimpleImplementation::default();
* let statement_factory = factories.statement_factory();
* let literal_factory = factories.literal_factory();
* let mut statements: Vec<SimpleStatement> = Default::default();
*
* statements.push(statement_factory.statement(
*     statement_factory.named_subject(
*         Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap().into()
*     ),
*     Iri::from_str("http://purl.org/dc/elements/1.1/title").unwrap().into(),
*     statement_factory.literal_object(literal_factory.string("Tony Benn")),
* ).unwrap());
* ```
*
*
*/

#![allow(clippy::module_name_repetitions)]

use crate::error::Result;
use crate::model::features::Featured;
use crate::model::literal::Literal;
use crate::model::Provided;
use rdftk_iri::Iri;
use rdftk_names::rdf;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Graphs
// ------------------------------------------------------------------------------------------------

///
/// This trait models an RDF statement.
///
pub trait Statement: Clone + Debug + Featured + Provided {
    type Literal: Literal;

    ///
    /// Return the subject of this statement.
    ///
    fn subject(&self) -> &SubjectNode<Self::Literal, Self>;

    ///
    /// Set the value of this statement's subject.
    ///
    fn set_subject(&mut self, subject: SubjectNode<Self::Literal, Self>);

    ///
    /// Return the predicate of this statement.
    ///
    fn predicate(&self) -> &Iri;

    ///
    /// Set the value of this statement's predicate.
    ///
    fn set_predicate(&mut self, predicate: Iri);

    ///
    /// Return the object of this statement.
    ///
    fn object(&self) -> &ObjectNode<Self::Literal, Self>;

    ///
    /// Set the value of this statement's object.
    ///
    fn set_object(&mut self, object: ObjectNode<Self::Literal, Self>);

    // --------------------------------------------------------------------------------------------
    // Other
    // --------------------------------------------------------------------------------------------

    ///
    /// This statement is considered nested if *either* subject or object is itself a statement
    /// ([RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html))
    ///
    fn is_nested(&self) -> bool {
        self.subject().is_statement() || self.object().is_statement()
    }
    ///
    /// Reify a single statement, returning the list of resulting statements.
    ///
    #[allow(clippy::type_complexity)]
    fn reify<F: StatementFactory<Literal = Self::Literal, Statement = Self>>(
        &self,
        statement_factory: &F,
    ) -> Result<(SubjectNode<Self::Literal, Self>, Vec<Self>)>
    where
        Self: Sized,
    {
        let mut statements: Vec<Self> = Default::default();
        let new_subject = statement_factory.blank_subject_new();
        statements.push(statement_factory.statement(
            new_subject.clone(),
            rdf::a_type().clone(),
            statement_factory.named_object(rdf::statement().clone()),
        )?);
        if let Some(statement) = self.subject().as_statement() {
            let nested = statement.reify(statement_factory)?;
            statements.extend(nested.1);
            statements.push(statement_factory.statement(
                new_subject.clone(),
                rdf::subject().clone(),
                statement_factory.subject_as_object(nested.0),
            )?);
        } else {
            statements.push(statement_factory.statement(
                new_subject.clone(),
                rdf::subject().clone(),
                statement_factory.subject_as_object(self.subject().clone()),
            )?);
        }
        statements.push(statement_factory.statement(
            new_subject.clone(),
            rdf::predicate().clone(),
            statement_factory.named_object(self.predicate().clone()),
        )?);
        if let Some(statement) = self.object().as_statement() {
            let nested = statement.reify(statement_factory)?;
            statements.extend(nested.1);
            statements.push(statement_factory.statement(
                new_subject.clone(),
                rdf::object().clone(),
                statement_factory.subject_as_object(nested.0),
            )?);
        } else {
            statements.push(statement_factory.statement(
                new_subject.clone(),
                rdf::object().clone(),
                self.object().clone(),
            )?);
        }
        Ok((new_subject, statements))
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Factories
// ------------------------------------------------------------------------------------------------

///
/// This trait provides a factory for construction of statements, and statement components.
///
pub trait StatementFactory: Debug + Provided {
    type Literal: Literal;
    type Statement: Statement<Literal = Self::Literal>;

    // --------------------------------------------------------------------------------------------
    // Whole statements
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new statement reference from the provided subject, predicate, and object.
    ///
    fn statement(
        &self,
        subject: SubjectNode<Self::Literal, Self::Statement>,
        predicate: Iri,
        object: ObjectNode<Self::Literal, Self::Statement>,
    ) -> Result<Self::Statement>;

    ///
    /// Construct a new statement reference from the provided subject, predicate, and object.
    ///
    fn statement_with_predicate(
        &self,
        subject: Self::Statement,
        predicate: Iri,
        object: ObjectNode<Self::Literal, Self::Statement>,
    ) -> Result<Self::Statement>;

    ///
    /// Construct a new statement reference from the provided subject, predicate, and object.
    ///
    fn statement_with_object(
        &self,
        subject: Self::Statement,
        object: ObjectNode<Self::Literal, Self::Statement>,
    ) -> Result<Self::Statement>;

    // --------------------------------------------------------------------------------------------
    // Subject nodes
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new subject node reference, as a blank node with a randomly assigned name.
    ///
    fn blank_subject_new(&self) -> SubjectNode<Self::Literal, Self::Statement> {
        self.blank_subject(BlankNode::generate())
    }

    ///
    /// Construct a new subject node reference, from the provided node.
    ///
    fn blank_subject(&self, name: BlankNode) -> SubjectNode<Self::Literal, Self::Statement>;

    ///
    /// Construct a new subject node reference, as a blank node with the specified name.
    ///
    fn blank_subject_named(
        &self,
        name: &str,
    ) -> Result<SubjectNode<Self::Literal, Self::Statement>> {
        Ok(self.blank_subject(BlankNode::from_str(name)?))
    }

    ///
    /// Construct a new subject node, with an Iri naming a resource.
    ///
    fn named_subject(&self, name: Iri) -> SubjectNode<Self::Literal, Self::Statement>;

    ///
    /// Construct a new subject node, where the subject **is an** existing statement. This is
    /// an extension specified by [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    fn statement_subject(
        &self,
        st: Arc<Self::Statement>,
    ) -> SubjectNode<Self::Literal, Self::Statement>;

    ///
    /// Return a new subject node reference, which refers to this object.
    ///
    fn object_as_subject(
        &self,
        obj: ObjectNode<Self::Literal, Self::Statement>,
    ) -> Option<SubjectNode<Self::Literal, Self::Statement>>;

    // --------------------------------------------------------------------------------------------
    // Object nodes
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new object node reference, as a blank node with a randomly assigned name.
    ///
    fn blank_object_new(&self) -> ObjectNode<Self::Literal, Self::Statement> {
        self.blank_object(BlankNode::generate())
    }

    ///
    /// Construct a new object node reference, as a blank node with the specified name.
    ///
    fn blank_object(&self, name: BlankNode) -> ObjectNode<Self::Literal, Self::Statement>;

    ///
    /// Construct a new object node reference, as a blank node with the specified name.
    ///
    fn blank_object_named(&self, name: &str) -> Result<ObjectNode<Self::Literal, Self::Statement>> {
        Ok(self.blank_object(BlankNode::from_str(name)?))
    }

    ///
    /// Construct a new object node, with an Iri naming a resource.
    ///
    fn named_object(&self, name: Iri) -> ObjectNode<Self::Literal, Self::Statement>;

    ///
    /// Construct a new object node, with with a literal value.
    ///
    fn literal_object(&self, value: Self::Literal) -> ObjectNode<Self::Literal, Self::Statement>;

    ///
    /// Construct a new object node, where the subject **is an** existing statement. This is
    /// an extension specified by [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    fn statement_object(
        &self,
        st: Arc<Self::Statement>,
    ) -> ObjectNode<Self::Literal, Self::Statement>;

    ///
    /// Return a new object node reference, which refers to this subject.
    ///
    fn subject_as_object(
        &self,
        st: SubjectNode<Self::Literal, Self::Statement>,
    ) -> ObjectNode<Self::Literal, Self::Statement>;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod bnode;
pub use bnode::*;

mod subject;
pub use subject::*;

mod object;
pub use object::*;
