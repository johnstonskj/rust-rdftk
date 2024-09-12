/*!
Provides the `TurtleWriter` implementation of the `GraphWriter` trait.

# Example

```rust
use rdftk_io::turtle::writer::TurtleWriter;
use rdftk_io::turtle::writer::TurtleOptions;
use rdftk_io::write_graph_to_string;
use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;
# use rdftk_core::model::graph::GraphRef;
# fn make_graph() -> GraphRef { rdftk_core::simple::graph::graph_factory().graph() }

let mut options = TurtleOptions::default();
options.use_sparql_style = true;
options.nest_blank_nodes = false;
let writer = TurtleWriter::with_id_base(
    &IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/").unwrap()),
    options,
);

let result = write_graph_to_string(&writer, &make_graph());
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
