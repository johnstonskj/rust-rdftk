/*!
Provides an implementation of the W3C
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/) recommendation.
Additional semantics taken from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_.

# Example

*/

use crate::{Graph, PrefixMappings};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait DataSet<G>
where
    G: Graph,
{
    fn has_default_graph(&self) -> bool;

    fn default_graph(&self) -> &Option<G>;

    fn has_graph_named(&self, name: &GraphName) -> bool;

    fn graph_named(&self, name: &GraphName) -> Option<&G>;

    fn graphs(&self) -> Vec<(&GraphName, &G)>;

    fn all_prefix_mappings(&self) -> Rc<dyn PrefixMappings>;
}

pub trait MutableDataSet<G>: DataSet<G>
where
    G: Graph,
{
    fn set_default_graph(&mut self, graph: G);

    fn unset_default_graph(&mut self);

    fn insert_graph(&mut self, name: GraphName, graph: G);

    fn remove_graph(&mut self, name: &GraphName);
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod name;
pub use name::GraphName;
use std::rc::Rc;
