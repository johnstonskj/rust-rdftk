/*!
Provides for reading and writing a `NamedGraph` instance in the
W3C [RDF 1.1 N-Quads](https://www.w3.org/TR/n-quads/), _a line-based syntax for RDF datasets_,
format.

Provides the `NQuadDataSetWriter` implementation of the `DataSetWriter` trait and the
`NQuadGraphWriter` implementation of the `GraphWriter` trait.

# Example Writer

```rust
use rdftk_io::nq::NQuadWriter;
# use objio::ObjectWriter;
# use rdftk_core::model::data_set::DataSet;
# fn make_data_set() -> DataSet { DataSet::default() }

let writer = NQuadWriter::default();

let result = writer.write_to_string(&make_data_set());
```

*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "N-Quads";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "nq";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "application/n-quads";

/// An IRI that defines the language.
pub const FORMAT_IRI: &str = "http://www.w3.org/ns/formats/N-Quads";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod reader;
pub use reader::NQuadReader;

mod writer;
pub use writer::NQuadWriter;
