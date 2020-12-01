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

[![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
[![OWL](https://www.w3.org/Icons/SW/Buttons/sw-owl-blue.png)](http://www.w3.org/2001/sw/wiki/OWL)
[![SPARQL](https://www.w3.org/Icons/SW/Buttons/sw-sparql-blue.png)](http://www.w3.org/2001/sw/wiki/SPARQL/) | 
[![PROV](https://www.w3.org/Icons/SW/Buttons/sw-prov-blue.png)](http://www.w3.org/2001/sw/wiki/PROV)
[![RIF](https://www.w3.org/Icons/SW/Buttons/sw-rif-blue.png)](http://www.w3.org/2001/sw/wiki/RIF)
[![SKOS](https://www.w3.org/Icons/SW/Buttons/sw-skos-blue.png)](http://www.w3.org/2001/sw/wiki/SKOS)

<div style="font-size: small; font-style: italic;">

All usage of the W3C Semantic Web Technology Buttons are in accordance with [W3C Semantic Web Logos and Policies](https://www.w3.org/2007/10/sw-logos.html).

</div>
