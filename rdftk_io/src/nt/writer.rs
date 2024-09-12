use objio::ObjectWriter;
use rdftk_core::error::Error;
use rdftk_core::model::graph::GraphRef;
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

impl ObjectWriter<GraphRef> for NTripleWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, object: &GraphRef) -> Result<(), Self::Error>
    where
        W: Write,
    {
        let graph = object.borrow();
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
