/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

# Example

```rust
use rdftk_core::model::data_set::{DataSet, DataSetRef};
use rdftk_core::model::graph::GraphRef;
use rdftk_core::model::statement::StatementRef;

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
    let graph = graph.borrow();
    for statement in graph.statements() {
        println!("    {}", statement);
    }
}
```

*/

use crate::model::features::Featured;
use crate::model::graph::{GraphFactoryRef, GraphRef};
use crate::model::Provided;
use std::cell::RefCell;
use std::collections::HashMap;
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
    /// Create a new model.graph instance.
    ///
    fn data_set(&self, default_graph: Option<GraphRef>) -> DataSetRef;

    ///
    ///  Create a new model.graph instance from the given statements and prefix mappings.
    ///
    fn data_set_from(
        &self,
        default_graph: Option<GraphRef>,
        graphs: HashMap<GraphNameRef, GraphRef>,
    ) -> DataSetRef {
        let data_set = self.data_set(default_graph);
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
/// The reference type for a model.graph factory returned by a model.graph.
///
pub type DataSetFactoryRef = Arc<dyn DataSetFactory>;

///
/// A `DataSet` is a mapping from `GraphName` to `Graph`; this introduces the notion of a named model.graph
/// although in actuality the model.graph itself is not named as the name is the key within the data set.
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
    /// Return `true` if this data set has a default model.graph, else `false`.
    ///
    fn has_default_graph(&self) -> bool;

    ///
    /// Return the default model.graph for this data set, if it exists.
    ///
    fn default_graph(&self) -> Option<&GraphRef>;

    ///
    /// Return `true` if this data set has a model.graph with the provided name, else `false`.
    ///
    fn has_graph_named(&self, name: &GraphNameRef) -> bool;

    ///
    /// Return the model.graph with the provided name from this data set, if it exists.
    ///
    fn graph_named(&self, name: &GraphNameRef) -> Option<&GraphRef>;

    ///
    /// Return an iterator over model.graph name/model.graph pairs.
    ///
    fn graphs<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a GraphNameRef, &'a GraphRef)> + 'a>;

    ///
    /// Set the provided model.graph as the default, unnamed model.graph, for this data set. Only one model.graph may
    /// be the default.
    ///
    fn set_default_graph(&mut self, graph: GraphRef);

    ///
    /// Remove any model.graph that may be set as the current default. This operation has no effect if
    /// no default model.graph is present.
    fn unset_default_graph(&mut self);

    ///
    /// Insert a new model.graph with it's associated name into the data set.
    ///
    fn insert(&mut self, name: GraphNameRef, graph: GraphRef);

    ///
    /// Remove the model.graph with the provided name from this data set. This operation has no effect if
    /// no such model.graph is present.
    ///
    fn remove(&mut self, name: &GraphNameRef);

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
/// The reference type for a model.graph data set.
///
pub type DataSetRef = Rc<RefCell<dyn DataSet>>;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod name;
pub use name::{GraphName, GraphNameRef};
