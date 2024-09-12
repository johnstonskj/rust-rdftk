/*!
One-line description.

More detailed description, with

# Example

*/

use crate::turtle::writer::TurtleOptions;
use objio::{impl_has_options, ObjectWriter};
use rdftk_core::error::Error;
use rdftk_core::model::{data_set::DataSetRef, graph::NamedGraphRef};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct TrigWriter {
    options: TurtleOptions,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_options!(TrigWriter, TurtleOptions);

impl ObjectWriter<DataSetRef> for TrigWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, data_set: &DataSetRef) -> Result<(), Self::Error>
    where
        W: std::io::Write,
    {
        let data_set = data_set.borrow();
        for graph in data_set.graphs() {
            self.write(w, graph)?;
        }
        Ok(())
    }
}

impl ObjectWriter<NamedGraphRef> for TrigWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &NamedGraphRef) -> Result<(), Self::Error>
    where
        W: std::io::prelude::Write,
    {
        let graph = graph.borrow();
        if let Some(name) = graph.name() {
            w.write_all(name.to_string().as_bytes())?;
            w.write_all(b" ")?;
        }
        w.write_all(b"{\n")?;

        // TODO: call the TurtleWriter

        w.write_all(b"}\n")?;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
