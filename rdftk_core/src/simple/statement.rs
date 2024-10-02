/*!
Simple, in-memory implementation of the `Statement` and `StatementFactory` traits.
*/

use crate::error::{provider_mismatch_error, Result};
use crate::model::features::Featured;
use crate::model::features::FEATURE_RDF_STAR;
use crate::model::statement::{BlankNode, ObjectNode, Statement, StatementFactory, SubjectNode};
use crate::model::Provided;
use crate::simple::literal::SimpleLiteral;
use rdftk_iri::Iri;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type SimpleSubjectNode = SubjectNode<SimpleLiteral, SimpleStatement>;

pub type SimpleObjectNode = ObjectNode<SimpleLiteral, SimpleStatement>;

///
/// Simple, in-memory implementation of the `Statement` trait.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SimpleStatement {
    subject: SimpleSubjectNode,
    predicate: Iri,
    object: SimpleObjectNode,
}

///
/// Simple, in-memory implementation of the `StatementFactory` trait.
///
#[derive(Clone, Debug, Default)]
pub struct SimpleStatementFactory {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleStatementFactory {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl StatementFactory for SimpleStatementFactory {
    type Literal = SimpleLiteral;
    type Statement = SimpleStatement;

    fn statement(
        &self,
        subject: SimpleSubjectNode,
        predicate: Iri,
        object: ObjectNode<Self::Literal, Self::Statement>,
    ) -> Result<Self::Statement> {
        if let Some(subject_id) = subject.provider_id() {
            if self.provider_id() != subject_id {
                return provider_mismatch_error(self.provider_id(), subject_id).into();
            }
        } else if let Some(object_id) = object.provider_id() {
            if self.provider_id() != object_id {
                return provider_mismatch_error(self.provider_id(), object_id).into();
            }
        }
        Ok(SimpleStatement {
            subject,
            predicate,
            object,
        })
    }

    fn statement_with_predicate(
        &self,
        subject: Self::Statement,
        predicate: Iri,
        object: SimpleObjectNode,
    ) -> Result<Self::Statement> {
        self.statement(subject.subject().clone(), predicate, object)
    }

    fn statement_with_object(
        &self,
        subject: Self::Statement,
        object: SimpleObjectNode,
    ) -> Result<Self::Statement> {
        self.statement(
            subject.subject().clone(),
            subject.predicate().clone(),
            object,
        )
    }

    fn blank_subject(&self, node: BlankNode) -> SimpleSubjectNode {
        SubjectNode::from(node)
    }

    fn named_subject(&self, name: Iri) -> SimpleSubjectNode {
        SubjectNode::from(name)
    }

    fn statement_subject(&self, st: Arc<Self::Statement>) -> SimpleSubjectNode {
        SubjectNode::from(st)
    }

    fn object_as_subject(&self, obj: SimpleObjectNode) -> Option<SimpleSubjectNode> {
        match obj {
            ObjectNode::Blank(v) => Some(self.blank_subject(v.clone())),
            ObjectNode::Resource(v) => Some(self.named_subject(v.clone())),
            ObjectNode::Statement(v) => Some(self.statement_subject(v.clone())),
            ObjectNode::Literal(_) => None,
        }
    }

    fn blank_object(&self, name: BlankNode) -> SimpleObjectNode {
        ObjectNode::from(name)
    }

    fn named_object(&self, name: Iri) -> SimpleObjectNode {
        ObjectNode::from(name)
    }

    fn literal_object(&self, value: SimpleLiteral) -> SimpleObjectNode {
        ObjectNode::from(value)
    }

    fn statement_object(&self, st: Arc<Self::Statement>) -> SimpleObjectNode {
        ObjectNode::from(st)
    }

    fn subject_as_object(
        &self,
        sub: SimpleSubjectNode,
    ) -> ObjectNode<Self::Literal, Self::Statement> {
        match sub {
            SubjectNode::Blank(v) => self.blank_object(v.clone()),
            SubjectNode::Resource(v) => self.named_object(v.clone()),
            SubjectNode::Statement(v) => self.statement_object(v.clone()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for SimpleStatement {
    fn supports_feature(&self, feature: &Iri) -> bool {
        feature == FEATURE_RDF_STAR.deref()
    }
}

impl Statement for SimpleStatement {
    type Literal = SimpleLiteral;

    fn subject(&self) -> &SimpleSubjectNode {
        &self.subject
    }

    fn set_subject(&mut self, subject: SimpleSubjectNode) {
        self.subject = subject;
    }

    fn predicate(&self) -> &Iri {
        &self.predicate
    }

    fn set_predicate(&mut self, predicate: Iri) {
        self.predicate = predicate;
    }

    fn object(&self) -> &SimpleObjectNode {
        &self.object
    }

    fn set_object(&mut self, object: SimpleObjectNode) {
        self.object = object;
    }

    fn is_nested(&self) -> bool {
        self.subject().is_statement() || self.object().is_statement()
    }
}

impl Display for SimpleStatement {
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

impl Provided for SimpleStatement {
    fn provider_id(&self) -> &'static str {
        super::PROVIDER_ID
    }
}
