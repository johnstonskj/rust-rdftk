use objio::ObjectWriter;
use rdftk_core::error::Error;
use rdftk_core::model::graph::Graph;
use std::io::Write;

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
        for subject in graph.subjects() {
            for predicate in graph.predicates_for(subject) {
                for object in graph.objects_for(subject, predicate) {
                    writeln!(w, "{} <{}> {} .", subject, predicate, object)?;
                }
            }
        }
        Ok(())
    }
}
