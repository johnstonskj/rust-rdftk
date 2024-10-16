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
* use rdftk_core::model::literal::Literal;
* use rdftk_core::model::statement::Statement;
* use rdftk_iri::Iri;
* use std::str::FromStr;
*
* let statement = Statement::new(
*     Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap(),
*     Iri::from_str("http://purl.org/dc/elements/1.1/title").unwrap(),
*     Literal::plain("Tony Benn"),
* );
* ```
*
*
*/

#![allow(clippy::module_name_repetitions)]

use crate::error::Result;
use crate::model::features::Featured;
use crate::model::features::FEATURE_RDF_STAR;
use rdftk_iri::Iri;
use rdftk_names::rdf;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `Statement` trait.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Statement {
    subject: SubjectNode,
    predicate: Iri,
    object: ObjectNode,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Statement {
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

// ------------------------------------------------------------------------------------------------

impl PartialOrd for Statement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Statement {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.subject.cmp(&other.subject) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.predicate.cmp(&other.predicate) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.object.cmp(&other.object)
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for Statement {
    fn supports_feature(&self, feature: &Iri) -> bool {
        *feature == *FEATURE_RDF_STAR
    }
}

impl Statement {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<S, O>(subject: S, predicate: Iri, object: O) -> Self
    where
        S: Into<SubjectNode>,
        O: Into<ObjectNode>,
    {
        Self {
            subject: subject.into(),
            predicate,
            object: object.into(),
        }
    }

    pub fn rdf_type<S, O>(subject: S, object: O) -> Self
    where
        S: Into<SubjectNode>,
        O: Into<ObjectNode>,
    {
        Self::new(subject, rdf::a_type().clone(), object)
    }

    // --------------------------------------------------------------------------------------------
    // Components
    // --------------------------------------------------------------------------------------------

    ///
    /// Return the subject of this statement.
    ///
    pub fn subject(&self) -> &SubjectNode {
        &self.subject
    }

    ///
    /// Set the value of this statement's subject.
    ///
    pub fn set_subject(&mut self, subject: SubjectNode) {
        self.subject = subject;
    }

    ///
    /// Return the predicate of this statement.
    ///
    pub fn predicate(&self) -> &Iri {
        &self.predicate
    }

    ///
    /// Set the value of this statement's predicate.
    ///
    pub fn set_predicate(&mut self, predicate: Iri) {
        self.predicate = predicate;
    }

    ///
    /// Return the object of this statement.
    ///
    pub fn object(&self) -> &ObjectNode {
        &self.object
    }

    ///
    /// Set the value of this statement's object.
    ///
    pub fn set_object(&mut self, object: ObjectNode) {
        self.object = object;
    }

    // --------------------------------------------------------------------------------------------
    // Other
    // --------------------------------------------------------------------------------------------

    ///
    /// This statement is considered nested if *either* subject or object is itself a statement
    /// ([RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html))
    ///
    pub fn is_nested(&self) -> bool {
        self.subject().is_statement() || self.object().is_statement()
    }

    ///
    /// Reify a single statement, returning the list of resulting statements.
    ///
    pub fn reify(&self) -> Result<(SubjectNode, Vec<Self>)> {
        let mut statements: Vec<Self> = Default::default();
        let new_subject: SubjectNode = BlankNode::generate().into();
        statements.push(Self::new(
            new_subject.clone(),
            rdf::a_type().clone(),
            rdf::statement().clone(),
        ));
        if let Some(statement) = self.subject().as_statement() {
            let nested = statement.reify()?;
            statements.extend(nested.1);
            statements.push(Self::new(
                new_subject.clone(),
                rdf::subject().clone(),
                nested.0.to_object(),
            ));
        } else {
            statements.push(Self::new(
                new_subject.clone(),
                rdf::subject().clone(),
                self.subject().to_object(),
            ));
        }
        statements.push(Self::new(
            new_subject.clone(),
            rdf::predicate().clone(),
            self.predicate().clone(),
        ));
        if let Some(statement) = self.object().as_statement() {
            let nested = statement.reify()?;
            statements.extend(nested.1);
            statements.push(Self::new(
                new_subject.clone(),
                rdf::object().clone(),
                nested.0.to_object(),
            ));
        } else {
            statements.push(Self::new(
                new_subject.clone(),
                rdf::object().clone(),
                self.object().clone(),
            ));
        }
        Ok((new_subject, statements))
    }
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
