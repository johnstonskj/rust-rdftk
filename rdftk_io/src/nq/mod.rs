/*!
Provides for reading and writing a `Graph` instance in the
W3C [RDF 1.1 N-Quads](https://www.w3.org/TR/n-quads/), _a line-based syntax for RDF datasets_,
format.
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

#[doc(hidden)]
pub mod reader;

pub mod writer;
