/*!
Provides for writing a `Graph` instance in the [GraphViz](https://graphviz.gitlab.io/) dot file
format.


# Example

```rust
use rdftk_io::dot::{DotOptions, DotWriter};
# use objio::{HasOptions, ObjectWriter};
# use rdftk_core::model::graph::GraphRef;
# fn make_graph() -> GraphRef { rdftk_core::simple::graph::graph_factory().graph() }

let mut options = DotOptions::default().with_blank_labels(true);

let writer = DotWriter::default().with_options(options);

let result = writer.write_to_string(&make_graph());
```

*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "GraphViz";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "dot";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "text/vnd.graphviz";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod writer;
pub use writer::{DotOptions, DotWriter};
