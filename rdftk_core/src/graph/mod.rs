/*!
Traits which describe the core capabilities of a graph. Note that this crate does not provide an
implementation of these traits as they are very dependent on their usage for performance, and
any backing storage.

# Example

```rust
use rdftk_core::Graph;

fn simple_graph_writer<'a>(graph: &'a impl Graph<'a>) {
    for statement in graph.statements() {
        println!("{}", statement);
    }
}
```
*/

use crate::statement::{ObjectNodeRef, StatementRef, SubjectNodeRef};
use rdftk_iri::IRIRef;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This enumeration describes the combination of triples that may be indexed by the `Graph`. The
/// indexes actually supported by a graph implementation can be retrieved using
/// `Graph::has_index` or `Graph::has_indices`.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GraphIndex {
    #[allow(missing_docs)]
    Subject,
    #[allow(missing_docs)]
    Predicate,
    #[allow(missing_docs)]
    Object,
    #[allow(missing_docs)]
    SubjectPredicate,
    #[allow(missing_docs)]
    SubjectPredicateObject,
    #[allow(missing_docs)]
    SubjectObject,
    #[allow(missing_docs)]
    PredicateObject,
}

///
/// A graph is an unordered list of statements and may include duplicates.
/// Note that this trait represents an immutable graph, a type should also implement the
/// `MutableGraph` trait for mutation.
///
pub trait Graph<'a> {
    ///
    /// The type used to return an iterator over the statements in this graph.
    ///
    type StatementIter: Iterator<Item = &'a StatementRef>;

    ///
    /// The type used to return an filtered iterator over the statements in this graph.
    ///
    type FilteredIter: Iterator<Item = &'a StatementRef>;

    ///
    /// Returns `true` if there are no statements in this graph, else `false`.
    ///
    fn is_empty(&self) -> bool;

    ///
    /// Return the number of statements in this graph.
    ///
    fn len(&self) -> usize;

    ///
    /// Returns `true` if this graph contains any statement with the provided subject, else `false`.
    ///
    fn contains_subject(&self, subject: &SubjectNodeRef) -> bool;

    ///
    /// Returns `true` if this graph contains any statement with the provided IRI as subject, else
    /// `false`.
    ///
    fn contains_individual(&self, subject: &IRIRef) -> bool;

    ///
    /// Returns `true` if this graph contains the provided statement, else `false`.
    ///
    fn contains(&self, statement: &StatementRef) -> bool;

    ///
    /// Returns `true` if this graph contains the any statement with the provided subject,
    /// predicate, and object, else `false`.
    ///
    fn contains_all(
        &self,
        subject: &SubjectNodeRef,
        predicate: &IRIRef,
        object: &ObjectNodeRef,
    ) -> bool;

    ///
    /// Return an iterator over all the statements in the graph.
    ///
    fn statements(&'a self) -> Self::StatementIter;

    ///
    /// Return an iterator over the statements in the graph that match the provided predicate.
    ///
    fn filter(&'a self, predicate: fn(&&StatementRef) -> bool) -> Self::FilteredIter;

    ///
    /// Return a set of all subjects in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    fn subjects(&self) -> HashSet<&SubjectNodeRef>;

    ///
    /// Return a set of all predicate in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    fn predicates(&self) -> HashSet<&IRIRef>;

    ///
    /// Return a set of all predicate referenced by the provided subject in the graph, note that
    /// this is a set so that it removes duplicates.
    ///
    fn predicates_for(&self, subject: &SubjectNodeRef) -> HashSet<&IRIRef>;

    ///
    /// Return a set of all objects in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    fn objects(&self) -> HashSet<&ObjectNodeRef>;

    ///
    /// Return a set of all objects referenced by the provided subject and predicate in the graph,
    /// note that this is a set so that it removes duplicates.
    ///
    fn objects_for(&self, subject: &SubjectNodeRef, predicate: &IRIRef) -> HashSet<&ObjectNodeRef>;

    ///
    /// Returns `true` if this graph has an index of the specified kind, else `false`.
    ///
    fn has_index(&self, index: &GraphIndex) -> bool;

    ///
    /// Returns `true` if this graph has **all* the specified index kinds, else `false`.
    ///
    fn has_indices(&self, indices: &[GraphIndex]) -> bool {
        indices.iter().all(|i| self.has_index(i))
    }

    ///
    /// Returns the set of prefix mappings held by the graph.
    ///
    fn prefix_mappings(&self) -> Rc<dyn PrefixMappings>;

    ///
    /// Returns the value factory that is associated with this graph, if present.
    ///
    fn value_factory(&self) -> Option<Rc<dyn ValueFactory>> {
        None
    }
}

///
/// This trait provides the set of common mutation operations on a graph.
///
pub trait MutableGraph<'a>: Graph<'a> {
    ///
    /// Insert a new statement into the graph.
    ///
    fn insert(&mut self, statement: StatementRef);

    ///
    /// Merge another graph into this one. Note that the graphs are required to have the same
    /// implementation type based in the type qualifiers for `StatementIter` and `FilteredIter`.
    ///
    fn merge(
        &mut self,
        other: &'a Rc<
            dyn Graph<'a, StatementIter = Self::StatementIter, FilteredIter = Self::FilteredIter>,
        >,
    );

    ///
    /// Remove any duplicates within the graph, replacing any number of identical statements with
    /// just one.
    ///
    fn dedup(&mut self);

    ///
    /// Remove any statement that matches the provided. If a graph has duplicates this method does
    /// not differentiate between them.
    ///
    fn remove(&mut self, statement: &StatementRef);

    ///
    /// Remove all statements from this graph that have the provided subject.
    ///
    fn remove_all_for(&mut self, subject: &SubjectNodeRef);

    ///
    /// Remove all statements from this graph.
    ///
    fn clear(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for GraphIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GraphIndex::Subject => "S",
                GraphIndex::Predicate => "P",
                GraphIndex::Object => "O",
                GraphIndex::SubjectPredicate => "SP",
                GraphIndex::SubjectPredicateObject => "SPO",
                GraphIndex::SubjectObject => "SO",
                GraphIndex::PredicateObject => "PO",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod caching;
pub use caching::ValueFactory;

pub mod mapping;
pub use mapping::{Prefix, PrefixMappings};
