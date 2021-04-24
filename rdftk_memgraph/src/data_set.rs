/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

# Example

*/

use crate::{Mappings, MemGraph};
use rdftk_core::data_set::{DataSet, GraphName, MutableDataSet};
use rdftk_core::{Graph, PrefixMappings};
use std::collections::HashMap;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct MemDataSet {
    default_graph: Option<MemGraph>,
    graphs: HashMap<GraphName, MemGraph>,
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

impl From<HashMap<GraphName, MemGraph>> for MemDataSet {
    fn from(graphs: HashMap<GraphName, MemGraph>) -> Self {
        Self {
            default_graph: None,
            graphs,
        }
    }
}

impl DataSet<MemGraph> for MemDataSet {
    fn has_default_graph(&self) -> bool {
        self.default_graph.is_some()
    }

    fn default_graph(&self) -> &Option<MemGraph> {
        &self.default_graph
    }

    fn has_graph_named(&self, name: &GraphName) -> bool {
        self.graphs.contains_key(name)
    }

    fn graph_named(&self, name: &GraphName) -> Option<&MemGraph> {
        self.graphs.get(name)
    }

    fn graphs(&self) -> Vec<(&GraphName, &MemGraph)> {
        self.graphs.iter().collect()
    }

    fn all_prefix_mappings(&self) -> Rc<dyn PrefixMappings> {
        let mut mappings = Mappings::default();
        if let Some(graph) = &self.default_graph {
            mappings.merge(graph.prefix_mappings());
        }
        for graph in self.graphs.values() {
            mappings.merge(graph.prefix_mappings());
        }
        Rc::from(mappings)
    }
}

impl MutableDataSet<MemGraph> for MemDataSet {
    fn set_default_graph(&mut self, graph: MemGraph) {
        self.default_graph = Some(graph);
    }

    fn unset_default_graph(&mut self) {
        self.default_graph = None;
    }

    fn insert_graph(&mut self, name: GraphName, graph: MemGraph) {
        let _ = self.graphs.insert(name, graph);
    }

    fn remove_graph(&mut self, name: &GraphName) {
        let _ = self.graphs.remove(name);
    }
}
