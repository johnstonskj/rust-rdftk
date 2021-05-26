/*!
Traits which describe the core capabilities of a graph. Note that this crate does not provide an
implementation of these traits as they are very dependent on their usage for performance, and
any backing storage.

# Example

```rust
use rdftk_core::Graph;
use rdftk_core::statement::StatementRef;

fn simple_graph_writer(graph: &impl Graph)
{
    for statement in graph.statements() {
        println!("{}", statement);
    }
}
```
*/

use crate::statement::{ObjectNodeRef, StatementRef, SubjectNodeRef};
use rdftk_iri::IRIRef;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;

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
/// A graph factory provides an interface to create a new graph. This allows for implementations
/// where underlying shared resources are required and so may be owned by the factory.
///
pub trait GraphFactory {
    ///
    /// Create a new graph instance.
    ///
    fn new_graph(&self) -> GraphRef;

    ///
    ///  Create a new graph instance from the given statements and prefix mappings.
    ///
    fn graph_from(
        &self,
        statements: &[StatementRef],
        prefix_mappings: Option<PrefixMappingRef>,
    ) -> GraphRef {
        let graph = self.new_graph();
        {
            let mut graph = graph.borrow_mut();
            for st in statements {
                graph.insert(st.clone());
            }
            if let Some(prefix_mappings) = prefix_mappings {
                graph.set_prefix_mappings(prefix_mappings)
            }
        }
        graph
    }
}

///
/// The reference type for a graph factory returned by a graph.
///
pub type GraphFactoryRef = Arc<dyn GraphFactory>;

///
/// The reference type for a graph returned by a graph factory.
///
pub type GraphRef = Rc<RefCell<dyn Graph>>;

///
/// A graph is an unordered list of statements and may include duplicates.
/// Note that this trait represents an immutable graph, a type should also implement the
/// `MutableGraph` trait for mutation.
///
pub trait Graph {
    ///
    /// Returns `true` if there are no statements in this graph, else `false`.
    ///
    fn is_empty(&self) -> bool;

    ///
    /// Return the number of statements in this graph.
    ///
    fn len(&self) -> usize;

    // --------------------------------------------------------------------------------------------
    // Query
    // --------------------------------------------------------------------------------------------

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
    fn contains(&self, statement: &StatementRef) -> bool {
        !self
            .matches(
                Some(statement.subject()),
                Some(statement.predicate()),
                Some(statement.object()),
            )
            .is_empty()
    }

    ///
    /// Returns `true` if this graph contains the any statement with the provided subject,
    /// predicate, and object, else `false`.
    ///
    fn contains_all(
        &self,
        subject: &SubjectNodeRef,
        predicate: &IRIRef,
        object: &ObjectNodeRef,
    ) -> bool {
        !self
            .matches(Some(subject), Some(predicate), Some(object))
            .is_empty()
    }

    ///
    /// Returns `true` if this graph contains the any statement with the provided subject,
    /// predicate, and object, else `false`.
    ///
    fn matches(
        &self,
        subject: Option<&SubjectNodeRef>,
        predicate: Option<&IRIRef>,
        object: Option<&ObjectNodeRef>,
    ) -> HashSet<&StatementRef>;

    // --------------------------------------------------------------------------------------------
    // Iterators
    // --------------------------------------------------------------------------------------------

    ///
    /// Return an iterator over all the statements in the graph.
    ///
    fn statements<'a>(&'a self) -> Box<dyn Iterator<Item = &'a StatementRef> + 'a>;

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

    // --------------------------------------------------------------------------------------------
    // Indexing
    // --------------------------------------------------------------------------------------------

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

    // --------------------------------------------------------------------------------------------
    // Namespace Management
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns the set of prefix mappings held by the graph.
    ///
    fn prefix_mappings(&self) -> Rc<RefCell<PrefixMappings>>;

    ///
    /// Set the prefix mappings held by the graph.
    ///
    fn set_prefix_mappings(&mut self, mappings: Rc<RefCell<PrefixMappings>>);

    // --------------------------------------------------------------------------------------------
    // Factories
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns the value factory that is associated with this graph, if present.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn value_factory(&self) -> Option<Arc<dyn ValueFactory>> {
        None
    }

    ///
    /// Return the factory that creates graphs of this kind.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn factory(&self) -> GraphFactoryRef;

    // --------------------------------------------------------------------------------------------
    // Mutators
    // --------------------------------------------------------------------------------------------

    ///
    /// Return an iterator over all the statements in the graph.
    ///
    fn statements_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut StatementRef> + 'a>;

    ///
    /// Insert a new statement into the graph.
    ///
    fn insert(&mut self, statement: StatementRef);

    ///
    /// Merge another graph into this one. Note that the graphs are required to have the same
    /// implementation type based in the type qualifiers for `StatementIter`.
    ///
    fn merge(&mut self, other: &Self)
    where
        Self: Sized;

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
pub use mapping::{Prefix, PrefixMappingRef, PrefixMappings};

pub mod skolem;
