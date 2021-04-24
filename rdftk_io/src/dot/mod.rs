/*!
Provides for writing a `Graph` instance in the [GraphViz](https://graphviz.gitlab.io/) dot file
format.
*/

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

/// The display name of this serialization format.
pub const NAME: &str = "GraphViz";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "dot";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "text/vnd.graphviz";

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod writer;
