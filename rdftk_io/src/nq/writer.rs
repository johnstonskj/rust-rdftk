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
use rdftk_core::data_set::{DataSet, GraphNameRef};
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
    name: Option<GraphNameRef>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NQuadDataSetWriter {
    fn default() -> Self {
        Self {}
    }
}

impl<'a, G: 'a> DataSetWriter<'a, G> for NQuadDataSetWriter
where
    G: Graph<'a>,
{
    fn write(&self, w: &mut impl Write, data_set: &'a impl DataSet<'a, G>) -> Result<()> {
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
    fn write<'a>(&self, w: &mut impl Write, graph: &impl Graph<'a>) -> Result<()> {
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
    /// Construct a new quad writer with the provided graph name.
    pub fn named(name: GraphNameRef) -> Self {
        Self { name: Some(name) }
    }
}
