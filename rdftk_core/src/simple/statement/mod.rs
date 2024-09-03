/*!
Simple, in-memory implementation of the `Statement` and `StatementFactory` traits.
*/

use crate::error::{ErrorKind, Result};
use crate::model::features::Featured;
use crate::model::features::FEATURE_RDF_STAR;
use crate::model::literal::{LiteralFactoryRef, LiteralRef};
use crate::model::statement::{
    BlankNode, ObjectNodeRef, Statement, StatementFactory, StatementFactoryRef, StatementRef,
    SubjectNodeRef,
};
use crate::model::Provided;
use crate::simple::literal::literal_factory;
use crate::simple::statement::subject::Subject;
use lazy_static::lazy_static;
use rdftk_iri::IriRef;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `Statement` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleStatement {
    subject: SubjectNodeRef,
    predicate: IriRef,
    object: ObjectNodeRef,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Retrieve the `Statement` factory for `simple::SimpleStatement` instances.
///
pub fn statement_factory() -> StatementFactoryRef {
    FACTORY.clone()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `StatementFactory` trait.
///
#[derive(Clone, Debug)]
struct SimpleStatementFactory {}

lazy_static! {
    static ref FACTORY: Arc<SimpleStatementFactory> = Arc::new(SimpleStatementFactory {});
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleStatementFactory {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl StatementFactory for SimpleStatementFactory {
    fn statement(
        &self,
        subject: SubjectNodeRef,
        predicate: IriRef,
        object: ObjectNodeRef,
    ) -> Result<StatementRef> {
        if self.provider_id() == subject.provider_id() && self.provider_id() == object.provider_id()
        {
            Ok(Rc::new(SimpleStatement {
                subject,
                predicate,
                object,
            }))
        } else {
            Err(ErrorKind::ProviderMismatch(
                self.provider_id().to_string(),
                object.provider_id().to_string(),
            )
            .into())
        }
    }

    fn statement_with_predicate(
        &self,
        subject: StatementRef,
        predicate: IriRef,
        object: ObjectNodeRef,
    ) -> Result<StatementRef> {
        self.statement(subject.subject().clone(), predicate, object)
    }

    fn statement_with_object(
        &self,
        subject: StatementRef,
        object: ObjectNodeRef,
    ) -> Result<StatementRef> {
        self.statement(
            subject.subject().clone(),
            subject.predicate().clone(),
            object,
        )
    }

    fn blank_subject(&self, node: BlankNode) -> SubjectNodeRef {
        Rc::new(SimpleSubjectNode {
            inner: Subject::BNode(node),
        })
    }

    fn named_subject(&self, name: IriRef) -> SubjectNodeRef {
        Rc::new(SimpleSubjectNode {
            inner: Subject::Iri(name),
        })
    }

    fn statement_subject(&self, st: StatementRef) -> SubjectNodeRef {
        Rc::new(SimpleSubjectNode {
            inner: Subject::Star(st),
        })
    }

    fn object_as_subject(&self, obj: ObjectNodeRef) -> Option<SubjectNodeRef> {
        if let Some(blank) = obj.as_blank() {
            return Some(self.blank_subject_named(blank.as_ref()).unwrap());
        }
        if let Some(iri) = obj.as_iri() {
            return Some(self.named_subject(iri.clone()));
        }
        if let Some(st) = obj.as_statement() {
            return Some(self.statement_subject(st.clone()));
        }
        None
    }

    fn blank_object(&self, name: BlankNode) -> ObjectNodeRef {
        Rc::new(SimpleObjectNode::from(name))
    }

    fn named_object(&self, name: IriRef) -> ObjectNodeRef {
        Rc::new(SimpleObjectNode::from(name))
    }

    fn literal_object(&self, value: LiteralRef) -> ObjectNodeRef {
        Rc::new(SimpleObjectNode::from(value))
    }

    fn statement_object(&self, st: StatementRef) -> ObjectNodeRef {
        Rc::new(SimpleObjectNode::from(st))
    }

    fn subject_as_object(&self, sub: SubjectNodeRef) -> ObjectNodeRef {
        if let Some(blank) = sub.as_blank() {
            return self.blank_object(blank.clone());
        }
        if let Some(iri) = sub.as_iri() {
            return self.named_object(iri.clone());
        }
        if let Some(st) = sub.as_statement() {
            return self.statement_object(st.clone());
        }
        unreachable!()
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for SimpleStatement {
    fn supports_feature(&self, feature: &IriRef) -> bool {
        feature == FEATURE_RDF_STAR.deref()
    }
}

impl Statement for SimpleStatement {
    fn subject(&self) -> &SubjectNodeRef {
        &self.subject
    }

    fn set_subject(&mut self, subject: SubjectNodeRef) {
        self.subject = subject;
    }

    fn predicate(&self) -> &IriRef {
        &self.predicate
    }

    fn set_predicate(&mut self, predicate: IriRef) {
        self.predicate = predicate;
    }

    fn object(&self) -> &ObjectNodeRef {
        &self.object
    }

    fn set_object(&mut self, object: ObjectNodeRef) {
        self.object = object;
    }

    fn factory(&self) -> StatementFactoryRef {
        statement_factory()
    }

    fn literal_factory(&self) -> LiteralFactoryRef {
        literal_factory()
    }

    fn is_nested(&self) -> bool {
        self.subject().is_statement() || self.object().is_statement()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod subject;
pub use subject::SimpleSubjectNode;

#[doc(hidden)]
pub mod object;
pub use object::SimpleObjectNode;
