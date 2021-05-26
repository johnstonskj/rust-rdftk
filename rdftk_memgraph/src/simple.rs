/*!
TBD
*/

use rdftk_core::graph::mapping::PrefixMappingRef;
use rdftk_core::graph::{
    Graph, GraphFactory, GraphFactoryRef, GraphIndex, GraphRef, PrefixMappings,
};
use rdftk_core::statement::{ObjectNodeRef, StatementList, StatementRef, SubjectNodeRef};
use rdftk_core::SubjectNode;
use rdftk_iri::IRIRef;
use rdftk_names::rdf;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A factory creating `MemGraph` instances.
///
#[derive(Clone, Debug)]
pub struct MemGraphFactory {}

///
/// A very simple in-memory implementation of the `Graph` and `NamedGraph` traits.
///
#[derive(Clone, Debug)]
pub struct MemGraph {
    statements: StatementList,
    mappings: PrefixMappingRef,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Retrieve the graph factory for simple `MemGraph` instances.
///
pub fn graph_factory() -> GraphFactoryRef {
    FACTORY.clone()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref FACTORY: Arc<MemGraphFactory> = Arc::new(MemGraphFactory::default());
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for MemGraphFactory {
    fn default() -> Self {
        Self {}
    }
}

impl GraphFactory for MemGraphFactory {
    fn new_graph(&self) -> GraphRef {
        Rc::new(RefCell::new(MemGraph {
            statements: Default::default(),
            mappings: Rc::new(RefCell::new(PrefixMappings::default())),
        }))
    }
}

// ------------------------------------------------------------------------------------------------

impl Graph for MemGraph {
    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn contains_subject(&self, subject: &SubjectNodeRef) -> bool {
        self.statements
            .iter()
            .any(|st| st.as_ref().subject() == subject)
    }

    fn contains_individual(&self, subject: &IRIRef) -> bool {
        let subject: SubjectNodeRef = SubjectNode::named(subject.clone()).into();
        self.objects_for(&subject, rdf::a_type()).is_empty()
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
                if st.subject() == subject && st.predicate() == predicate {
                    Some(st.object())
                } else {
                    None
                }
            })
            .collect()
    }

    fn has_index(&self, _: &GraphIndex) -> bool {
        false
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
        self.statements.push(statement);
    }

    fn merge(&mut self, other: &Self) {
        for st in other.statements() {
            self.statements.push(st.clone())
        }
    }

    fn dedup(&mut self) {
        let mut sts: HashSet<StatementRef> = self.statements.drain(..).collect();
        self.statements = sts.drain().collect()
    }

    fn remove(&mut self, statement: &StatementRef) {
        self.statements.retain(|st| st != statement);
    }

    fn remove_all_for(&mut self, subject: &SubjectNodeRef) {
        self.statements.retain(|st| st.subject() != subject);
    }

    fn clear(&mut self) {
        self.statements.clear()
    }
}

impl<'a> MemGraph {
    ///
    /// Construct a new `MemGraph` from the list of statements.
    ///
    pub fn with(&mut self, statements: StatementList) -> &mut Self {
        self.statements = statements;
        self
    }
}
