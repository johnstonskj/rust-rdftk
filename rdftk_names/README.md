# RDFtk: Names

This crate provides a set of modules that contain the IRIs and QName strings for commonly used vocabularies. It also 
provides macro support for defining new namespaces in the same style as this library.

[![crates.io](https://img.shields.io/crates/v/rdftk_names.svg)](https://crates.io/crates/rdftk_names)
[![docs.rs](https://docs.rs/rdftk_names/badge.svg)](https://docs.rs/rdftk_names)

## Vocabularies

Te following table shows the set of namespaces supported, those with "TBD" in the module column are yet to be encoded.

| Module           | Vocabulary | Namespace |
|------------------|------------|-----------|
| `dt::dcam`       | [DCMI Abstract Model](https://www.dublincore.org/specifications/dublin-core/abstract-model/) | `http://purl.org/dc/dcam/` |
| `dt::dcmi_types` | [DCMI Type Vocabulary](https://www.dublincore.org/specifications/dublin-core/dcmi-type-vocabulary/) | `http://purl.org/dc/dcmitype/` |
| `dt::elements`'  | [DCMI Terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/) | `http://purl.org/dc/elements/1.1/` |
| `dt::terms`      | [DCMI Terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/) legacy elements | `http://purl.org/dc/terms/` |
| `foaf`           | [Friend of a Friend](http://xmlns.com/foaf/spec/) | `http://xmlns.com/foaf/0.1/` |
| `geo`            | [Basic Geo Vocabulary](https://www.w3.org/2003/01/geo/) | `http://www.w3.org/2003/01/geo/wgs84_pos#` |
| `owl`            | [![OWL](https://www.w3.org/Icons/SW/Buttons/sw-owl-blue.png)](http://www.w3.org/2001/sw/wiki/OWL) Web Ontology Language  | `http://www.w3.org/2002/07/owl#` |
| `rdf`            | [![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF) RDF Syntax  | `http://www.w3.org/1999/02/22-rdf-syntax-ns#` |
| `rdfs`           | [![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF) RDF Schema  | `http://www.w3.org/2000/01/rdf-schema#` |
| `xsd`            | [XML Schema data types](https://www.w3.org/TR/xmlschema-2) | `http://www.w3.org/2001/XMLSchema#` |
| TBD              | [![SKOS](https://www.w3.org/Icons/SW/Buttons/sw-skos-blue.png)](http://www.w3.org/2001/sw/wiki/SKOS) Simple Knowledge Organization System (SKOS) | `http://www.w3.org/2004/02/skos/core#` |
| TBD              | [![PROV](https://www.w3.org/Icons/SW/Buttons/sw-prov-blue.png)](http://www.w3.org/2001/sw/wiki/PROV) PROV | `http://www.w3.org/ns/prov#` |
| TBD              | [![RDFa](https://www.w3.org/Icons/SW/Buttons/sw-rdfa-blue.png)](http://www.w3.org/2001/sw/wiki/RDFa) RDF in Attributes (RDFa) | `http://www.w3.org/ns/rdfa#` |
| TBD              | [RDF Calendar](https://www.w3.org/TR/rdfcal/) | `http://www.w3.org/2002/12/cal#`  |
| TBD              | [vCard Ontology](https://www.w3.org/TR/vcard-rdf/) | `http://www.w3.org/2006/vcard/ns#` |
| TBD              | [The Organization Ontology](https://www.w3.org/TR/vocab-org/) | `http://www.w3.org/ns/org#` |
| TBD              | [Creative Commons Rights Expression Language](https://wiki.creativecommons.org/wiki/CC_REL) | `http://creativecommons.org/ns#` |

## The namespace Macro

The `namespace!` macro takes three parameters:

* The common prefix for the module,
* The namespace IRI for the module,
* A list of pairs where the first is the name of the function to return the IRI for the name, and the second is the 
  string name of the vocabulary element.
  
Note that as this macro uses `paste::item` the client will need to have a dependency on the [paste crate](https://crates.io/crates/paste), 
and a macro use statement in their code.

## Example

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

**Version 0.1.2**

* Added `rdf:li`.

**Version 0.1.1**

* Made `nsname!` macro public.

**Version 0.1.0**

* First release

## TODO

TBD