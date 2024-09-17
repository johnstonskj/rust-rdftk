/*!
Provides for writing a `Graph` instance in the
W3C [RDF 1.1 JSON Alternate Serialization (RDF/JSON)](https://www.w3.org/TR/rdf-json/) format.
 */

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "JSON";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "json";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "application/rdf+json";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod syntax;

mod reader;
pub use reader::{JsonReader, JsonReaderOptions};

mod writer;
pub use writer::{JsonOptions, JsonWriter};
