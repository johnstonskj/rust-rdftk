/*!
One-line description.

More detailed description, with

# Example

*/

use rdftk_core::graph::mapping::PrefixMappingRef;
use rdftk_core::graph::{
    Graph, GraphFactory, GraphFactoryRef, GraphIndex, GraphRef, PrefixMappings,
};
use rdftk_core::statement::{
    ObjectNodeRef, StatementList, StatementRef, SubjectNode, SubjectNodeRef,
};
use rdftk_iri::IRIRef;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A factory creating IndexedMemGraph instances.
///
#[derive(Clone, Debug)]
pub struct IndexedMemGraphFactory {}

///
/// An indexed in-memory implementation of the `Graph` and `NamedGraph` traits.
///
#[derive(Clone, Debug)]
pub struct IndexedMemGraph {
    statements: StatementList,
    mappings: PrefixMappingRef,
    s_index: HashMap<SubjectNodeRef, StatementList>,
    p_index: HashMap<IRIRef, StatementList>,
    o_index: HashMap<ObjectNodeRef, StatementList>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref FACTORY: Arc<IndexedMemGraphFactory> = Arc::new(IndexedMemGraphFactory {});
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl GraphFactory for IndexedMemGraphFactory {
    fn new_graph(&self) -> GraphRef {
        Rc::new(RefCell::new(IndexedMemGraph {
            statements: Default::default(),
            mappings: Rc::new(RefCell::new(PrefixMappings::default())),
            s_index: Default::default(),
            p_index: Default::default(),
            o_index: Default::default(),
        }))
    }
}

// ------------------------------------------------------------------------------------------------

impl Graph for IndexedMemGraph {
    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn contains_subject(&self, subject: &SubjectNodeRef) -> bool {
        self.s_index.contains_key(subject)
    }

    fn contains_individual(&self, subject: &IRIRef) -> bool {
        self.s_index
            .contains_key(&SubjectNodeRef::from(SubjectNode::from(subject.clone())))
    }

    fn matches(
        &self,
        subject: Option<&SubjectNodeRef>,
        predicate: Option<&IRIRef>,
        object: Option<&ObjectNodeRef>,
    ) -> HashSet<&StatementRef> {
        let s_sts: HashSet<&StatementRef> = if let Some(subject) = subject {
            if let Some(sts) = self.s_index.get(subject) {
                HashSet::from_iter(sts)
            } else {
                Default::default()
            }
        } else {
            Default::default()
        };
        let p_sts: HashSet<&StatementRef> = if let Some(predicate) = predicate {
            if let Some(sts) = self.p_index.get(predicate) {
                HashSet::from_iter(sts)
            } else {
                Default::default()
            }
        } else {
            Default::default()
        };
        let o_sts: HashSet<&StatementRef> = if let Some(object) = object {
            if let Some(sts) = self.o_index.get(object) {
                HashSet::from_iter(sts)
            } else {
                Default::default()
            }
        } else {
            Default::default()
        };
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

    fn predicates(&self) -> HashSet<&IRIRef> {
        self.p_index.keys().collect()
    }

    fn predicates_for(&self, subject: &SubjectNodeRef) -> HashSet<&IRIRef> {
        if let Some(sts) = self.s_index.get(subject) {
            sts.iter().map(|st| st.predicate()).collect()
        } else {
            Default::default()
        }
    }

    fn objects(&self) -> HashSet<&ObjectNodeRef> {
        self.o_index.keys().collect()
    }

    fn objects_for(
        &self,
        _subject: &SubjectNodeRef,
        _predicate: &IRIRef,
    ) -> HashSet<&ObjectNodeRef> {
        todo!()
    }

    fn has_index(&self, index: &GraphIndex) -> bool {
        matches!(
            index,
            GraphIndex::Subject | GraphIndex::Predicate | GraphIndex::Object
        )
    }

    fn prefix_mappings(&self) -> PrefixMappingRef {
        self.mappings.clone()
    }

    fn set_prefix_mappings(&mut self, mappings: PrefixMappingRef) {
        self.mappings = mappings;
    }

    fn factory(&self) -> GraphFactoryRef {
        FACTORY.clone()
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
        self.statements.push(statement);
    }

    fn merge(&mut self, other: &Self) {
        for st in other.statements() {
            self.insert(st.clone());
        }
    }

    fn dedup(&mut self) {
        todo!()
    }

    fn remove(&mut self, _statement: &StatementRef) {
        todo!()
    }

    fn remove_all_for(&mut self, subject: &SubjectNodeRef) {
        if let Some(sts) = self.s_index.remove(subject) {
            for st in sts {
                self.remove(&st);
            }
        }
    }

    fn clear(&mut self) {
        self.statements.clear();
        self.s_index.clear();
        self.p_index.clear();
        self.o_index.clear();
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
