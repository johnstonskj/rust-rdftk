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

* [`Core`](./rdftk_core): The core data model, concrete implementations for `Statement`s and `Literal`s, along with a concrete
  `Resource` type that provides a builder-like experience for models. 
* [`Graph`](./rdftk_graph): Traits that describe the behavior of `Graph`s, these are implemented by different strategies such as `memgraph`.
* [`IO`](./rdftk_io): Traits for reading/writing statements and graphs as well as implementations for several common formats.
* [`IRI`](./rdftk_iri): An implementation of a true IRI type.
* [`MemGraph`](./rdftk_memgraph): A simple in-memory graph.
* [`Names`](./rdftk_names): Common vocabularies.
* [`Query`](./rdftk_query): Placeholder for query APIs and SPARQL support.

The following diagram shows the crate dependencies, but for clarify it has combined `core` and `graph` into a single 
unit.

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
   │    ┌──────────────┐           ┌──────────────┐
   │    │      io      │           │    query     │
   │    └──────────────┘           └──────────────┘
   │                                        │
   └────────────────────────────────────────┘
```

[![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
[![OWL](https://www.w3.org/Icons/SW/Buttons/sw-owl-blue.png)](http://www.w3.org/2001/sw/wiki/OWL)
[![RDFa](https://www.w3.org/Icons/SW/Buttons/sw-rdfa-blue.png)](http://www.w3.org/2001/sw/wiki/RDFa)
[![SKOS](https://www.w3.org/Icons/SW/Buttons/sw-skos-blue.png)](http://www.w3.org/2001/sw/wiki/SKOS)
[![PROV](https://www.w3.org/Icons/SW/Buttons/sw-prov-blue.png)](http://www.w3.org/2001/sw/wiki/PROV)
[![SPARQL](https://www.w3.org/Icons/SW/Buttons/sw-sparql-blue.png)](http://www.w3.org/2001/sw/wiki/SPARQL/)


[![N-Triples](https://img.shields.io/badge/RDF-N--Triples-blue)](https://www.w3.org/TR/n-triples/)
[![N-Quads](https://img.shields.io/badge/RDF-N--Quads-blue)](https://www.w3.org/TR/n-quads/)
[![N3](https://img.shields.io/badge/RDF-N3-blue)](https://www.w3.org/TeamSubmission/n3/)
[![Turtle](https://img.shields.io/badge/RDF-Turtle-blue)](https://www.w3.org/TR/turtle/)

<div style="font-size: smaller">
_All usage of the W3C Semantic Web Technology Buttons are in accordance with https://www.w3.org/2007/10/sw-logos.html_.
</div>