# RDFtk: IO

![io](https://img.shields.io/badge/RDFtk-io-BD1B89?logo=data:image/png;base64,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) This crate provides traits for reading and writing `Statement`s and `Graph`s as
well as implementations of these for common representations.

[![crates.io](https://img.shields.io/crates/v/rdftk_io.svg)](https://crates.io/crates/rdftk_io)
[![docs.rs](https://docs.rs/rdftk_io/badge.svg)](https://docs.rs/rdftk_io)

The following are some well-known formats (see [Wikipedia](https://en.wikipedia.org/wiki/Resource_Description_Framework#Serialization_formats) for a description of
different serializations), support is indicated in the final column with an **R**
for read support and **W** for write support. One additional module, `dot` allows for
the creation of [GraphViz](https://graphviz.gitlab.io/) dot files for a visualization of a graph's structure.

| Module    | Name                                                                                                | MIME Type                   | R/W     |
|-----------|---------------------------------------------------------------------------------------------------- |-----------------------------|---------|
| `nt`      | [RDF 1.1 N-Triples](https://www.w3.org/TR/n-triples/); A line-based syntax for an RDF graph         | `application/n-triples`     | **R+W** |
| `nq`      | [RDF 1.1 N-Quads](https://www.w3.org/TR/n-quads/); A line-based syntax for RDF datasets             | `application/n-quads`       | **W**   |
| `turtle`  | [RDF 1.1 Turtle](https://www.w3.org/TR/turtle/); Terse RDF Triple Language                          | `text/turtle`               | **W**   |
| `trig`    | [RDF 1.1 TriG](https://www.w3.org/TR/trig/); RDF Dataset Language                                   | `application/trig`          |         |
| `xml`     | [RDF 1.1 XML Syntax](https://www.w3.org/TR/rdf-syntax-grammar/)                                     | `application/rdf+xml`       | **W**   |
| `json`    | [RDF 1.1 JSON Alternate Serialization](https://www.w3.org/TR/rdf-json/)                             | `application/rdf+json`      | **R+W** |
| `n3`      | [Notation3 (N3): A readable RDF syntax](https://www.w3.org/TeamSubmission/n3/)                      | `text/rdf+n3`               | **W**   |
| TBD       | [Binary RDF Representation for Publication and Exchange (HDT)](https://www.w3.org/Submission/HDT/)  | N/A                         |         |
| TBD       | [RDF Binary using Apache Thrift](https://afs.github.io/rdf-thrift/)                                 | `application/x-binary-rdf`  |         |
| TBD       | [JSON-LD 1.1](https://www.w3.org/TR/json-ld/); A JSON-based Serialization for Linked Data           | `application/ld+json`       |         |
| TBD       | [RDFa Core 1.1 - Third Edition](https://www.w3.org/TR/rdfa-core/)                                   | `text/html`                 |         |

Each module will also provide public constants `NAME`, `FILE_EXTENSION`, and
`MIME_TYPE`.

# Example

An example, reading an existing NTriple file.

```rust
use rdftk_io::nt::reader::NTriplesReader;
use rdftk_io::GraphReader;
use rdftk_memgraph::simple::graph_factory;
use std::fs::File;
use std::path::PathBuf;

let file_path = PathBuf::from("tests/w3c/nt/literal.nt");
let mut file = File::open(file_path).unwrap();
let reader = NTriplesReader::default();
let graph = reader.read(&mut file, graph_factory()).unwrap();
```

## Changes

### Version 0.3.2

* Feature: added `GraphWriter` and `DataSetWriter` traits so that clients need not
  import `objio`.

### Version 0.3.1

* Feature: moved to new v0.5 core package.
* Tests: all tests now passing.

### Version 0.3.0

* Feature: moved to new `rdftk_core` package.
* Refactor: moved reader/writer traits to new `objio` crate.
* Build: updated Rust edition from 2018 to 2021.
* Build: cargo audit/outdated/udeps
* Docs: added API docs as much as possible.
* Docs: run markuplint on `README.md`.

### Version 0.2.1

* Added JSON reader.

### Version 0.2.0

* Updated rdftk_core dependency to 0.3.0, this is a significant API change.
* All read operations require a graph factory now.

### Version 0.1.9

* Added unicode escape handling into the IRI parsing.
* All W3C test cases for NTriples now passing.
* Removed local error module, using core error types now.

### Version 0.1.8

* Using new style interfaces, a consistent use of traits and trait reference
  types.
* Have a working NTriple parser.

### Version 0.1.7

* Using rdftk_core 0.2, this has changes in the signature of both Graph and
  DataSet traits.
  * This results in changes to both GraphWriter and DataSetWriter traits.
* Applied a lot more warnings in lib.rs
* Fixed resulting Clippy suggestions.

### Version 0.1.6

* Made all modules have separate reader/writer sub-modules.
* Put all modules behind features.
* Added JSON writer.
* Re-write NQuad writer to write datasets.
* Re-wrote NTripe writer to use NQuad writer.
* Fixed formatting in Turtle writer.
* Added XML writer.

### Version 0.1.5

* Internal change to use `StatementRef`.

### Version 0.1.4

* API changes in core crate:
  * Altered `PrefixMappings::compress` and `PrefixMappings::expand` to take
    references.

### Version 0.1.2

* Made all `IRI` into `IRIRef`.

### Version 0.1.1

* Made all local dependencies only major/minor valued.

### Version 0.1.0

* First release.
* Provides write support only for N-Triples, N-Quads, and GraphViz.

## TODO

1. The core; N-Triples, N-Quads, N3, and Turtle need read and write support.
2. The extended core; RDF/XML, JSON-LD, and RDFa need read and write support.
3. The rest; RDF/JSON, TriG, HDT, and BinaryRDF will be implemended as needed.

[![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
[![RDFa](https://www.w3.org/Icons/SW/Buttons/sw-rdfa-blue.png)](http://www.w3.org/2001/sw/wiki/RDFa)
