/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

# Example

```rust
use rdftk_core::model::data_set::DataSet;
use rdftk_core::model::graph::Graph;
use rdftk_core::model::statement::Statement;

fn simple_dataset_writer(data_set: &impl DataSet)
{
    if let Some(graph) = data_set.default_graph() {
        simple_graph_writer(graph);
    }
    for graph in data_set.graphs() {
        simple_graph_writer(graph);
    }
}

fn simple_graph_writer(graph: &impl Graph)
{
    if graph.is_named() {
        println!("{} {{", graph.name().unwrap());
    } else {
        println!("{{");
    }
    for statement in graph.statements() {
        println!("    {:?}", statement);
    }
    println!("}}");
}
```

*/

use crate::model::features::Featured;
use crate::model::graph::{Graph, GraphName};
use crate::model::Provided;
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Data Set
// ------------------------------------------------------------------------------------------------

///
/// A `DataSet` is a mapping from `GraphName` to `Graph`; this introduces the notion of a named graph
/// although in actuality the graph itself is not named as the name is the key within the data set.
/// Note that this trait represents an immutable data set, a type should also implement the
/// `MutableDataSet` trait for mutation.
///
pub trait DataSet: Debug + Featured {
    type Graph: Graph;

    ///
    /// Returns `true` if there are no graphs in this data set, else `false`.
    ///
    fn is_empty(&self) -> bool;

    ///
    /// Return the number of graphs in this data set.
    ///
    fn len(&self) -> usize;

    fn contains_graph(&self, name: &Option<GraphName>) -> bool;

    ///
    /// Return `true` if this data set has a default graph, else `false`.
    ///
    fn has_default_graph(&self) -> bool {
        self.contains_graph(&None)
    }

    ///
    /// Return `true` if this data set has a graph with the provided name, else `false`.
    ///
    fn has_graph_named(&self, name: &GraphName) -> bool {
        self.contains_graph(&Some(name.clone()))
    }

    ///
    /// Return the default graph for this data set, if it exists.
    ///
    fn default_graph(&self) -> Option<&Self::Graph> {
        self.graph(&None)
    }

    ///
    /// Return the graph with the provided name from this data set, if it exists.
    ///
    fn graph_named(&self, name: &GraphName) -> Option<&Self::Graph> {
        self.graph(&Some(name.clone()))
    }

    fn graph(&self, name: &Option<GraphName>) -> Option<&Self::Graph>;

    fn graph_mut(&mut self, name: &Option<GraphName>) -> Option<&mut Self::Graph>;

    ///
    /// Return an iterator over all graphs.
    ///
    fn graphs(&self) -> impl Iterator<Item = &Self::Graph>;

    ///
    /// Insert a new graph with it's associated name into the data set.
    ///
    fn insert(&mut self, graph: Self::Graph);

    ///
    /// Add all the graphs from the provided vector.
    ///
    fn extend(&mut self, graphs: Vec<Self::Graph>);

    ///
    /// Remove the graph with the provided name from this data set. This operation has no effect if
    /// no such graph is present.
    ///
    fn remove(&mut self, named: &Option<GraphName>);

    ///
    /// Remove all graphs from this data set.
    ///
    fn clear(&mut self);
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Factories
// ------------------------------------------------------------------------------------------------

///
/// A data set factory provides an interface to create a new data set. This allows for
/// implementations where underlying shared resources are required and so may be owned by the
/// factory.
///
pub trait DataSetFactory: Debug + Provided {
    type Graph: Graph;
    type DataSet: DataSet<Graph = Self::Graph>;
    ///
    /// Create a new graph instance.
    ///
    fn data_set(&self) -> Self::DataSet;

    ///
    ///  Create a new graph instance from the given statements and prefix mappings.
    ///
    fn data_set_from(&self, graphs: Vec<Self::Graph>) -> Self::DataSet {
        let mut data_set = self.data_set();
        for graph in graphs {
            data_set.insert(graph);
        }
        data_set
    }
}
