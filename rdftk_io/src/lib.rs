/*!
Traits for reading/writing `Statement`s and `Graph`s as well as implementations for common file
formats.

The following are some well-known formats (see [Wikipedia](https://en.wikipedia.org/wiki/Resource_Description_Framework#Serialization_formats)
for a description of different serializations), support is indicated in the final column with
an **R** for read support and **W** for write support. One additional module, `dot` allows for the
creation of [GraphViz](https://graphviz.gitlab.io/) dot files for a visualization of a graph's structure.

| Module   | Name          | MIME Type                                       | Specification | R/W |
|----------|---------------|-------------------------------------------------|---------------|-----|
| `nt`     | [![N-Triples](https://img.shields.io/badge/RDF-N--Triples-blue)](https://www.w3.org/TR/n-triples/) | `application/n-triples` | [W3C](https://www.w3.org/TR/n-triples/) | **W** |
| `nq`     | [![N-Quads](https://img.shields.io/badge/RDF-N--Quads-blue)](https://www.w3.org/TR/n-quads/)       | `application/n-quads`   | [W3C](https://www.w3.org/TR/n-quads/) | **W** |
| `n3`     | [![N3](https://img.shields.io/badge/RDF-N3-blue)](https://www.w3.org/TeamSubmission/n3/)           | `text/rdf+n3`           | [W3C Submission](https://www.w3.org/TeamSubmission/n3/) |     |
| `turtle` |[![Turtle](https://img.shields.io/badge/RDF-Turtle-blue)](https://www.w3.org/TR/turtle/)            | `text/turtle`           | [W3C](https://www.w3.org/TR/turtle/) | **W** |
| `xml`    | RDF/XML       | `application/rdf+xml`       | [W3C](https://www.w3.org/TR/rdf-syntax-grammar/) |     |
| `json`   | JSON-LD       | `application/ld+json`       | [W3C](https://www.w3.org/TR/json-ld/) |     |
| TBD      | [![RDFa](https://www.w3.org/Icons/SW/Buttons/sw-rdfa-blue.png)](http://www.w3.org/2001/sw/wiki/RDFa) | `text/html`                            | [W3C](https://www.w3.org/TR/rdfa-core/) |     |
| TBD      | RDF/JSON      | `application/rdf+json`      | [W3C](https://www.w3.org/TR/rdf-json/) |     |
| TBD      | TriG          | `application/trig`          | [W3C](https://www.w3.org/TR/trig/) |     |
| TBD      | HDT           | ?                           | [W3C Submission](https://www.w3.org/Submission/2011/SUBM-HDT-20110330/) |     |
| TBD      | BinaryRDF     | `application/x-binary-rdf`  | [Community](https://afs.github.io/rdf-thrift/rdf-binary-thrift.html) |     |

Each module will also provide public constants `NAME`, `FILE_EXTENSION`, and `MIME_TYPE`.

# Example

TBD

*/

use rdftk_graph::{Graph, NamedGraph};
use std::io::{Read, Write};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Read an entire `Graph` from the provided implementation of
/// [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html).
///
pub trait GraphReader {
    fn read<R: Read, G: Graph>(&self, r: &mut R) -> std::io::Result<Rc<G>>;
}

// ------------------------------------------------------------------------------------------------

// ///
// /// Write a single [`Statement`](../rdftk_core/statement/struct.Statement.html) using the provided
// /// implementation of [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html).
// ///
// pub trait StatementWriter<W: Write> {
//     /// Write the formatted statement `statement` using the write implementation `w`.
//     fn write(&self, w: &mut W, statement: &Statement) -> std::io::Result<()>;
// }
//

///
/// Write all [`Statement`](../rdftk_core/statement/struct.Statement.html)s in the
/// [`Graph`](../rdftk_graph/graph/trait.Graph.html) using the provided implementation of
/// [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html).
///
pub trait GraphWriter {
    /// Write the formatted graph `Graph` using the write implementation `w`.
    fn write(&self, w: &mut impl Write, graph: &impl Graph) -> std::io::Result<()>;
}

///
/// Write all [`Statement`](../rdftk_core/statement/struct.Statement.html)s in the
/// [`NamedGraph`](../rdftk_graph/graph/trait.NamedGraph.html) using the provided implementation of
/// [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html).
///
pub trait NamedGraphWriter {
    /// Write the formatted graph `Graph` using the write implementation `w`.
    fn write(&self, w: &mut impl Write, graph: &impl NamedGraph) -> std::io::Result<()>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_graph_to_string(w: &impl GraphWriter, graph: &impl Graph) -> std::io::Result<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    w.write(&mut buffer, graph)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}

pub fn write_named_graph_to_string(
    w: &impl NamedGraphWriter,
    graph: &impl NamedGraph,
) -> std::io::Result<String> {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    w.write(&mut buffer, graph)?;
    Ok(String::from_utf8(buffer.into_inner()).unwrap())
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod dot;

#[doc(hidden)]
pub mod json;

#[doc(hidden)]
pub mod n3;

pub mod nq;

pub mod nt;

#[doc(hidden)]
pub mod turtle;

#[doc(hidden)]
pub mod xml;
