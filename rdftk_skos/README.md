# RDFtk: SKOS

![skos](https://img.shields.io/badge/RDFtk-skos-BD1B89?logo=data:image/png;base64,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) 
This crate provides a data model with RDF support for the Simple Knowledge Organization System (SKOS) vocabulary.

[![crates.io](https://img.shields.io/crates/v/rdftk_skos.svg)](https://crates.io/crates/rdftk_skos)
[![docs.rs](https://docs.rs/rdftk_skos/badge.svg)](https://docs.rs/rdftk_skos)

## Example

TBD

## Changes

**Version 0.1.29**

* More core API changes.

**Version 0.1.28**

* Using rdftk_core 0.2, this has changes in the signature of both Graph and DataSet traits.
* Applied a lot more warnings in lib.rs
* Fixed resulting Clippy suggestions.

**Version 0.1.27**

* Added support for the term_status namespace.

**Version 0.1.25**

* Responding to module name changes in IO.

**Version 0.1.25**

* Added: a feature to wrap the documentation writer.
* Refactor: renamed module `simple` to `model`.
* Dependency: updated `somedoc` dependency; this had breaking API changes.

**Version 0.1.24**

* De-dup the flattened list of concepts before documenting.

**Version 0.1.23**

* Added `to_rdf_graph_with_mappings` to allow for the mappings in `make_document_with_mappings` to flow through.

**Version 0.1.22**

* Added "_external relations_" to `Concept::to_statements`.

**Version 0.1.21**

* Added the notion of "_external relations_" to a concept.
* Changed signatures on the document writer functions.
* API changes in core crate:
  * Altered `PrefixMappings::compress` and `PrefixMappings::expand` to take references.

**Version 0.1.20**

* upgraded `somedoc` dependency.

**Version 0.1.19**

* Fixed a bug in creating concept trees, links were going to "Collection:", not "Concept".

**Version 0.1.18**

* No longer try to format anchor text, `somedoc` does it now.

**Version 0.1.17**

* Moved from `CodeBlock` to `Formatted`, XWiki is unpleasant when it doesn't know a language.
* Fixed a bug where "|" was written twice if no collections present.
* Fixed a bug where the heading for the code block was "Collections", not "Appendix - RDF".

**Version 0.1.16**

* Fixed bug in document; highlighting the correct language.

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
