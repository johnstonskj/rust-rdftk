/*!
Simple, in-memory implementation of the `DataSet` and `DataSetFactory` traits.
*/

use crate::model::data_set::{DataSet, DataSetFactory, DataSetFactoryRef, DataSetRef};
use crate::model::features::Featured;
use crate::model::graph::{named::GraphNameRef, GraphFactoryRef, NamedGraphRef};
use crate::model::Provided;
use crate::simple::graph_factory;
use lazy_static::lazy_static;
use rdftk_iri::IriRef;
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
    graphs: HashMap<Option<GraphNameRef>, NamedGraphRef>,
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
    fn data_set(&self) -> DataSetRef {
        Rc::new(RefCell::new(SimpleDataSet {
            graphs: Default::default(),
        }))
    }

    fn graph_factory(&self) -> GraphFactoryRef {
        graph_factory()
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for SimpleDataSet {
    fn supports_feature(&self, _feature: &IriRef) -> bool {
        false
    }
}

impl DataSet for SimpleDataSet {
    fn is_empty(&self) -> bool {
        self.graphs.is_empty()
    }

    fn len(&self) -> usize {
        self.graphs.len()
    }

    // --------------------------------------------------------------------------------------------
    // Accessors
    // --------------------------------------------------------------------------------------------

    fn contains_graph(&self, name: &Option<GraphNameRef>) -> bool {
        self.graphs.contains_key(name)
    }

    fn graph(&self, name: &Option<GraphNameRef>) -> Option<&NamedGraphRef> {
        self.graphs.get(name)
    }

    fn graph_mut(&mut self, name: &Option<GraphNameRef>) -> Option<&mut NamedGraphRef> {
        self.graphs.get_mut(name)
    }

    fn graphs(&self) -> Box<dyn Iterator<Item = &NamedGraphRef> + '_> {
        Box::from(self.graphs.values())
    }

    // --------------------------------------------------------------------------------------------
    // Mutators
    // --------------------------------------------------------------------------------------------

    fn insert(&mut self, graph: NamedGraphRef) {
        let graph_name = graph.borrow().name().cloned();
        let _ = self.graphs.insert(graph_name, graph);
    }

    fn extend(&mut self, graphs: Vec<NamedGraphRef>) {
        graphs.into_iter().for_each(|g| self.insert(g))
    }

    fn remove(&mut self, name: &Option<GraphNameRef>) {
        let _ = self.graphs.remove(name);
    }

    fn clear(&mut self) {
        self.graphs.clear();
    }

    // --------------------------------------------------------------------------------------------
    // Factories
    // --------------------------------------------------------------------------------------------

    fn factory(&self) -> DataSetFactoryRef {
        data_set_factory()
    }

    fn graph_factory(&self) -> GraphFactoryRef {
        graph_factory()
    }
}
