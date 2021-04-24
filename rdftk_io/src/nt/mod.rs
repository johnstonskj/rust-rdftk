/*!
Provides for reading and writing a `Graph` instance in the
W3C [RDF 1.1 N-Triples](https://www.w3.org/TR/n-triples/), _a line-based syntax for an RDF graph_
format.
*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "N-Triples";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "nt";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "application/n-triples";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod reader;

pub mod writer;
