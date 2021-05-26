/*!
Provides for writing out in the [RDF 1.1 XML Syntax](https://www.w3.org/TR/rdf-syntax-grammar/)
format.
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

pub mod writer;
