/*!
Simple, in-memory implementation of the `DataSet` and `DataSetFactory` traits.
*/

use crate::model::data_set::{
    DataSet, DataSetFactory, DataSetFactoryRef, DataSetRef, GraphNameRef,
};
use crate::model::features::Featured;
use crate::model::graph::{GraphFactoryRef, GraphRef};
use crate::model::Provided;
use crate::simple::graph_factory;
use rdftk_iri::IRIRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `DataSet` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleDataSet {
    default_graph: Option<GraphRef>,
    graphs: HashMap<GraphNameRef, GraphRef>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Retrieve the `DataSet` factory for `simple::SimpleDataSet` instances.
///
pub fn data_set_factory() -> DataSetFactoryRef {
    FACTORY.clone()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `DataSetFactory` trait.
///
#[derive(Clone, Debug)]
struct SimpleDataSetFactory {}

lazy_static! {
    static ref FACTORY: Arc<SimpleDataSetFactory> = Arc::new(SimpleDataSetFactory {});
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleDataSetFactory {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl DataSetFactory for SimpleDataSetFactory {
    fn data_set(&self, default_graph: Option<GraphRef>) -> DataSetRef {
        Rc::new(RefCell::new(SimpleDataSet {
            default_graph,
            graphs: Default::default(),
        }))
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for SimpleDataSet {
    fn supports_feature(&self, _feature: &IRIRef) -> bool {
        false
    }
}

impl DataSet for SimpleDataSet {
    fn is_empty(&self) -> bool {
        self.graphs.is_empty() && self.default_graph.is_none()
    }

    fn len(&self) -> usize {
        self.graphs.len() + (if self.default_graph.is_some() { 1 } else { 0 })
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
        data_set_factory()
    }

    fn graph_factory(&self) -> GraphFactoryRef {
        graph_factory()
    }
}
