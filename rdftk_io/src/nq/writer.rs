/*!
Provides the `NQuadDataSetWriter` implementation of the `DataSetWriter` trait and the
`NQuadGraphWriter` implementation of the `GraphWriter` trait.

# Example

```rust
use rdftk_io::nq::writer::NQuadDataSetWriter;
use rdftk_io::write_data_set_to_string;
# use std::cell::RefCell;
# use std::rc::Rc;
# use rdftk_core::model::data_set::DataSetRef;
# use rdftk_core::simple::data_set::data_set_factory;
# fn make_data_set() -> DataSetRef { data_set_factory().data_set(None) }

let writer = NQuadDataSetWriter::default();

let result = write_data_set_to_string(&writer, &make_data_set());
```

*/

use crate::{DataSetWriter, NamedGraphWriter};
use rdftk_core::error::Result;
use rdftk_core::model::data_set::DataSetRef;
use rdftk_core::model::graph::named::NamedGraphRef;
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
/// This struct implements the `NamedGraphWriter` trait and will write out a serialized form of a
/// member graph.
///
#[derive(Debug)]
pub struct NQuadGraphWriter {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NQuadDataSetWriter {
    fn default() -> Self {
        Self {}
    }
}

impl DataSetWriter for NQuadDataSetWriter {
    fn write<W>(&self, w: &mut W, data_set: &DataSetRef) -> Result<()>
    where
        W: Write,
    {
        let data_set = data_set.borrow();
        let graph_writer = NQuadGraphWriter::default();
        for graph in data_set.graphs() {
            graph_writer.write(w, graph)?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for NQuadGraphWriter {
    fn default() -> Self {
        Self {}
    }
}

impl NamedGraphWriter for NQuadGraphWriter {
    fn write<W>(&self, w: &mut W, graph: &NamedGraphRef) -> Result<()>
    where
        W: Write,
    {
        let graph = graph.borrow();
        let graph_name = graph.name();
        for subject in graph.subjects() {
            for predicate in graph.predicates_for(subject) {
                for object in graph.objects_for(subject, predicate) {
                    if let Some(graph_name) = graph_name {
                        writeln!(w, "{} <{}> {} {} .", subject, predicate, object, graph_name)
                            .map_err(io_error)?;
                    } else {
                        writeln!(w, "{} <{}> {} .", subject, predicate, object)
                            .map_err(io_error)?;
                    }
                }
            }
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn io_error(e: std::io::Error) -> rdftk_core::error::Error {
    use rdftk_core::error::ErrorKind;
    rdftk_core::error::Error::with_chain(e, ErrorKind::ReadWrite(super::NAME.to_string()))
}
