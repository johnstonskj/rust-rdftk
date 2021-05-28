/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

# Example

*/

use rdftk_core::data_set::{DataSet, DataSetFactory, DataSetFactoryRef, DataSetRef, GraphNameRef};
use rdftk_core::graph::{Featured, GraphRef};
use rdftk_iri::IRIRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An implementation of the data set factory trait.
///
#[derive(Clone, Debug)]
pub struct MemDataSetFactory {}

///
/// This implementation of the core `DataSet` and `MutableDataSet` traits is a simple in-memory hash
/// from graph name to a `MemGraph` implementation.
///
#[allow(missing_debug_implementations)]
pub struct MemDataSet {
    default_graph: Option<GraphRef>,
    graphs: HashMap<GraphNameRef, GraphRef>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Retrieve the graph factory for simple `MemGraph` instances.
///
pub fn data_set_factory() -> DataSetFactoryRef {
    FACTORY.clone()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref FACTORY: Arc<MemDataSetFactory> = Arc::new(MemDataSetFactory {});
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Featured for MemDataSetFactory {
    fn supports_feature(&self, _feature: &IRIRef) -> bool {
        false
    }
}

impl DataSetFactory for MemDataSetFactory {
    fn new_data_set(&self, default_graph: Option<GraphRef>) -> DataSetRef {
        Rc::new(RefCell::new(MemDataSet {
            default_graph,
            graphs: Default::default(),
        }))
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for MemDataSet {
    fn supports_feature(&self, _feature: &IRIRef) -> bool {
        false
    }
}

impl DataSet for MemDataSet {
    fn is_empty(&self) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn has_default_graph(&self) -> bool {
        self.default_graph.is_some()
    }

    fn default_graph(&self) -> Option<&GraphRef> {
        self.default_graph.as_ref()
    }

    fn has_graph_named(&self, name: &GraphNameRef) -> bool {
        self.graphs.contains_key(name)
    }

    fn graph_named(&self, name: &GraphNameRef) -> Option<&GraphRef> {
        self.graphs.get(name)
    }

    fn graphs<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a GraphNameRef, &'a GraphRef)> + 'a> {
        Box::new(self.graphs.iter())
    }

    // --------------------------------------------------------------------------------------------
    // Mutators
    // --------------------------------------------------------------------------------------------

    fn set_default_graph(&mut self, graph: GraphRef) {
        self.default_graph = Some(graph);
    }

    fn unset_default_graph(&mut self) {
        self.default_graph = None;
    }

    fn insert(&mut self, name: GraphNameRef, graph: GraphRef) {
        let _ = self.graphs.insert(name, graph);
    }

    fn remove(&mut self, name: &GraphNameRef) {
        let _ = self.graphs.remove(name);
    }

    fn clear(&mut self) {
        self.graphs.clear();
        self.default_graph = None;
    }

    fn factory(&self) -> DataSetFactoryRef {
        FACTORY.clone()
    }
}
