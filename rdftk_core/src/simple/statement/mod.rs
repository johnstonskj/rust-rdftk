/*!
Simple, in-memory implementation of the `Statement` and `StatementFactory` traits.
*/

use crate::error::{ErrorKind, Result};
use crate::model::features::Featured;
use crate::model::features::FEATURE_RDF_STAR;
use crate::model::literal::{LiteralFactoryRef, LiteralRef};
use crate::model::statement::{
    ObjectNodeRef, Statement, StatementFactory, StatementFactoryRef, StatementRef, SubjectNodeRef,
};
use crate::model::Provided;
use crate::simple::statement::object::Object;
use crate::simple::statement::subject::Subject;
use rdftk_iri::IRIRef;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use unique_id::sequence::SequenceGenerator as IDGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `Statement` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleStatement {
    subject: SubjectNodeRef,
    predicate: IRIRef,
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
        predicate: IRIRef,
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
        predicate: IRIRef,
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

    fn blank_subject(&self) -> SubjectNodeRef {
        Rc::new(SimpleSubjectNode {
            inner: Subject::BNode(new_blank_node_id()),
        })
    }

    fn blank_subject_named(&self, name: &str) -> Result<SubjectNodeRef> {
        Ok(Rc::new(SimpleSubjectNode {
            inner: Subject::BNode(name.to_string()),
        }))
    }

    fn named_subject(&self, name: IRIRef) -> SubjectNodeRef {
        Rc::new(SimpleSubjectNode {
            inner: Subject::IRI(name),
        })
    }

    fn statement_subject(&self, st: StatementRef) -> SubjectNodeRef {
        Rc::new(SimpleSubjectNode {
            inner: Subject::Star(st),
        })
    }

    fn object_as_subject(&self, obj: ObjectNodeRef) -> Option<SubjectNodeRef> {
        if let Some(blank) = obj.as_blank() {
            return Some(self.blank_subject_named(blank).unwrap());
        }
        if let Some(iri) = obj.as_iri() {
            return Some(self.named_subject(iri.clone()));
        }
        if let Some(st) = obj.as_statement() {
            return Some(self.statement_subject(st.clone()));
        }
        None
    }

    fn blank_object(&self) -> ObjectNodeRef {
        Rc::new(SimpleObjectNode {
            inner: Object::BNode(new_blank_node_id()),
        })
    }

    fn blank_object_named(&self, name: &str) -> Result<ObjectNodeRef> {
        Ok(Rc::new(SimpleObjectNode {
            inner: Object::BNode(name.to_string()),
        }))
    }

    fn named_object(&self, name: IRIRef) -> ObjectNodeRef {
        Rc::new(SimpleObjectNode {
            inner: Object::IRI(name),
        })
    }

    fn literal_object(&self, value: LiteralRef) -> ObjectNodeRef {
        Rc::new(SimpleObjectNode {
            inner: Object::Literal(value),
        })
    }

    fn statement_object(&self, st: StatementRef) -> ObjectNodeRef {
        Rc::new(SimpleObjectNode {
            inner: Object::Star(st),
        })
    }

    fn subject_as_object(&self, sub: SubjectNodeRef) -> ObjectNodeRef {
        if let Some(blank) = sub.as_blank() {
            return self.blank_object_named(blank).unwrap();
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
    fn supports_feature(&self, feature: &IRIRef) -> bool {
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

    fn predicate(&self) -> &IRIRef {
        &self.predicate
    }

    fn set_predicate(&mut self, predicate: IRIRef) {
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

fn new_blank_node_id() -> String {
    format!("B{}", IDGenerator::default().next_id())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod subject;
pub use subject::SimpleSubjectNode;

#[doc(hidden)]
pub mod object;
use crate::simple::literal::literal_factory;
pub use object::SimpleObjectNode;
