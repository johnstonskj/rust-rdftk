/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

# Example

```rust
use rdftk_core::model::data_set::{DataSet, DataSetRef};
use rdftk_core::model::graph::NamedGraphRef;
use rdftk_core::model::statement::StatementRef;

fn simple_dataset_writer(data_set: &DataSetRef)
{
    let data_set = data_set.borrow();
    if let Some(graph) = data_set.default_graph() {
        simple_graph_writer(graph);
    }
    for graph in data_set.graphs() {
        simple_graph_writer(graph);
    }
}

fn simple_graph_writer(graph: &NamedGraphRef)
{
    let graph = graph.borrow();
    if graph.is_named() {
        println!("{} {{", graph.name().unwrap());
    } else {
        println!("{{");
    }
    for statement in graph.statements() {
        println!("    {}", statement);
    }
    println!("}}");
}
```

*/

use crate::model::features::Featured;
use crate::model::graph::named::GraphNameRef;
use crate::model::graph::named::NamedGraphRef;
use crate::model::graph::GraphFactoryRef;
use crate::model::Provided;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A data set factory provides an interface to create a new data set. This allows for
/// implementations where underlying shared resources are required and so may be owned by the
/// factory.
///
pub trait DataSetFactory: Debug + Provided {
    ///
    /// Create a new graph instance.
    ///
    fn data_set(&self) -> DataSetRef;

    ///
    ///  Create a new graph instance from the given statements and prefix mappings.
    ///
    fn data_set_from(&self, graphs: Vec<NamedGraphRef>) -> DataSetRef {
        let data_set = self.data_set();
        {
            let mut data_set = data_set.borrow_mut();
            for graph in graphs {
                data_set.insert(graph);
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
/// A `DataSet` is a mapping from `GraphName` to `Graph`; this introduces the notion of a named graph
/// although in actuality the graph itself is not named as the name is the key within the data set.
/// Note that this trait represents an immutable data set, a type should also implement the
/// `MutableDataSet` trait for mutation.
///
pub trait DataSet: Debug + Featured {
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
    fn has_default_graph(&self) -> bool {
        self.default_graph().is_some()
    }

    ///
    /// Return the default graph for this data set, if it exists.
    ///
    fn default_graph(&self) -> Option<&NamedGraphRef>;

    ///
    /// Return `true` if this data set has a graph with the provided name, else `false`.
    ///
    fn has_graph_named(&self, name: &GraphNameRef) -> bool;

    ///
    /// Return the graph with the provided name from this data set, if it exists.
    ///
    fn graph_named(&self, name: &GraphNameRef) -> Option<&NamedGraphRef>;

    ///
    /// Return an iterator over all graphs.
    ///
    fn graphs(&self) -> Box<dyn Iterator<Item = &NamedGraphRef> + '_>;

    ///
    /// Return an iterator over graph names.
    ///
    fn graph_names(&self) -> Box<dyn Iterator<Item = &GraphNameRef> + '_>;

    ///
    /// Insert a new graph with it's associated name into the data set.
    ///
    fn insert(&mut self, graph: NamedGraphRef);

    ///
    /// Add all the graphs from the provided vector.
    ///
    fn extend(&mut self, graphs: Vec<NamedGraphRef>);

    ///
    /// Remove the graph with the provided name from this data set. This operation has no effect if
    /// no such graph is present.
    ///
    fn remove(&mut self, named: &GraphNameRef);

    ///
    /// Remove the default graph from this data set. This operation has no effect if no default
    /// graph is present.
    ///
    fn remove_default(&mut self);

    ///
    /// Remove all graphs from this data set.
    ///
    fn clear(&mut self);

    ///
    /// Return the factory that creates data sets using the same provider as `self`.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn factory(&self) -> DataSetFactoryRef;

    ///
    /// Return the factory that creates graphs managed by data set's of this kind.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn graph_factory(&self) -> GraphFactoryRef;
}

///
/// The reference type for a graph data set.
///
pub type DataSetRef = Rc<RefCell<dyn DataSet>>;
