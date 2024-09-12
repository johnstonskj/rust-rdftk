/*!
Provides for reading and writing a `NamedGraph` instance in the
W3C [RDF 1.1 N-Quads](https://www.w3.org/TR/n-quads/), _a line-based syntax for RDF datasets_,
format.

Provides the `NQuadDataSetWriter` implementation of the `DataSetWriter` trait and the
`NQuadGraphWriter` implementation of the `GraphWriter` trait.

# Example

```rust
use rdftk_io::nq::writer::NQuadDataSetWriter;
use rdftk_io::write_data_set_to_string;
# use std::cell::RefCell;
# use std::rc::Rc;
# use rdftk_core::model::data_set::DataSetRef;
# use rdftk_core::simple::data_set::data_set_factory;
# fn make_data_set() -> DataSetRef { data_set_factory().data_set(None) }

let writer = NQuadDataSetWriter::default();

let result = write_data_set_to_string(&writer, &make_data_set());
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
