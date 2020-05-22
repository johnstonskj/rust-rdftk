# Crate rdfktk_names

This crate provides a set of modules that contain the URIs and QName strings for commonly used vocabularies. It also 
provides macro support for defining new namespaces in the same style as this library.

[![crates.io](https://img.shields.io/crates/v/rdftk_names.svg)](https://crates.io/crates/rdftk_names)
[![docs.rs](https://docs.rs/rdftk_names/badge.svg)](https://docs.rs/rdftk_names)

## Vocabularies

* `dc`: Dublin Core Metadata
* `foaf`: Friend of a Friend
* `geo`: W3C Geographic
* `owl`: Web Ontology Language
* `rdf`: RDF Syntax
* `rdfs`: RDF Schema
* `xsd`: XML Schema data types

# Example

The following example replicates the `geo` module using the `namespace!` macro.

```rust
#[macro_use]
extern crate rdftk_names;

namespace! {
    "geo",
    "http://www.w3.org/2003/01/geo/wgs84_pos#",
    {
        spatial_thing, "SpatialThing",
        temporal_thing, "TemporalThing",
        event, "Event",
        point, "Point",
        lat, "lat",
        location, "location",
        long, "long",
        alt, "alt",
        lat_long, "lat_long"
    }
}
```

## Changes

**Version 0.1.0**

* First release

## TODO

* SKOS, `http://www.w3.org/2004/02/skos/core#`
* PROV, `http://www.w3.org/ns/prov#`
* RDFa, `tp://www.w3.org/ns/rdfa#`
* org, `http://www.w3.org/ns/org#`
* gldp, `http://www.w3.org/ns/people#`
* vcard, `http://www.w3.org/2006/vcard/ns#`
* CC, `http://creativecommons.org/ns#`