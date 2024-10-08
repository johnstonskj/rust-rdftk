/*!
Provides for reading and writing a `Graph` instance in the
[RDF 1.1 Turtle](https://www.w3.org/TR/turtle/), _Terse RDF Triple Language_, format.


# Example Writer

An example, reading an existing NTriple file.

```rust
use objio::{HasOptions, ObjectReader};
use rdftk_io::nt::NTripleReader;
use std::fs::File;
use std::path::PathBuf;

let file_path = PathBuf::from("tests/w3c/nt/literal.nt");
let mut file = File::open(file_path).unwrap();
let reader = NTripleReader::default();
let graph = reader.read(&mut file).unwrap();
```


# Example Writer with Options

```rust
use rdftk_io::turtle::{TurtleWriter, TurtleWriterOptions};
use rdftk_iri::Iri;
use std::str::FromStr;
# use objio::{HasOptions, ObjectWriter};
# use rdftk_core::model::graph::Graph;
# fn make_graph() -> Graph { Graph::default() }

let mut options = TurtleWriterOptions::default()
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
pub use reader::TurtleReader;

mod writer;
pub use writer::{TurtleWriter, TurtleWriterOptions};
