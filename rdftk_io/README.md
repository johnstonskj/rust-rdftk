# RDFtk: IO

Traits for reading/wtiting `Statement`s and `Graph`s as well as implementations for common file formats.

[![crates.io](https://img.shields.io/crates/v/rdftk_io.svg)](https://crates.io/crates/rdftk_io)
[![docs.rs](https://docs.rs/rdftk_io/badge.svg)](https://docs.rs/rdftk_io)

The following are some well-known formats (see [Wikipedia](https://en.wikipedia.org/wiki/Resource_Description_Framework#Serialization_formats)
for a description of different serializations), support is indicated in the final column with
an **R** for read support and **W** for write support.

| Name          | MIME Type                                       | Specification | R/W |
|---------------|-------------------------------------------------|---------------|-----|
| [N-Triples](https://img.shields.io/badge/RDF-N--Triples-blue) | application/n-triples | [W3C](https://www.w3.org/TR/n-triples/)  | **W** |
| [N-Quads](https://img.shields.io/badge/RDF-N--Quads-blue)     | application/n-quads,  | [W3C](https://www.w3.org/TR/n-quads/)  |     |
| [N3](https://img.shields.io/badge/RDF-N3-blue)                | text/rdf+n3           | [W3C Submission](https://www.w3.org/TeamSubmission/n3/) |     |
| [Turtle](https://img.shields.io/badge/RDF-Turtle-blue)        | text/turtle           | [W3C](https://www.w3.org/TR/turtle/) |     |
| RDF/XML       | application/rdf+xml       | [W3C](https://www.w3.org/TR/rdf-syntax-grammar/) |     |
| JSON-LD       | application/ld+json       | [W3C](https://www.w3.org/TR/json-ld/)            |     |
| RDF/JSON      | application/rdf+json      | [W3C](https://www.w3.org/TR/rdf-json/)           |     |
| TriG          | application/trig          | [W3C](https://www.w3.org/TR/trig/)               |     |
| [RDFa](https://www.w3.org/Icons/SW/Buttons/sw-rdfa-blue.png)  | ?                            | [W3C](https://www.w3.org/TR/rdfa-core/) |     |
| HDT           | ?                         | [W3C Submission](https://www.w3.org/Submission/2011/SUBM-HDT-20110330/) |     |
| BinaryRDF     | application/x-binary-rdf  | [Community](https://afs.github.io/rdf-thrift/rdf-binary-thrift.html)    |     |

Each module will also provide public constants `NAME`, `FILE_EXTENSION`, and `MIME_TYPE`.

## Changes

**Version 0.1.0**

* First release

## TODO

TBD 

[![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
