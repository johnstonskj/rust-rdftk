// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::error::Result;
use crate::model::literal::LiteralRef;
use crate::model::statement::{ObjectNodeRef, StatementRef, SubjectNodeRef};
use crate::model::Provided;
use rdftk_iri::IRIRef;
use std::fmt::Debug;
use std::sync::Arc;

///
/// A statement factory provides an interface to create new statements and statement nodes. This
/// allows for implementations where underlying shared resources are required and so may be owned
/// by the factory. This factory may only be retrieved using the `Graph::statement_factory` method.
///
pub trait StatementFactory: Debug + Provided {
    // --------------------------------------------------------------------------------------------
    // Whole statements
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new statement reference from the provided subject, predicate, and object.
    ///
    fn statement(
        &self,
        subject: SubjectNodeRef,
        predicate: IRIRef,
        object: ObjectNodeRef,
    ) -> Result<StatementRef>;

    ///
    /// Construct a new statement reference from the provided subject, predicate, and object.
    ///
    fn statement_with_predicate(
        &self,
        subject: StatementRef,
        predicate: IRIRef,
        object: ObjectNodeRef,
    ) -> Result<StatementRef>;

    ///
    /// Construct a new statement reference from the provided subject, predicate, and object.
    ///
    fn statement_with_object(
        &self,
        subject: StatementRef,
        object: ObjectNodeRef,
    ) -> Result<StatementRef>;

    // --------------------------------------------------------------------------------------------
    // Subject nodes
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new subject node reference, as a blank node with a randomly assigned name.
    ///
    fn blank_subject(&self) -> SubjectNodeRef;

    ///
    /// Construct a new subject node reference, as a blank node with the specified name. This
    /// will panic if name is not a valid blank node name.
    ///
    fn blank_subject_named(&self, name: &str) -> Result<SubjectNodeRef>;

    ///
    /// Construct a new subject node, with an IRI naming a resource.
    ///
    fn named_subject(&self, name: IRIRef) -> SubjectNodeRef;

    ///
    /// Construct a new subject node, where the subject **is an** existing statement. This is
    /// an extension specified by [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    fn statement_subject(&self, st: StatementRef) -> SubjectNodeRef;

    ///
    /// Return a new subject node reference, which refers to this object.
    ///
    fn object_as_subject(&self, obj: ObjectNodeRef) -> Option<SubjectNodeRef>;

    // --------------------------------------------------------------------------------------------
    // Predicate nodes
    // --------------------------------------------------------------------------------------------

    // --------------------------------------------------------------------------------------------
    // Object nodes
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new object node reference, as a blank node with a randomly assigned name.
    ///
    fn blank_object(&self) -> ObjectNodeRef;

    ///
    /// Construct a new object node reference, as a blank node with the specified name.
    ///
    fn blank_object_named(&self, name: &str) -> Result<ObjectNodeRef>;

    ///
    /// Construct a new object node, with an IRI naming a resource.
    ///
    fn named_object(&self, name: IRIRef) -> ObjectNodeRef;

    ///
    /// Construct a new object node, with with a literal value.
    ///
    fn literal_object(&self, value: LiteralRef) -> ObjectNodeRef;

    ///
    /// Construct a new object node, where the subject **is an** existing statement. This is
    /// an extension specified by [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    fn statement_object(&self, st: StatementRef) -> ObjectNodeRef;

    ///
    /// Return a new object node reference, which refers to this subject.
    ///
    fn subject_as_object(&self, st: SubjectNodeRef) -> ObjectNodeRef;
}

pub type StatementFactoryRef = Arc<dyn StatementFactory>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
