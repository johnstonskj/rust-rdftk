# RDFtk: MemGraph

An implementation of the `Graph` traits for simple in-memory cases.

[![crates.io](https://img.shields.io/crates/v/rdftk_memgraph.svg)](https://crates.io/crates/rdftk_memgraph)
[![docs.rs](https://docs.rs/rdftk_memgraph/badge.svg)](https://docs.rs/rdftk_memgraph)

## Changes

**Version 0.1.5**

* API changes in core crate:
  * Split `Graph` into `Graph` and `MutableGraph`.
  * Split `NamedGraph` into `NamedGraph` and `MutableNamedGraph`.
  * Altered `PrefixMappings::compress` and `PrefixMappings::expand` to take references.
  
**Version 0.1.4**

* API changes in IRI crate.

**Version 0.1.3**

* Explicit version management.

**Version 0.1.2**

* Made all `IRI. into `IRIRef`.

**Version 0.1.1**

* Made all local dependencies only major/minor valued.

**Version 0.1.0**

* First release

## TODO

TBD 

[![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
