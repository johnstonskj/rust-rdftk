/*!
Provides for writing a `NamedGraph` instance in the
[RDF 1.1 N-Quads](https://www.w3.org/TR/n-quads/), _a line-based syntax for RDF datasets_,
format.
*/

use crate::NamedGraphWriter;
use rdftk_graph::NamedGraph;
use std::io::Write;
use std::marker::PhantomData;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This struct implements the `NamedGraphWriter` trait and will write out a serialized form of the
/// entire graph.
///
#[derive(Debug)]
pub struct NQuadWriter {
    inner: PhantomData<u8>,
}

/// The display name of this serialization format.
pub const NAME: &str = "N-Quads";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "nq";

/// The MIME type used for this serialization format.
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
