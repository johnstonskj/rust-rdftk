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

use crate::nq::writer::NQuadGraphWriter;
use crate::GraphWriter;
use rdftk_core::error::Result;
use rdftk_core::model::graph::GraphRef;
use std::borrow::Borrow;
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
    fn write(&self, w: &mut impl Write, graph: &GraphRef) -> Result<()> {
        let graph = graph.borrow();
        let inner_writer = NQuadGraphWriter::default();
        inner_writer.write(w, graph)
    }
}
