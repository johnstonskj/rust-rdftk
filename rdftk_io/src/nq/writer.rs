use objio::ObjectWriter;
use rdftk_core::error::Error;
use rdftk_core::model::data_set::DataSetRef;
use rdftk_core::model::graph::named::NamedGraphRef;
use std::io::Write;

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

impl ObjectWriter<DataSetRef> for NQuadWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, data_set: &DataSetRef) -> Result<(), Self::Error>
    where
        W: Write,
    {
        let data_set = data_set.borrow();
        for graph in data_set.graphs() {
            self.write(w, graph)?;
        }
        Ok(())
    }
}

impl ObjectWriter<NamedGraphRef> for NQuadWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &NamedGraphRef) -> Result<(), Self::Error>
    where
        W: Write,
    {
        let graph = graph.borrow();
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
