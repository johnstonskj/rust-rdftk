/*!
One-line description.

More detailed description, with

| Name    | MIME Type                                        | Specification |
|---------|--------------------------------------------------|---------------|
TriG      |  application/trig, application/x-trig            | tbd           |
TriX      |   application/trix                               | tbd           |
N3        |  text/n3, text/rdf+n3                            | tbd           |
Turtle    |  text/turtle, application/x-turtle               | tbd           |
N-Triples |  application/n-triples, text/plain               | tbd           |
RDF/XML   |  application/rdf+xml, application/xml            | tbd           |
BinaryRDF |  application/x-binary-rdf                        | tbd           |
N-Quads   |  application/n-quads, text/x-nquads, text/nquads | tbd           |
JSON-LD   |  application/ld+json                             | tbd           |
RDF/JSON  |  application/rdf+json                            | tbd           |

# Example

*/

use rdftk_core::Statement;
use rdftk_graph::Graph;
use std::io::{Read, Write};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait StatementReader<R: Read> {
    fn read(&self, r: &mut R) -> std::io::Result<Statement>;
}

pub trait GraphReader<R: Read, G: Graph> {
    fn read(&self, r: &mut R) -> std::io::Result<Rc<G>>;
    fn read_with(&self, r: &mut R, reader: &dyn StatementReader<R>) -> std::io::Result<Rc<G>>;
}

// ------------------------------------------------------------------------------------------------

pub trait StatementWriter<W: Write> {
    fn write(&self, w: &mut W, statement: &Statement) -> std::io::Result<()>;
}

pub trait GraphWriter<W: Write, G: Graph> {
    fn write(&self, w: &mut W, graph: &G) -> std::io::Result<()>;
    fn write_with(
        &self,
        w: &mut W,
        graph: &G,
        statement_writer: &dyn StatementWriter<W>,
    ) -> std::io::Result<()> {
        self.begin(w, graph)?;
        for statement in graph.statements() {
            statement_writer.write(w, &statement)?;
        }
        self.end(w, graph)?;
        Ok(())
    }
    fn begin(&self, w: &mut W, graph: &G) -> std::io::Result<()>;
    fn end(&self, w: &mut W, graph: &G) -> std::io::Result<()>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod json;

pub mod n3;

pub mod nt;

pub mod turtle;

pub mod xml;
