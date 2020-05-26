/*!
Provides for writing a graph in the
[RDF 1.1 N-Quads](https://www.w3.org/TR/n-quads/), _a line-based syntax for RDF datasets_,
formats

# Example

TBD

*/

use crate::NamedGraphWriter;
use rdftk_graph::NamedGraph;
use std::io::Write;
use std::marker::PhantomData;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct NQuadWriter {
    inner: PhantomData<u8>,
}

pub const NAME: &str = "N-Quads";

pub const FILE_EXTENSION: &str = "nq";

pub const MIME_TYPE: &str = "application/n-quads";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NQuadWriter {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<W: Write, G: NamedGraph> NamedGraphWriter<W, G> for NQuadWriter {
    fn write(&self, w: &mut W, graph: &G) -> std::io::Result<()> {
        for statement in graph.statements() {
            write!(
                w,
                "{} <{}> {} {} .",
                statement.subject(),
                statement.predicate(),
                statement.object(),
                graph.name().as_ref().unwrap(),
            )?;
        }
        Ok(())
    }
}
