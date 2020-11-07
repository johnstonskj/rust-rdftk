# RDFtk: SKOS

A data model with RDF support for the Simple Knowledge Organization System (SKOS) vocabulary.

[![crates.io](https://img.shields.io/crates/v/rdftk_skos.svg)](https://crates.io/crates/rdftk_skos)
[![docs.rs](https://docs.rs/rdftk_skos/badge.svg)](https://docs.rs/rdftk_skos)

## Example

TBD

## Changes

**Version 0.1.15**

* Updated the `somedoc` crate version.

**Version 0.1.14**

* Updated the `somedoc` crate version.

**Version 0.1.14**

* Moved to use the `somedoc` crate for document generation.

**Version 0.1.13**

* Only show "jump to collections" if there are collections.
* Fixed `fmt` issues.
* Updated [paste](https://crates.io/crates/paste) dependency.

**Version 0.1.12**

* Enhancement; refactored traits, use `Resource` for `Named`, all resources are both _labeled_ and _propertied_.
* Enhancement; copying constructors from `LiteralProperty` as methods on `Resource`.

**Version 0.1.11**

* Enhancement; added logic to pull `skos:definition` property and display as text for any resource.

**Version 0.1.10**

* Fixed bug in jump-to links; an unwanted trailing ']' character.
* Fixed bug in tree generation; it was following non-narrower relationships.

**Version 0.1.9**

* Enhancement; added links at the top to jump down to different sections.

**Version 0.1.8**

* Fixed bug in `label_to_fragment`; trim label string to get rid of trailing '-'.
* Enhancement; make instances italic in concept tree.

**Version 0.1.7**

* Fixed markdown generation of fragment links.
* Fixed inverse relationship display for `ConceptRelation::Related`.

**Version 0.1.6**

* Explicit version management.

**Version 0.1.5**

* `model` module now renamed `simple` and it follows a more strict hierarchy approach.
* More performance work, but more to do.

**Version 0.1.4**

* Using new `IRIRef` approach for all crates.

**Version 0.1.3**

* Added relationship kinds from the ISO schema.

**Version 0.1.2**

* Added new `markdown::write_concept_tree_markdown`.

**Version 0.1.1**

* Added `is_top_concept` method to `Scheme`.
* Added "Top" indicator to concepts in markdown.
* Added concept tree to markdown.

**Version 0.1.0**

* First release.

## TODO

1. Performance!!!

[![SKOS](https://www.w3.org/Icons/SW/Buttons/sw-skos-blue.png)](http://www.w3.org/2001/sw/wiki/SKOS)
