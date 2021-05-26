/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

# Example

```rust
use rdftk_core::GraphRef;
use rdftk_core::data_set::{DataSet, DataSetRef};
use rdftk_core::statement::StatementRef;

fn simple_dataset_writer(data_set: &DataSetRef)
{
    let data_set = data_set.borrow();
    if let Some(graph) = data_set.default_graph() {
        println!("{{");
        simple_graph_writer(graph);
        println!("}}");
    }
    for (name, graph) in data_set.graphs() {
        println!("{} {{", name);
        simple_graph_writer(graph);
        println!("}}");
    }
}

fn simple_graph_writer(graph: &GraphRef)
{
    for statement in graph.statements() {
        println!("    {}", statement);
    }
}
```

*/

use crate::graph::GraphRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This enumeration describes the combination of quads that may be indexed by the `DataSet`. The
/// indexes actually supported by a data set implementation can be retrieved using
/// `DataSet::has_index` or `DataSet::has_indices`.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataSetIndex {
    #[allow(missing_docs)]
    Graph,
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
    #[allow(missing_docs)]
    SubjectGraph,
    #[allow(missing_docs)]
    PredicateGraph,
    #[allow(missing_docs)]
    ObjectGraph,
    #[allow(missing_docs)]
    SubjectPredicateGraph,
    #[allow(missing_docs)]
    SubjectPredicateObjectGraph,
    #[allow(missing_docs)]
    SubjectObjectGraph,
    #[allow(missing_docs)]
    PredicateObjectGraph,
}

///
/// A data set factory provides an interface to create a new data set. This allows for
/// implementations where underlying shared resources are required and so may be owned by the
/// factory.
///
pub trait DataSetFactory {
    ///
    /// Create a new graph instance.
    ///
    fn new_data_set(&self, default_graph: Option<GraphRef>) -> DataSetRef;

    ///
    ///  Create a new graph instance from the given statements and prefix mappings.
    ///
    fn data_set_from(
        &self,
        default_graph: Option<GraphRef>,
        graphs: HashMap<GraphNameRef, GraphRef>,
    ) -> DataSetRef {
        let data_set = self.new_data_set(default_graph);
        {
            let mut data_set = data_set.borrow_mut();
            for (name, graph) in graphs {
                data_set.insert(name, graph);
            }
        }
        data_set
    }
}

///
/// The reference type for a graph factory returned by a graph.
///
pub type DataSetFactoryRef = Arc<dyn DataSetFactory>;

///
/// The reference type for a graph data set.
///
pub type DataSetRef = Rc<RefCell<dyn DataSet>>;

///
/// A `DataSet` is a mapping from `GraphName` to `Graph`; this introduces the notion of a named graph
/// although in actuality the graph itself is not named as the name is the key within the data set.
/// Note that this trait represents an immutable data set, a type should also implement the
/// `MutableDataSet` trait for mutation.
///
pub trait DataSet {
    ///
    /// Returns `true` if there are no graphs in this data set, else `false`.
    ///
    fn is_empty(&self) -> bool;

    ///
    /// Return the number of graphs in this data set.
    ///
    fn len(&self) -> usize;

    ///
    /// Return `true` if this data set has a default graph, else `false`.
    ///
    fn has_default_graph(&self) -> bool;

    ///
    /// Return the default graph for this data set, if it exists.
    ///
    fn default_graph(&self) -> Option<&GraphRef>;

    ///
    /// Return `true` if this data set has a graph with the provided name, else `false`.
    ///
    fn has_graph_named(&self, name: &GraphNameRef) -> bool;

    ///
    /// Return the graph with the provided name from this data set, if it exists.
    ///
    fn graph_named(&self, name: &GraphNameRef) -> Option<&GraphRef>;

    ///
    /// Return an iterator over graph name/graph pairs.
    ///
    fn graphs<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a GraphNameRef, &'a GraphRef)> + 'a>;

    ///
    /// Returns `true` if this data set has an index of the specified kind, else `false`.
    ///
    fn has_index(&self, index: &DataSetIndex) -> bool;

    ///
    /// Returns `true` if this data set has **all* the specified index kinds, else `false`.
    ///
    fn has_indices(&self, indices: &[DataSetIndex]) -> bool {
        indices.iter().all(|i| self.has_index(i))
    }

    ///
    /// Set the provided graph as the default, unnamed graph, for this data set. Only one graph may
    /// be the default.
    ///
    fn set_default_graph(&mut self, graph: GraphRef);

    ///
    /// Remove any graph that may be set as the current default. This operation has no effect if
    /// no default graph is present.
    fn unset_default_graph(&mut self);

    ///
    /// Insert a new graph with it's associated name into the data set.
    ///
    fn insert(&mut self, name: GraphNameRef, graph: GraphRef);

    ///
    /// Remove the graph with the provided name from this data set. This operation has no effect if
    /// no such graph is present.
    ///
    fn remove(&mut self, name: &GraphNameRef);

    ///
    /// Remove all graphs from this data set.
    ///
    fn clear(&mut self);

    ///
    /// Return the factory that creates data sets of this kind.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn factory(&self) -> DataSetFactoryRef;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for DataSetIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataSetIndex::Subject => "S",
                DataSetIndex::Predicate => "P",
                DataSetIndex::Object => "O",
                DataSetIndex::SubjectPredicate => "SP",
                DataSetIndex::SubjectPredicateObject => "SPO",
                DataSetIndex::SubjectObject => "SO",
                DataSetIndex::PredicateObject => "PO",
                DataSetIndex::Graph => "G",
                DataSetIndex::SubjectGraph => "SG",
                DataSetIndex::PredicateGraph => "PG",
                DataSetIndex::ObjectGraph => "OG",
                DataSetIndex::SubjectPredicateGraph => "SPG",
                DataSetIndex::SubjectPredicateObjectGraph => "SPOG",
                DataSetIndex::SubjectObjectGraph => "SGO",
                DataSetIndex::PredicateObjectGraph => "POG",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod name;
pub use name::{GraphName, GraphNameRef};
