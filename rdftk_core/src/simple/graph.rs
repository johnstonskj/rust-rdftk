/*!
Simple, in-memory implementation of the `Graph` and `GraphFactory` traits.
*/

use crate::model::features::{Featured, FEATURE_GRAPH_DUPLICATES, FEATURE_RDF_STAR};
use crate::model::graph::{Graph, GraphFactory, GraphFactoryRef, GraphRef, PrefixMappingRef};
use crate::model::literal::LiteralFactoryRef;
use crate::model::statement::{
    ObjectNodeRef, StatementFactoryRef, StatementList, StatementRef, SubjectNodeRef,
};
use crate::model::Provided;
use crate::simple::empty_mappings;
use crate::simple::literal::literal_factory;
use crate::simple::statement::statement_factory;
use rdftk_iri::IRIRef;
use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `GraphFactory` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleGraphFactory {}

///
/// Simple, in-memory implementation of the `Graph` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleGraph {
    statements: StatementList,
    mappings: PrefixMappingRef,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Retrieve the `GraphFactory` factory for `simple::SimpleGraph` instances.
///
pub fn graph_factory() -> GraphFactoryRef {
    FACTORY.clone()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref FACTORY: Arc<SimpleGraphFactory> = Arc::new(SimpleGraphFactory::default());
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SimpleGraphFactory {
    fn default() -> Self {
        Self {}
    }
}

impl Provided for SimpleGraphFactory {
    fn provider_id(&self) -> &'static str {
        super::PROVIDER_ID
    }
}

impl GraphFactory for SimpleGraphFactory {
    fn graph(&self) -> GraphRef {
        self.with_mappings(empty_mappings())
    }

    fn with_mappings(&self, mappings: PrefixMappingRef) -> GraphRef {
        Rc::new(RefCell::new(SimpleGraph {
            statements: Default::default(),
            mappings,
        }))
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for SimpleGraph {
    fn supports_feature(&self, feature: &IRIRef) -> bool {
        feature == FEATURE_GRAPH_DUPLICATES.deref() || feature == FEATURE_RDF_STAR.deref()
    }
}

impl Graph for SimpleGraph {
    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn contains_subject(&self, subject: &SubjectNodeRef) -> bool {
        self.statements.iter().any(|st| st.subject() == subject)
    }

    fn contains_individual(&self, subject: &IRIRef) -> bool {
        let factory = self.statement_factory();
        let subject = factory.named_subject(subject.clone());
        self.contains_subject(&subject)
    }

    fn matches(
        &self,
        subject: Option<&SubjectNodeRef>,
        predicate: Option<&IRIRef>,
        object: Option<&ObjectNodeRef>,
    ) -> HashSet<&StatementRef> {
        self.statements
            .iter()
            .filter(|st| {
                let st = st;
                (subject.is_some() && st.subject() == subject.unwrap())
                    && (predicate.is_some() && st.predicate() == predicate.unwrap())
                    && (object.is_some() && st.object() == object.unwrap())
            })
            .collect()
    }

    fn statements<'a>(&'a self) -> Box<dyn Iterator<Item = &'a StatementRef> + 'a> {
        Box::new(self.statements.iter())
    }

    fn subjects(&self) -> HashSet<&SubjectNodeRef> {
        self.statements.iter().map(|st| st.subject()).collect()
    }

    fn predicates(&self) -> HashSet<&IRIRef> {
        self.statements.iter().map(|st| st.predicate()).collect()
    }

    fn predicates_for(&self, subject: &SubjectNodeRef) -> HashSet<&IRIRef> {
        self.statements
            .iter()
            .filter_map(|st| {
                let st = st;
                if st.subject() == subject {
                    Some(st.predicate())
                } else {
                    None
                }
            })
            .collect()
    }

    fn objects(&self) -> HashSet<&ObjectNodeRef> {
        self.statements.iter().map(|st| st.object()).collect()
    }

    fn objects_for(&self, subject: &SubjectNodeRef, predicate: &IRIRef) -> HashSet<&ObjectNodeRef> {
        self.statements
            .iter()
            .filter_map(|st| {
                let st = st;
                if st.subject() == subject && st.predicate() == predicate {
                    Some(st.object())
                } else {
                    None
                }
            })
            .collect()
    }

    fn prefix_mappings(&self) -> PrefixMappingRef {
        self.mappings.clone()
    }

    fn set_prefix_mappings(&mut self, mappings: PrefixMappingRef) {
        self.mappings = mappings;
    }

    fn factory(&self) -> GraphFactoryRef {
        graph_factory()
    }

    fn statement_factory(&self) -> StatementFactoryRef {
        statement_factory()
    }

    fn literal_factory(&self) -> LiteralFactoryRef {
        literal_factory()
    }

    fn statements_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut StatementRef> + 'a> {
        Box::new(self.statements.iter_mut())
    }

    fn insert(&mut self, statement: StatementRef) {
        self.statements.push(statement);
    }

    fn merge(&mut self, other: &Self) {
        for st in other.statements() {
            self.statements.push(st.clone())
        }
    }

    fn dedup(&mut self) -> StatementList {
        let mut discarded: StatementList = Default::default();
        let mut kept: HashSet<StatementRef> = Default::default();
        for st in &self.statements {
            if kept.contains(st) {
                discarded.push(st.clone());
            } else {
                let _ = kept.insert(st.clone());
            }
        }
        self.statements = kept.drain().collect();
        discarded
    }

    fn remove(&mut self, statement: &StatementRef) {
        for (idx, st) in self.statements.iter().enumerate() {
            if st == statement {
                let _ = self.statements.remove(idx);
                break;
            }
        }
    }

    fn remove_all_for(&mut self, subject: &SubjectNodeRef) -> StatementList {
        let mut discarded: StatementList = Default::default();
        let mut kept: StatementList = Default::default();
        for st in &self.statements {
            if st.subject() == subject {
                kept.push(st.clone());
            } else {
                discarded.push(st.clone());
            }
        }
        self.statements = kept;
        discarded
    }

    fn clear(&mut self) {
        self.statements.clear()
    }
}
