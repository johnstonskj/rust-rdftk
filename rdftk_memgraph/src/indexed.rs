/*!
One-line description.

More detailed description, with

# Example

*/

use crate::Mappings;
use rdftk_core::graph::{Graph, GraphIndex, MutableGraph, PrefixMappings};
use rdftk_core::statement::{ObjectNodeRef, StatementList, StatementRef, SubjectNodeRef};
use rdftk_core::SubjectNode;
use rdftk_iri::IRIRef;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An indexed in-memory implementation of the `Graph` and `NamedGraph` traits.
///
#[derive(Clone, Debug)]
pub struct IndexedMemGraph {
    statements: StatementList,
    mappings: Rc<dyn PrefixMappings>,
    s_index: HashMap<SubjectNodeRef, StatementList>,
    p_index: HashMap<IRIRef, StatementList>,
    o_index: HashMap<ObjectNodeRef, StatementList>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for IndexedMemGraph {
    fn default() -> Self {
        Self {
            statements: Default::default(),
            mappings: Rc::new(Mappings::default()),
            s_index: Default::default(),
            p_index: Default::default(),
            o_index: Default::default(),
        }
    }
}

impl<'a> Graph<'a> for IndexedMemGraph {
    type StatementIter = std::slice::Iter<'a, StatementRef>;
    type FilteredIter = std::iter::Filter<Self::StatementIter, fn(&&StatementRef) -> bool>;

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

    fn contains(&self, statement: &StatementRef) -> bool {
        self.contains_all(
            statement.subject(),
            statement.predicate(),
            statement.object(),
        )
    }

    fn contains_all(
        &self,
        _subject: &SubjectNodeRef,
        _predicate: &IRIRef,
        _object: &ObjectNodeRef,
    ) -> bool {
        todo!()
    }

    fn statements(&'a self) -> Self::StatementIter {
        self.statements.iter()
    }

    fn filter(&'a self, filter: fn(&&StatementRef) -> bool) -> Self::FilteredIter {
        self.statements.iter().filter(filter)
    }

    fn subjects(&self) -> HashSet<&SubjectNodeRef> {
        self.s_index.keys().collect()
    }

    fn predicates(&self) -> HashSet<&IRIRef> {
        self.p_index.keys().collect()
    }

    fn predicates_for(&self, _subject: &SubjectNodeRef) -> HashSet<&IRIRef> {
        todo!()
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

    fn has_index(&self, _index: &GraphIndex) -> bool {
        todo!()
    }

    fn prefix_mappings(&self) -> Rc<dyn PrefixMappings> {
        self.mappings.clone()
    }
}

impl<'a> MutableGraph<'a> for IndexedMemGraph {
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

    fn merge(
        &mut self,
        _other: &'a Rc<
            dyn Graph<'a, StatementIter = Self::StatementIter, FilteredIter = Self::FilteredIter>,
        >,
    ) {
        todo!()
    }

    fn dedup(&mut self) {
        todo!()
    }

    fn remove(&mut self, _statement: &StatementRef) {
        todo!()
    }

    fn remove_all_for(&mut self, _subject: &SubjectNodeRef) {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
