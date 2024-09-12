/*!
Provides for writing out in the [RDF 1.1 XML Syntax](https://www.w3.org/TR/rdf-syntax-grammar/)
format.

# Writer Example

This writer has a number of options, it can be written in a plain, streaming, form or alternatively
pretty-printed  for readability. It is also possible to pick one of the type styles described
in the specification, "flat" or "striped".

```rust
use rdftk_io::xml::writer::{XmlOptions, XmlWriter};
use rdftk_io::write_graph_to_string;
# let graph = rdftk_core::simple::graph::graph_factory().graph();

let options: XmlOptions = XmlOptions::flat().pretty().clone();

let writer = XmlWriter::new(options);

println!("{}", write_graph_to_string(&writer, &graph).unwrap());
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

#[doc(hidden)]
pub mod reader;

mod writer;
pub use writer::{XmlOptions, XmlStyle, XmlWriter};
