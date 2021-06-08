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

use crate::{DataSetWriter, GraphWriter};
use rdftk_core::error::Result;
use rdftk_core::model::data_set::{DataSetRef, GraphNameRef};
use rdftk_core::model::graph::GraphRef;
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

impl DataSetWriter for NQuadDataSetWriter {
    fn write(&self, w: &mut impl Write, data_set: &DataSetRef) -> Result<()> {
        let data_set = data_set.borrow();
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
    fn write(&self, w: &mut impl Write, graph: &GraphRef) -> Result<()> {
        let graph = graph.borrow();
        for subject in graph.subjects() {
            for predicate in graph.predicates_for(subject) {
                for object in graph.objects_for(subject, predicate) {
                    if let Some(graph_name) = &self.name {
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

impl NQuadGraphWriter {
    /// Construct a new quad writer with the provided model.graph name.
    pub fn named(name: GraphNameRef) -> Self {
        Self { name: Some(name) }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn io_error(e: std::io::Error) -> rdftk_core::error::Error {
    use rdftk_core::error::ErrorKind;
    rdftk_core::error::Error::with_chain(e, ErrorKind::ReadWrite(super::NAME.to_string()))
}
