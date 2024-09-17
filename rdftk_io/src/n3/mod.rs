/*!
Provides for reading and writing a `Graph` instance in the
proposed W3C [Notation3 (N3)](https://www.w3.org/TeamSubmission/n3/), _a readable RDF syntax_,
format.
*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "N3";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "n3";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "application/n3";

/// An IRI that defines the language.
pub const FORMAT_IRI: &str = "http://www.w3.org/ns/formats/N3";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod reader;
pub use reader::{N3Reader, N3ReaderOptions};

pub mod writer;
