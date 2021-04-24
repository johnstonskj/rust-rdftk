/*!
* Provides the `NQuadDataSetWriter` implementation of the `DataSetWriter` trait and the
* `NQuadGraphWriter` implementation of the `GraphWriter` trait.
*
* # Example
*
* ```rust
* use rdftk_io::nq::writer::NQuadDataSetWriter;
* use rdftk_io::write_data_set_to_string;
* # use rdftk_memgraph::MemGraph;use rdftk_memgraph::data_set::MemDataSet;
* # fn make_data_set() -> MemDataSet { MemDataSet::default() }
*
* let writer = NQuadDataSetWriter::default();
*
* let result = write_data_set_to_string(&writer, &make_data_set());
* ```
*
*/

use crate::error::Result;
use crate::{DataSetWriter, GraphWriter};
use rdftk_core::data_set::{DataSet, GraphName};
use rdftk_core::Graph;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This struct implements the `DataSetWriter` trait and will write out a serialized form of the
/// entire data set.
///
#[derive(Debug)]
pub struct NQuadDataSetWriter {}

///
/// This struct implements the `DataSetWriter` trait and will write out a serialized form of the
/// entire data set.
///
#[derive(Debug)]
pub struct NQuadGraphWriter {
    name: Option<GraphName>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NQuadDataSetWriter {
    fn default() -> Self {
        Self {}
    }
}

impl<G: Graph> DataSetWriter<G> for NQuadDataSetWriter {
    fn write(&self, w: &mut impl Write, data_set: &impl DataSet<G>) -> Result<()> {
        if let Some(graph) = data_set.default_graph() {
            let inner_writer = NQuadGraphWriter::default();
            inner_writer.write(w, graph)?;
        }
        for (name, graph) in data_set.graphs() {
            let inner_writer = NQuadGraphWriter::named(name.clone());
            inner_writer.write(w, graph)?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for NQuadGraphWriter {
    fn default() -> Self {
        Self { name: None }
    }
}

impl GraphWriter for NQuadGraphWriter {
    fn write(&self, w: &mut impl Write, graph: &impl Graph) -> Result<()> {
        for subject in graph.subjects() {
            for predicate in graph.predicates_for(subject) {
                for object in graph.objects_for(subject, predicate) {
                    if let Some(graph_name) = &self.name {
                        writeln!(w, "{} <{}> {} {} .", subject, predicate, object, graph_name)?;
                    } else {
                        writeln!(w, "{} <{}> {} .", subject, predicate, object)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl NQuadGraphWriter {
    pub fn named(name: GraphName) -> Self {
        Self { name: Some(name) }
    }
}
