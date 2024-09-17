/*!
Provides the `TurtleWriter` implementation of the `GraphWriter` trait.

# Example

```rust
use rdftk_io::turtle::{TurtleWriter, TurtleOptions};
use rdftk_iri::{IriRef, Iri};
use std::str::FromStr;
# use objio::{HasOptions, ObjectWriter};
# use rdftk_core::model::graph::GraphRef;
# fn make_graph() -> GraphRef { rdftk_core::simple::graph::graph_factory().graph() }

let mut options = TurtleOptions::default()
    .with_id_base(Iri::from_str("http://en.wikipedia.org/wiki/").unwrap().into())
    .with_sparql_style()
    .without_nested_blank_nodes();

let writer = TurtleWriter::default()
    .with_options(options);

let result = writer.write_to_string(&make_graph());
```
*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod cursor;

mod options;
pub use options::TurtleOptions;

mod triple_type;

mod turtle_writer;
pub use turtle_writer::TurtleWriter;
