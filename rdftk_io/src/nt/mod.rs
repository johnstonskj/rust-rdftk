/*!
Provides for reading and writing a `Graph` instance in the
W3C [RDF 1.1 N-Triples](https://www.w3.org/TR/n-triples/), _a line-based syntax for an RDF graph_
format.


# Example Writer

```rust
use rdftk_io::nt::NTripleWriter;
# use objio::ObjectWriter;
# use rdftk_core::model::graph::Graph;
# fn make_graph() -> Graph { Graph::default() }

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
