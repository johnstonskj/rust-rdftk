/*!
Provides for writing out in both the
[RDF 1.1 N-Triples](https://www.w3.org/TR/n-triples/), _a line-based syntax for an RDF graph_, and
[RDF 1.1 N-Quads](https://www.w3.org/TR/n-quads/), _a line-based syntax for RDF datasets_,
formats.

# Example

TBD

*/

use crate::{GraphWriter, StatementWriter};
use rdftk_core::Statement;
use rdftk_graph::Graph;
use std::io::Write;
use std::marker::PhantomData;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct NTripleWriter {
    inner: PhantomData<u8>,
}

pub const NAME: &str = "N-Triples";

pub const FILE_EXTENSION: &str = "nt";

pub const MIME_TYPE: &str = "application/n-triples";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NTripleWriter {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<W: Write> StatementWriter<W> for NTripleWriter {
    fn write(&self, w: &mut W, statement: &Statement) -> std::io::Result<()> {
        write!(w, "{}", statement)
    }
}

impl<W: Write, G: Graph> GraphWriter<W, G> for NTripleWriter {
    fn write(&self, w: &mut W, graph: &G) -> std::io::Result<()> {
        self.write_with(w, graph, self)
    }

    fn begin(&self, _: &mut W, _: &G) -> std::io::Result<()> {
        Ok(())
    }

    fn end(&self, _: &mut W, _: &G) -> std::io::Result<()> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
