# RDFtk: An RDF Toolkit for Rust

The RDF toolkit for Rust is a set of crates providing the ability to work with RDF data. The goal is to provide a 
consistent set of tools for reading and writing files, manipulating models programmatically, and working with graph
(triple) stores.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![travis.ci](https://travis-ci.org/johnstonskj/rust-rdftk.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-rdftk)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-rdftk.svg)](https://github.com/johnstonskj/rust-rdftk/stargazers)

## Crates

The following set of crates are included in this repository (and Rust workspace), they are not yet complete and will 
probably be joined by others over time.

* `core`: The core data model, concrete implementations for `Statement`s and `Literal`s, along with a concrete
  `Resource` type that provides a builder-like experience for models. 
* `graph`: Traits that describe the behavior of `Graph`s, these are implemented by different strategies such as `memgraph`.
* `io`: Traits for reading/writing statements and graphs as well as implementations for several common formats.
* `iri`: An implementation of a true IRI type.
* `memgraph`: A simple in-memory graph.
* `names`: Common vocabularies.
* `query`: Placeholder for query APIs and SPARQL support.

```text
                     ┌──────────────┐
        ┌────────────│   memgraph   │─────────────┐
        │            └──────────────┘             │
        │                    │                    │
        │                    │                    │
        ▼                    ▼                    ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│     iri      │◀────│  core/graph  │────▶│    names     │
└──────────────┘     └──────────────┘     └──────────────┘
   ▲    ▲                    ▲                    ▲
   │    │                    │                    │
   │    └─────┬──────────────┴──────────────┬─────┘
   │          │                             │
   │          │                             │
   │  ┌──────────────┐              ┌──────────────┐
   │  │      io      │              │    query     │
   │  └──────────────┘              └──────────────┘
   │                                        │
   └────────────────────────────────────────┘
```

[![RDF](http://www.w3.org/RDF/icons/rdf_w3c_button.32)](http://www.w3.org/RDF/)
