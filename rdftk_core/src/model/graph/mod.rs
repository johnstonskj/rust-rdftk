/*!
Traits which describe the core capabilities of a graph. Note that this crate does not provide an
implementation of these traits as they are very dependent on their usage for performance, and
any backing storage.

# Example

```rust
use rdftk_core::model::graph::Graph;
use rdftk_core::model::statement::StatementRef;

fn simple_graph_writer(graph: &impl Graph)
{
    for statement in graph.statements() {
        println!("{}", statement);
    }
}
```
*/

use crate::model::features::Featured;
use crate::model::graph::mapping::PrefixMappingFactoryRef;
use crate::model::literal::LiteralFactoryRef;
use crate::model::statement::{
    ObjectNodeRef, StatementFactoryRef, StatementList, StatementRef, SubjectNodeRef,
};
use crate::model::Provided;
pub use mapping::{PrefixMappingRef, PrefixMappings};
use rdftk_iri::IRIRef;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A graph factory provides an interface to create a new graph. This allows for implementations
/// where underlying shared resources are required and so may be owned by the factory.
///
/// The method for getting the initial factory instance is not specified here. By convention
/// implementors *may* provide a function `graph_factory` in the root module for their crate.
///
pub trait GraphFactory: Debug + Provided {
    ///
    /// Create a new graph instance.
    ///
    fn graph(&self) -> GraphRef;

    ///
    /// Retrieve a prefix-mapping factory.
    ///
    fn mapping_factory(&self) -> PrefixMappingFactoryRef;

    ///
    /// Create a new graph instance with the provided namespace mappings.
    ///
    fn with_mappings(&self, prefix_mappings: PrefixMappingRef) -> GraphRef;

    ///
    ///  Create a new graph instance from the given statements and prefix mappings.
    ///
    fn graph_from(
        &self,
        statements: &[StatementRef],
        prefix_mappings: Option<PrefixMappingRef>,
    ) -> GraphRef {
        let graph = self.graph();
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
/// The actual object storage type, reference counted for memory management.
///
pub type GraphFactoryRef = Arc<dyn GraphFactory>;

// ------------------------------------------------------------------------------------------------

///
/// A graph is an unordered list of statements and may include duplicates.
/// Note that this trait represents an immutable graph, a type should also implement the
/// `MutableGraph` trait for mutation.
///
pub trait Graph: Debug + Featured {
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
    /// Return a set of all subjects that are not blank nodes
    ///
    fn node_subjects(&self) -> HashSet<&SubjectNodeRef> {
        self.subjects().into_iter().filter(|s| ! s.is_blank()).collect()
    }

    ///
    /// Return a set of all subjects that are blank nodes
    ///
    fn blank_node_subjects(&self) -> HashSet<&SubjectNodeRef> {
        self.subjects().into_iter().filter(|s| s.is_blank()).collect()
    }

    ///
    /// Return a set of all predicate in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    fn predicates(&self) -> HashSet<&IRIRef>;

    ///
    /// Return a set of all predicate referenced by the provided subject in graph, note that
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
    // Namespace Management
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns the set of prefix mappings held by the graph.
    ///
    fn prefix_mappings(&self) -> PrefixMappingRef;

    ///
    /// Set the prefix mappings held by the graph.
    ///
    fn set_prefix_mappings(&mut self, mappings: PrefixMappingRef);

    // --------------------------------------------------------------------------------------------
    // Factories
    // --------------------------------------------------------------------------------------------

    ///
    /// Return the factory that creates graphs using the same provider as `self`.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn factory(&self) -> GraphFactoryRef;

    ///
    /// Return the factory that creates statements using the same provider as `self`.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn statement_factory(&self) -> StatementFactoryRef;

    ///
    /// Return the factory that creates literals using the same provider as `self`.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn literal_factory(&self) -> LiteralFactoryRef;

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
    /// just one. This will return a list of all statements removed.
    ///
    /// This method does nothing if this graph has does not support the feature
    /// `FEATURE_GRAPH_DUPLICATES` and will therefore always return an empty list.
    ///
    fn dedup(&mut self) -> StatementList;

    ///
    /// Remove any statement that matches the provided. If a graph has duplicates this method does
    /// not differentiate between them.
    ///
    fn remove(&mut self, statement: &StatementRef);

    ///
    /// Remove all statements from this graph that have the provided subject.
    ///
    fn remove_all_for(&mut self, subject: &SubjectNodeRef) -> StatementList;

    ///
    /// Remove all statements from this graph.
    ///
    fn clear(&mut self);
}

///
/// The actual object storage type, reference counted for memory management.
///
pub type GraphRef = Rc<RefCell<dyn Graph>>;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod mapping;

pub mod skolem;
