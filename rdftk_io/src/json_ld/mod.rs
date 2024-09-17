/*!
Provides support for the W3C's
[JSON-LD 1.1](https://www.w3.org/TR/json-ld/), _A JSON-based Serialization for Linked Data_, format.

*/

/// The display name of this serialization format.
pub const NAME: &str = "JSON-LD";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "jsonld";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "application/ld+json";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub mod reader;

#[doc(hidden)]
pub mod writer;
