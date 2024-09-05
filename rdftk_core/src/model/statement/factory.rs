use crate::error::Result;
use crate::model::literal::LiteralRef;
use crate::model::statement::BlankNode;
use crate::model::statement::{ObjectNodeRef, StatementRef, SubjectNodeRef};
use crate::model::Provided;
use rdftk_iri::IriRef;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait provides a factory for construction of statements, and statement components.
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
        predicate: IriRef,
        object: ObjectNodeRef,
    ) -> Result<StatementRef>;

    ///
    /// Construct a new statement reference from the provided subject, predicate, and object.
    ///
    fn statement_with_predicate(
        &self,
        subject: StatementRef,
        predicate: IriRef,
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
    fn blank_subject_new(&self) -> SubjectNodeRef {
        self.blank_subject(BlankNode::generate())
    }

    ///
    /// Construct a new subject node reference, from the provided node.
    ///
    fn blank_subject(&self, name: BlankNode) -> SubjectNodeRef;

    ///
    /// Construct a new subject node reference, as a blank node with the specified name.
    ///
    fn blank_subject_named(&self, name: &str) -> Result<SubjectNodeRef> {
        Ok(self.blank_subject(BlankNode::from_str(name)?))
    }

    ///
    /// Construct a new subject node, with an Iri naming a resource.
    ///
    fn named_subject(&self, name: IriRef) -> SubjectNodeRef;

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
    // Object nodes
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new object node reference, as a blank node with a randomly assigned name.
    ///
    fn blank_object_new(&self) -> ObjectNodeRef {
        self.blank_object(BlankNode::generate())
    }

    ///
    /// Construct a new object node reference, as a blank node with the specified name.
    ///
    fn blank_object(&self, name: BlankNode) -> ObjectNodeRef;

    ///
    /// Construct a new object node reference, as a blank node with the specified name.
    ///
    fn blank_object_named(&self, name: &str) -> Result<ObjectNodeRef> {
        Ok(self.blank_object(BlankNode::from_str(name)?))
    }

    ///
    /// Construct a new object node, with an Iri naming a resource.
    ///
    fn named_object(&self, name: IriRef) -> ObjectNodeRef;

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

///
/// A reference counted wrapper around a [`StatementFactory`] instance.
///
pub type StatementFactoryRef = Arc<dyn StatementFactory>;
