/*!
One-line description.

More detailed description, with

# Example

*/

use crate::nt::parser;
use crate::GraphReader;
use rdftk_core::error::Result;
use rdftk_core::graph::{GraphFactoryRef, GraphRef};
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct NTriplesReader {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NTriplesReader {
    fn default() -> Self {
        Self {}
    }
}

impl GraphReader for NTriplesReader {
    fn read(&self, r: &mut impl Read, factory: GraphFactoryRef) -> Result<GraphRef> {
        let mut content: String = String::new();
        let _ = r.read_to_string(&mut content).map_err(io_error)?;
        parser::parse_graph(&content, factory)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub fn io_error(e: std::io::Error) -> rdftk_core::error::Error {
    use rdftk_core::error::ErrorKind;
    rdftk_core::error::Error::with_chain(e, ErrorKind::ReadWrite(super::NAME.to_string()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
