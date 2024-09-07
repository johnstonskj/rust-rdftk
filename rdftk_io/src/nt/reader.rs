/*!
Provides the `NTripleReader` implementation of the `GraphReader` trait.

# Example

```rust
use rdftk_core::simple::graph_factory;
use rdftk_io::nt::reader::NTriplesReader;
use rdftk_io::GraphReader;
use std::fs::File;
use std::path::PathBuf;

let file_path = PathBuf::from("tests/w3c/nt/literal.nt");
let mut file = File::open(file_path).unwrap();
let reader = NTriplesReader::default();
assert!(reader.read(&mut file, graph_factory()).is_ok());
```

*/

use crate::nt::parser;
use crate::GraphReader;
use rdftk_core::error::Result;
use rdftk_core::model::graph::{GraphFactoryRef, GraphRef};
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An implementation of the GraphReader trait to read resources in the NTriples representation.
///
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
    fn read<R>(&self, r: &mut R, factory: GraphFactoryRef) -> Result<GraphRef>
    where
        R: Read,
    {
        let mut content: String = String::new();
        let _ = r.read_to_string(&mut content).map_err(io_error)?;
        parser::parse_graph(&content, factory)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn io_error(e: std::io::Error) -> rdftk_core::error::Error {
    use rdftk_core::error::ErrorKind;
    rdftk_core::error::Error::with_chain(e, ErrorKind::ReadWrite(super::NAME.to_string()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
