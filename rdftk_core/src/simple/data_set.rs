/*!
Simple, in-memory implementation of the `DataSet` and `DataSetFactory` traits.
*/

use super::graph::SimpleGraph;
use crate::model::data_set::{DataSet, DataSetFactory};
use crate::model::features::Featured;
use crate::model::graph::{Graph, GraphName};
use crate::model::Provided;
use rdftk_iri::Iri;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `DataSet` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleDataSet {
    graphs: HashMap<Option<GraphName>, SimpleGraph>,
}

///
/// Simple, in-memory implementation of the `DataSetFactory` trait.
///
#[derive(Clone, Debug, Default)]
pub struct SimpleDataSetFactory {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleDataSetFactory {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl DataSetFactory for SimpleDataSetFactory {
    type Graph = SimpleGraph;
    type DataSet = SimpleDataSet;

    fn data_set(&self) -> Self::DataSet {
        SimpleDataSet {
            graphs: Default::default(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for SimpleDataSet {
    fn supports_feature(&self, _feature: &Iri) -> bool {
        false
    }
}

impl DataSet for SimpleDataSet {
    type Graph = SimpleGraph;

    fn is_empty(&self) -> bool {
        self.graphs.is_empty()
    }

    fn len(&self) -> usize {
        self.graphs.len()
    }

    // --------------------------------------------------------------------------------------------
    // Accessors
    // --------------------------------------------------------------------------------------------

    fn contains_graph(&self, name: &Option<GraphName>) -> bool {
        self.graphs.contains_key(name)
    }

    fn graph(&self, name: &Option<GraphName>) -> Option<&Self::Graph> {
        self.graphs.get(name)
    }

    fn graph_mut(&mut self, name: &Option<GraphName>) -> Option<&mut Self::Graph> {
        self.graphs.get_mut(name)
    }

    fn graphs(&self) -> impl Iterator<Item = &Self::Graph> {
        Box::from(self.graphs.values())
    }

    // --------------------------------------------------------------------------------------------
    // Mutators
    // --------------------------------------------------------------------------------------------

    fn insert(&mut self, graph: Self::Graph) {
        let graph_name = graph.name().cloned();
        let _ = self.graphs.insert(graph_name, graph);
    }

    fn extend(&mut self, graphs: Vec<Self::Graph>) {
        graphs.into_iter().for_each(|g| self.insert(g))
    }

    fn remove(&mut self, name: &Option<GraphName>) {
        let _ = self.graphs.remove(name);
    }

    fn clear(&mut self) {
        self.graphs.clear();
    }
}
