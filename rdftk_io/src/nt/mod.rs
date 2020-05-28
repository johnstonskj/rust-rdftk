/*!
Provides for writing a `Graph` instance in the
[RDF 1.1 N-Triples](https://www.w3.org/TR/n-triples/), _a line-based syntax for an RDF graph_
format.
*/

use crate::GraphWriter;
use rdftk_graph::Graph;
use std::io::Write;
use std::marker::PhantomData;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This struct implements the `GraphWriter` trait and will write out a serialized form of the
/// entire graph.
///
#[derive(Debug)]
pub struct NTripleWriter {
    inner: PhantomData<u8>,
}

/// The display name of this serialization format.
pub const NAME: &str = "N-Triples";

/// The common file extension for this serialization format.
pub const FILE_EXTENSION: &str = "nt";

/// The MIME type used for this serialization format.
pub const MIME_TYPE: &str = "application/n-triples";

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

impl GraphWriter for NTripleWriter {
    fn write(&self, w: &mut impl Write, graph: &impl Graph) -> std::io::Result<()> {
        for statement in graph.statements() {
            writeln!(
                w,
                "{} <{}> {} .",
                statement.subject(),
                statement.predicate(),
                statement.object()
            )?;
        }
        Ok(())
    }
}
