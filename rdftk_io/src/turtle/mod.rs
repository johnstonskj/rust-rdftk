/*!
Provides for reading and writing a `Graph` instance in the
[RDF 1.1 Turtle](https://www.w3.org/TR/turtle/), _Terse RDF Triple Language_, format.


# Example Writer

An example, reading an existing NTriple file.

```rust
use objio::{HasOptions, ObjectReader};
use rdftk_io::nt::{NTripleReaderOptions, NTripleReader};
use rdftk_core::simple::graph_factory;
use std::fs::File;
use std::path::PathBuf;

let file_path = PathBuf::from("tests/w3c/nt/literal.nt");
let mut file = File::open(file_path).unwrap();
let reader = NTripleReader::default()
    .with_options(NTripleReaderOptions::default().with_factory(graph_factory()));
let graph = reader.read(&mut file).unwrap();
```


# Example Writer with Options

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
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "Turtle";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "ttl";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "text/turtle";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod reader;
pub use reader::{TurtleReader, TurtleReaderOptions};

mod writer;
pub use writer::{TurtleOptions, TurtleWriter};
