/*!
Traits for reading/wtiting `Statement`s and `Graph`s as well as implementations for common file
formats.

The following are some well-known formats (see [Wikipedia](https://en.wikipedia.org/wiki/Resource_Description_Framework#Serialization_formats)
for a description of different serializations), support is indicated by a bold name and an **R** for
read support and **W** for write support.

| Name          | MIME Type                                       | Specification | R/W |
|---------------|-------------------------------------------------|---------------|-----|
| **N-Triples** | application/n-triples, text/plain               | [W3C](https://www.w3.org/TR/n-triples/)                 | **W** |
| N-Quads       | application/n-quads, text/x-nquads, text/nquads | [W3C](https://www.w3.org/TR/n-quads/)                   |     |
| N3            | text/n3, text/rdf+n3                            | [W3C Submission](https://www.w3.org/TeamSubmission/n3/) |     |
| Turtle        | text/turtle, application/x-turtle               | [W3C](https://www.w3.org/TR/turtle/)                    |     |
| RDF/XML       | application/rdf+xml, application/xml            | [W3C](https://www.w3.org/TR/rdf-syntax-grammar/)        |     |
| JSON-LD       | application/ld+json                             | [W3C](https://www.w3.org/TR/json-ld/)                   |     |
| RDF/JSON      | application/rdf+json                            | [W3C](https://www.w3.org/TR/rdf-json/)                  |     |
| TriG          | application/trig, application/x-trig            | [W3C](https://www.w3.org/TR/trig/)                      |     |
| RDFa          | ?                                               | [W3C](https://www.w3.org/TR/rdfa-core/)                 |     |
| HDT           | ?                                               | [W3C Submission](https://www.w3.org/Submission/2011/SUBM-HDT-20110330/) |     |
| BinaryRDF     | application/x-binary-rdf                        | [Community](https://afs.github.io/rdf-thrift/rdf-binary-thrift.html)    |     |

Each module will also provide public constants `NAME`, `FILE_EXTENSION`, and `MIME_TYPE`.

# Example

TBD

*/

use rdftk_core::Statement;
use rdftk_graph::Graph;
use std::io::{Read, Write};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Read a single [`Statement`](../rdftk_core/statement/struct.Statement.html) from the provided implementation of [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html).
pub trait StatementReader<R: Read> {
    fn read(&self, r: &mut R) -> std::io::Result<Statement>;
}

/// Read an entire `Graph` from the provided implementation of [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html).
pub trait GraphReader<R: Read, G: Graph> {
    fn read(&self, r: &mut R) -> std::io::Result<Rc<G>>;
    fn read_with(&self, r: &mut R, reader: &dyn StatementReader<R>) -> std::io::Result<Rc<G>>;
}

// ------------------------------------------------------------------------------------------------

/// Write a single [`Statement`](../rdftk_core/statement/struct.Statement.html) using the provided implementation of [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html).
pub trait StatementWriter<W: Write> {
    fn write(&self, w: &mut W, statement: &Statement) -> std::io::Result<()>;
}

/// Write all [`Statement`](../rdftk_core/statement/struct.Statement.html)s in the [`Graph`](../rdftk_graph/graph/trait.Graph.html) using the provided implementation of [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html).
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

#[doc(hidden)]
pub mod json;

#[doc(hidden)]
pub mod n3;

pub mod nt;

#[doc(hidden)]
pub mod turtle;

#[doc(hidden)]
pub mod xml;
