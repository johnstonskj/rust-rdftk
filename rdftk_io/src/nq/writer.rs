use objio::ObjectWriter;
use rdftk_core::error::Error;
use rdftk_core::model::data_set::DataSet;
use rdftk_core::model::graph::Graph;
use std::io::Write;

use crate::GraphWriter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// .
///
#[derive(Debug, Default)]
pub struct NQuadWriter {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectWriter<DataSet> for NQuadWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, data_set: &DataSet) -> Result<(), Self::Error>
    where
        W: Write,
    {
        for graph in data_set.graphs() {
            self.write(w, graph)?;
        }
        Ok(())
    }
}

impl ObjectWriter<Graph> for NQuadWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &Graph) -> Result<(), Self::Error>
    where
        W: Write,
    {
        let graph_name = graph.name();
        for subject in graph.subjects() {
            for predicate in graph.predicates_for(subject) {
                for object in graph.objects_for(subject, predicate) {
                    if let Some(graph_name) = graph_name {
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

impl GraphWriter for NQuadWriter {}
