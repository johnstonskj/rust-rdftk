/*!
Provides for reading and writing a `Graph` instance in the
W3C [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_ format.
*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "TriG";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "trig";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "text/trig";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod reader;
pub use reader::{TrigReader, TrigReaderOptions};

mod writer;
pub use writer::{TrigWriter, TrigWriterOptions};
