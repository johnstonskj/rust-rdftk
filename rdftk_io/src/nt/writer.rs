/*!
Provides the `NTripleWriter` implementation of the `GraphWriter` trait.

# Example

```rust
use rdftk_io::nt::writer::NTripleWriter;
use rdftk_io::write_graph_to_string;
# use rdftk_memgraph::MemGraph;
# fn make_graph() -> MemGraph { MemGraph::default() }

let writer = NTripleWriter::default();

let result = write_graph_to_string(&writer, &make_graph());
```

*/

use crate::error::Result;
use crate::nq::writer::NQuadGraphWriter;
use crate::GraphWriter;
use rdftk_core::graph::Graph;
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
    fn write<'a>(&self, w: &mut impl Write, graph: &'a impl Graph<'a>) -> Result<()> {
        let inner_writer = NQuadGraphWriter::default();
        inner_writer.write(w, graph)
    }
}
