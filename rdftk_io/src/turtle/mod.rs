/*!
Provides for reading and writing a `Graph` instance in the
[RDF 1.1 Turtle](https://www.w3.org/TR/turtle/), _Terse RDF Triple Language_, format.

*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "Turtle";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "ttl";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "text/turtle";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod reader;
pub use reader::{TurtleReader, TurtleReaderOptions};

mod writer;
pub use writer::{TurtleOptions, TurtleWriter};
