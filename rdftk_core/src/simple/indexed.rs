/*!
Simple, in-memory implementation of the `Graph` and `GraphFactory` traits with support for
subject, predicate, and object, indices.
*/

use crate::model::features::{
    Featured, FEATURE_GRAPH_DUPLICATES, FEATURE_IDX_OBJECT, FEATURE_IDX_PREDICATE,
    FEATURE_IDX_SUBJECT, FEATURE_RDF_STAR,
};
use crate::model::graph::{Graph, GraphFactory, GraphFactoryRef, GraphRef, PrefixMappingRef};
use crate::model::literal::LiteralFactoryRef;
use crate::model::statement::{
    ObjectNodeRef, StatementFactoryRef, StatementList, StatementRef, SubjectNodeRef,
};
use crate::model::Provided;
use crate::simple::empty_mappings;
use crate::simple::literal::literal_factory;
use crate::simple::statement::statement_factory;
use rdftk_iri::IriRef;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use lazy_static::lazy_static;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `Graph` trait.
///
#[derive(Clone, Debug)]
pub struct IndexedSimpleGraph {
    statements: StatementList,
    mappings: PrefixMappingRef,
    s_index: HashMap<SubjectNodeRef, StatementList>,
    p_index: HashMap<IriRef, StatementList>,
    o_index: HashMap<ObjectNodeRef, StatementList>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `GraphFactory` trait.
///
#[derive(Clone, Debug)]
struct IndexedSimpleGraphFactory {}

lazy_static! {
    static ref FACTORY: Arc<IndexedSimpleGraphFactory> =
        Arc::new(IndexedSimpleGraphFactory::default());
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
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for IndexedSimpleGraphFactory {
    fn default() -> Self {
        Self {}
    }
}

impl Provided for IndexedSimpleGraphFactory {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl GraphFactory for IndexedSimpleGraphFactory {
    fn graph(&self) -> GraphRef {
        self.with_mappings(empty_mappings())
    }

    fn with_mappings(&self, prefix_mappings: PrefixMappingRef) -> GraphRef {
        Rc::new(RefCell::new(IndexedSimpleGraph {
            statements: Default::default(),
            mappings: prefix_mappings,
            s_index: Default::default(),
            p_index: Default::default(),
            o_index: Default::default(),
        }))
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for IndexedSimpleGraph {
    fn supports_feature(&self, feature: &IriRef) -> bool {
        feature == FEATURE_GRAPH_DUPLICATES.deref()
            || feature == FEATURE_RDF_STAR.deref()
            || feature == FEATURE_IDX_SUBJECT.deref()
            || feature == FEATURE_IDX_PREDICATE.deref()
            || feature == FEATURE_IDX_OBJECT.deref()
    }
}

impl Graph for IndexedSimpleGraph {
    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn contains_subject(&self, subject: &SubjectNodeRef) -> bool {
        self.s_index.contains_key(subject)
    }

    fn contains_individual(&self, subject: &IriRef) -> bool {
        let factory = self.statement_factory();
        let subject = factory.named_subject(subject.clone());
        self.contains_subject(&subject)
    }

    fn matches(
        &self,
        subject: Option<&SubjectNodeRef>,
        predicate: Option<&IriRef>,
        object: Option<&ObjectNodeRef>,
    ) -> HashSet<&StatementRef> {
        let s_sts: HashSet<&StatementRef> = subject
            .map(|subject| {
                self.s_index
                    .get(subject)
                    .map(|sts| HashSet::from_iter(sts))
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let p_sts: HashSet<&StatementRef> = predicate
            .map(|predicate| {
                self.p_index
                    .get(predicate)
                    .map(|sts| HashSet::from_iter(sts))
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let o_sts: HashSet<&StatementRef> = object
            .map(|object| {
                self.o_index
                    .get(object)
                    .map(|sts| HashSet::from_iter(sts))
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        s_sts
            .intersection(&p_sts)
            .cloned()
            .collect::<HashSet<&StatementRef>>()
            .intersection(&o_sts)
            .cloned()
            .collect::<HashSet<&StatementRef>>()
    }

    fn statements<'a>(&'a self) -> Box<dyn Iterator<Item = &'a StatementRef> + 'a> {
        Box::new(self.statements.iter())
    }

    fn subjects(&self) -> HashSet<&SubjectNodeRef> {
        self.s_index.keys().collect()
    }

    fn predicates(&self) -> HashSet<&IriRef> {
        self.p_index.keys().collect()
    }

    fn predicates_for(&self, subject: &SubjectNodeRef) -> HashSet<&IriRef> {
        self.s_index
            .get(subject)
            .map(|sts| sts.iter().map(|st| st.predicate()).collect())
            .unwrap_or_default()
    }

    fn objects(&self) -> HashSet<&ObjectNodeRef> {
        self.o_index.keys().collect()
    }

    fn objects_for(&self, subject: &SubjectNodeRef, predicate: &IriRef) -> HashSet<&ObjectNodeRef> {
        self.matches(Some(subject), Some(predicate), None)
            .iter()
            .map(|st| st.object())
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
        match self.s_index.get_mut(statement.subject()) {
            None => {
                let _ = self
                    .s_index
                    .insert(statement.subject().clone(), vec![statement.clone()]);
            }
            Some(sts) => {
                sts.push(statement.clone());
            }
        }
        match self.p_index.get_mut(statement.predicate()) {
            None => {
                let _ = self
                    .p_index
                    .insert(statement.predicate().clone(), vec![statement.clone()]);
            }
            Some(sts) => {
                sts.push(statement.clone());
            }
        }
        match self.o_index.get_mut(statement.object()) {
            None => {
                let _ = self
                    .o_index
                    .insert(statement.object().clone(), vec![statement.clone()]);
            }
            Some(sts) => {
                sts.push(statement.clone());
            }
        }
        self.statements.push(statement);
    }

    fn merge(&mut self, other: &Self)
    where
        Self: Sized,
    {
        other.statements().for_each(|st| self.insert(st.clone()))
    }

    fn dedup(&mut self) -> StatementList {
        let (keep, discard) = self.statements.iter().fold(
            (HashSet::<StatementRef>::default(), StatementList::default()),
            |(mut keep, mut discard), st| {
                if keep.contains(st) {
                    (&mut discard).push(st.clone());
                } else {
                    let _ = (&mut keep).insert(st.clone());
                }
                (keep, discard)
            },
        );
        self.statements = keep.into_iter().collect::<StatementList>();
        for st in &discard {
            self.remove_indices_for(st);
        }
        discard
    }

    fn remove(&mut self, statement: &StatementRef) {
        for (idx, st) in self.statements.iter().enumerate() {
            if st == statement {
                let _ = self.statements.remove(idx);
                self.remove_indices_for(statement);
                break;
            }
        }
    }

    fn remove_all_for(&mut self, subject: &SubjectNodeRef) -> StatementList {
        let sts: Option<StatementList> = self.s_index.get(subject).map(|sts| sts.to_vec());
        if let Some(sts) = sts {
            for st in &sts {
                self.remove(st)
            }
            sts
        } else {
            Default::default()
        }
    }

    fn clear(&mut self) {
        self.statements.clear();
        self.s_index.clear();
        self.p_index.clear();
        self.o_index.clear();
    }
}

impl IndexedSimpleGraph {
    fn remove_indices_for(&mut self, statement: &StatementRef) {
        Self::remove_from_index(statement, statement.subject(), &mut self.s_index);
        Self::remove_from_index(statement, statement.predicate(), &mut self.p_index);
        Self::remove_from_index(statement, statement.object(), &mut self.o_index);
    }

    fn remove_from_index<T: Eq + Hash>(
        statement: &StatementRef,
        key: &T,
        index: &mut HashMap<T, StatementList>,
    ) {
        let _ = index.get_mut(key).map(|sts| {
            sts.iter().position(|st| st == statement).map(|idx| {
                let _ = sts.remove(idx);
            })
        });
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
