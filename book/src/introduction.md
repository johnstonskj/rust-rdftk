# Introduction

The RDF toolkit for Rust is a set of crates providing the ability to work with RDF data. The goal is to provide a consistent set of tools for reading and writing files, manipulating models programmatically, and working with graph (triple) stores.


## Goals

1. **Usability**
   1. Avoid lifetimes in public structures
   1. Consistency
   1. As Rust-like as possible
1. **Correctness** Sensible standards conformance
1. **Flexibility**

## Not Goals

1. **Performance**; in terms of implementation, the primary goals of the RDFtk project is to provide a consistent and complete set of crates for handling RDF data. To this end the crates will value readability and usability over runtime optimization (either memory, or speed) at this time. Getting the right interface will be the primary aim with those optimizations coming as necessary.
1. **Productization**
