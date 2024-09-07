/*!
Provides the `NTripleWriter` implementation of the `GraphWriter` trait.

# Example

```rust
use rdftk_io::nt::writer::NTripleWriter;
use rdftk_io::write_graph_to_string;
# use rdftk_core::model::graph::GraphRef;
# fn make_graph() -> GraphRef { rdftk_core::simple::graph::graph_factory().graph() }

let writer = NTripleWriter::default();

let result = write_graph_to_string(&writer, &make_graph());
```

*/

use crate::turtle::writer::io_error;
use crate::GraphWriter;
use rdftk_core::error::Result;
use rdftk_core::model::graph::GraphRef;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This struct implements the `GraphWriter` trait and will write out a serialized form of the
/// entire graph.
///
#[derive(Debug)]
pub struct NTripleWriter {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NTripleWriter {
    fn default() -> Self {
        Self {}
    }
}

impl GraphWriter for NTripleWriter {
    fn write<W>(&self, w: &mut W, graph: &GraphRef) -> Result<()>
    where
        W: Write,
    {
        let graph = graph.borrow();
        for subject in graph.subjects() {
            for predicate in graph.predicates_for(subject) {
                for object in graph.objects_for(subject, predicate) {
                    writeln!(w, "{} <{}> {} .", subject, predicate, object).map_err(io_error)?;
                }
            }
        }
        Ok(())
    }
}
