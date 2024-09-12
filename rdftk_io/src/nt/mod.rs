/*!
Provides for reading and writing a `Graph` instance in the
W3C [RDF 1.1 N-Triples](https://www.w3.org/TR/n-triples/), _a line-based syntax for an RDF graph_
format.


# Writer Example

```rust
use rdftk_io::nt::writer::NTripleWriter;
use rdftk_io::write_graph_to_string;
# use rdftk_core::model::graph::GraphRef;
# fn make_graph() -> GraphRef { rdftk_core::simple::graph::graph_factory().graph() }

let writer = NTripleWriter::default();

let result = writer.write_to_string(&make_graph());
```

*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "N-Triples";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "nt";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "application/n-triples";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod reader;
pub use reader::NTripleReader;

mod writer;
pub use writer::NTripleWriter;
