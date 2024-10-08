/*!
Provides for writing out in the [RDF 1.1 XML Syntax](https://www.w3.org/TR/rdf-syntax-grammar/)
format.

# Example Writer

This writer has a number of options, it can be written in a plain, streaming, form or alternatively
pretty-printed  for readability. It is also possible to pick one of the type styles described
in the specification, "flat" or "striped".

```rust
use rdftk_io::xml::{XmlOptions, XmlWriter};
# use objio::{HasOptions, ObjectWriter};
# use  rdftk_core::model::graph::Graph;
# fn make_graph() -> Graph { Graph::default() }

let options: XmlOptions = XmlOptions::default().flat().pretty();

let writer = XmlWriter::default().with_options(options);

println!("{}", writer.write_to_string(&make_graph()).unwrap());
```

*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "XML";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "rdf";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "application/rdf+xml";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod syntax;

mod reader;
pub use reader::XmlReader;

mod writer;
pub use writer::{XmlOptions, XmlStyle, XmlWriter};
