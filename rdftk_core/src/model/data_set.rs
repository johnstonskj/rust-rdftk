/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

The [`DataSet`] type provides a mapping from `Option<GraphName>` to `Graph`.

# Example

```rust
use rdftk_core::model::data_set::DataSet;
use rdftk_core::model::graph::Graph;
use rdftk_core::model::statement::Statement;

fn simple_dataset_writer(data_set: &DataSet)
{
    if let Some(graph) = data_set.default_graph() {
        simple_graph_writer(graph);
    }
    for graph in data_set.graphs() {
        simple_graph_writer(graph);
    }
}

fn simple_graph_writer(graph: &Graph)
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
use rdftk_iri::Iri;
use std::collections::HashMap;
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
#[derive(Clone, Debug, Default)]
pub struct DataSet {
    graphs: HashMap<Option<GraphName>, Graph>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Graph> for DataSet {
    fn from(graphs: Graph) -> Self {
        Self::from_iter([graphs])
    }
}

impl From<Vec<Graph>> for DataSet {
    fn from(graphs: Vec<Graph>) -> Self {
        Self::from_iter(graphs)
    }
}

impl From<HashMap<Option<GraphName>, Graph>> for DataSet {
    fn from(graphs: HashMap<Option<GraphName>, Graph>) -> Self {
        Self { graphs }
    }
}

impl FromIterator<Graph> for DataSet {
    fn from_iter<T: IntoIterator<Item = Graph>>(iter: T) -> Self {
        Self::from(
            iter.into_iter()
                .map(|g| (g.name().cloned(), g))
                .collect::<HashMap<Option<GraphName>, Graph>>(),
        )
    }
}

impl Featured for DataSet {
    fn supports_feature(&self, _feature: &Iri) -> bool {
        false
    }
}

impl DataSet {
    ///
    /// Returns `true` if there are no graphs in this data set, else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.graphs.is_empty()
    }

    ///
    /// Return the number of graphs in this data set.
    ///
    pub fn len(&self) -> usize {
        self.graphs.len()
    }

    // --------------------------------------------------------------------------------------------
    // Access > Query
    // --------------------------------------------------------------------------------------------

    pub fn contains_graph(&self, name: &Option<GraphName>) -> bool {
        self.graphs.contains_key(name)
    }

    ///
    /// Return `true` if this data set has a default graph, else `false`.
    ///
    pub fn has_default_graph(&self) -> bool {
        self.contains_graph(&None)
    }

    ///
    /// Return `true` if this data set has a graph with the provided name, else `false`.
    ///
    pub fn has_graph_named(&self, name: &GraphName) -> bool {
        self.contains_graph(&Some(name.clone()))
    }

    // --------------------------------------------------------------------------------------------
    // Access > Graphs
    // --------------------------------------------------------------------------------------------

    pub fn graph(&self, name: &Option<GraphName>) -> Option<&Graph> {
        self.graphs.get(name)
    }

    pub fn graph_mut(&mut self, name: &Option<GraphName>) -> Option<&mut Graph> {
        self.graphs.get_mut(name)
    }

    ///
    /// Return the default graph for this data set, if it exists.
    ///
    pub fn default_graph(&self) -> Option<&Graph> {
        self.graph(&None)
    }

    ///
    /// Return a reference to the graph with the provided name from this data set, if it exists.
    ///
    pub fn graph_named(&self, name: &GraphName) -> Option<&Graph> {
        self.graph(&Some(name.clone()))
    }

    ///
    /// Return an iterator over all graphs.
    ///
    pub fn graphs(&self) -> impl Iterator<Item = &Graph> {
        self.graphs.values()
    }

    // --------------------------------------------------------------------------------------------
    // Mutators
    // --------------------------------------------------------------------------------------------

    ///
    /// Insert a new graph with it's associated name into the data set.
    ///
    pub fn insert(&mut self, graph: Graph) {
        let graph_name = graph.name().cloned();
        let _ = self.graphs.insert(graph_name, graph);
    }

    ///
    /// Add all the graphs from the provided vector.
    ///
    pub fn extend(&mut self, graphs: Vec<Graph>) {
        graphs.into_iter().for_each(|g| self.insert(g))
    }

    ///
    /// Remove the graph with the provided name from this data set. This operation has no effect if
    /// no such graph is present.
    ///
    pub fn remove(&mut self, name: &Option<GraphName>) {
        let _ = self.graphs.remove(name);
    }

    ///
    /// Remove all graphs from this data set.
    ///
    pub fn clear(&mut self) {
        self.graphs.clear();
    }
}
