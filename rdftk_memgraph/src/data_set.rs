/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

# Example

*/

use crate::MemGraph;
use rdftk_core::data_set::{DataSet, DataSetIndex, GraphNameRef, MutableDataSet};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This implementation of the core `DataSet` and `MutableDataSet` traits is a simple in-memory hash
/// from graph name to a `MemGraph` implementation.
///
#[derive(Debug)]
pub struct MemDataSet {
    default_graph: Option<MemGraph>,
    graphs: HashMap<GraphNameRef, MemGraph>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for MemDataSet {
    fn default() -> Self {
        Self {
            default_graph: None,
            graphs: Default::default(),
        }
    }
}

impl From<MemGraph> for MemDataSet {
    fn from(v: MemGraph) -> Self {
        Self {
            default_graph: Some(v),
            graphs: Default::default(),
        }
    }
}

impl From<HashMap<GraphNameRef, MemGraph>> for MemDataSet {
    fn from(graphs: HashMap<GraphNameRef, MemGraph>) -> Self {
        Self {
            default_graph: None,
            graphs,
        }
    }
}

impl<'a> DataSet<'a, MemGraph> for MemDataSet {
    type GraphIter = std::collections::hash_map::Iter<'a, GraphNameRef, MemGraph>;

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn has_default_graph(&self) -> bool {
        self.default_graph.is_some()
    }

    fn default_graph(&self) -> &Option<MemGraph> {
        &self.default_graph
    }

    fn has_graph_named(&self, name: &GraphNameRef) -> bool {
        self.graphs.contains_key(name)
    }

    fn graph_named(&self, name: &GraphNameRef) -> Option<&MemGraph> {
        self.graphs.get(name)
    }

    fn graphs(&'a self) -> Self::GraphIter {
        self.graphs.iter()
    }

    fn has_index(&self, _: &DataSetIndex) -> bool {
        false
    }
}

impl<'a> MutableDataSet<'a, MemGraph> for MemDataSet {
    fn set_default_graph(&mut self, graph: MemGraph) {
        self.default_graph = Some(graph);
    }

    fn unset_default_graph(&mut self) {
        self.default_graph = None;
    }

    fn insert(&mut self, name: GraphNameRef, graph: MemGraph) {
        let _ = self.graphs.insert(name, graph);
    }

    fn remove(&mut self, name: &GraphNameRef) {
        let _ = self.graphs.remove(name);
    }

    fn clear(&mut self) {
        self.graphs.clear();
        self.default_graph = None;
    }
}
