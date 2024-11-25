use objio::ObjectWriter;
use rdftk_core::error::Error;
use rdftk_core::model::graph::Graph;
use std::io::Write;

use crate::GraphWriter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This struct implements the `ObjectWriter` trait for graphs and will write out a serialized
/// form of the entire graph.
///
#[derive(Debug, Default)]
pub struct NTripleWriter {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectWriter<Graph> for NTripleWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &Graph) -> Result<(), Self::Error>
    where
        W: Write,
    {
        let simple_graph = graph.simplify()?;
        for subject in simple_graph.subjects() {
            for predicate in simple_graph.predicates_for(subject) {
                for object in simple_graph.objects_for(subject, predicate) {
                    writeln!(w, "{} <{}> {} .", subject, predicate, object)?;
                }
            }
        }
        Ok(())
    }
}

impl GraphWriter for NTripleWriter {}
