# RDFtk: Names

This crate provides a set of modules that contain the URIs and QName strings for commonly used vocabularies. It also 
provides macro support for defining new namespaces in the same style as this library.

[![crates.io](https://img.shields.io/crates/v/rdftk_names.svg)](https://crates.io/crates/rdftk_names)
[![docs.rs](https://docs.rs/rdftk_names/badge.svg)](https://docs.rs/rdftk_names)

## Vocabularies

* `dc`: Dublin Core Metadata; [dublincore.org](https://www.dublincore.org/), includ
  * `dcam`
  * `dcmi_types`
  * `elements`
  * `terms`
* `foaf`: Friend of a Friend; [specification](http://xmlns.com/foaf/spec/)
* `geo`: W3C Geographic
* `owl`: Web Ontology Language [![OWL](https://www.w3.org/Icons/SW/Buttons/sw-owl-blue.png)](http://www.w3.org/2001/sw/wiki/OWL)
* `rdf`: RDF Syntax [![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
* `rdfs`: RDF Schema [![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
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

* SKOS, `http://www.w3.org/2004/02/skos/core#`; [![SKOS](https://www.w3.org/Icons/SW/Buttons/sw-skos-blue.png)](http://www.w3.org/2001/sw/wiki/SKOS)
* PROV, `http://www.w3.org/ns/prov#`; [![PROV](https://www.w3.org/Icons/SW/Buttons/sw-prov-blue.png)](http://www.w3.org/2001/sw/wiki/PROV)
* RDFa, `tp://www.w3.org/ns/rdfa#`;  [![RDFa](https://www.w3.org/Icons/SW/Buttons/sw-rdfa-blue.png)](http://www.w3.org/2001/sw/wiki/RDFa)
* org, `http://www.w3.org/ns/org#`; https://www.w3.org/TR/vocab-org/
* vcard, `http://www.w3.org/2006/vcard/ns#`; https://www.w3.org/TR/vcard-rdf/
* CC, `http://creativecommons.org/ns#`; https://wiki.creativecommons.org/wiki/CC_REL
