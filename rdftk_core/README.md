# RDFtk: Core

![core](https://img.shields.io/badge/RDFtk-core-BD1B89?logo=data:image/png;base64,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)
This crate provides an implementation of the RDF abstract syntax along with a `Resource` type that provides a builder-like experience for models.

[![crates.io](https://img.shields.io/crates/v/rdftk_core.svg)](https://crates.io/crates/rdftk_core)
[![docs.rs](https://docs.rs/rdftk_core/badge.svg)](https://docs.rs/rdftk_core)

From [RDF 1.1 Concepts and Abstract Syntax](https://www.w3.org/TR/rdf11-concepts/);

> The core structure of the abstract syntax is a set of triples, each consisting of a subject, a
> predicate and an object. A set of such triples is called an RDF graph. An RDF graph can be
> visualized as a node and directed-arc diagram, in which each triple is represented as a
> node-arc-node link.
>
> ![rdf-graph](https://raw.githubusercontent.com/johnstonskj/rust-rdftk/master/rdftk_core/doc/rdf-graph.svg)
>
> There can be three kinds of nodes in an RDF graph: IRIs, literals, and blank nodes.

In this library the triple, or statement, as well as subject, predicate, and object types are
in the module [`statement`](statement/index.html). Literal's as objects are supported in the
[`literal`](literal/index.html) module. Traits that describe graphs are provided by the
[`graph`](graph/index.html) module.

Additional features are provided such as support for data sets (module [`data_set`](data_set/index.html))
as well as support for extensions to the core RDF abstract model such as
[RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).

## Example

```rust
use rdftk_core::{Literal, Statement, StatementList, SubjectNode};
use rdftk_iri::IRI;
use std::rc::Rc;
use std::str::FromStr;

pub fn make_statements() -> StatementList {
    let mut statements: StatementList = Default::default();
    
    statements.push(Statement::new(
        SubjectNode::named(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()),
        IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap(),
        Literal::new("Tony Benn").into(),
    ).into());
    // ...
    statements
}
```

## Changes

**Version 0.2.4**

* Created new Featured trait implemented by a number of types that allow client query of various optional
  capabilities. This also subsumes the has_index/has_all_indices capability as all index queries are now
  feature queries.

**Version 0.2.3**

* Copied some errors from rdftk_io.
* Renamed the Io variant to ReadWrite.

**Version 0.2.2**

* Reworked APIs to take <name>Ref types and to be consistent in use of trait objects and types throughout.
* Added factory type for data sets.
* Made PrefixMappings a concrete type in the core::graph::mapping module.
* Added InvalidMatch and Io variants to ErrorKind.
* Added mutators to Statment.

**Version 0.2.1**

* Changed API, removed mutable traits for Graph and DataSet, moved methods into their base traits.
* Added factory types for graphs.
* Added Skolemization function for graphs.

**Version 0.2.0**

* A change to the API, all `Statement`, and statement components are now passed as `Rc` references.
  * Added additional *_ref* constructors to allow cleaner client code.
* A change to the API, `Graph` and `DataSet` now use type parameters to describe iterators returned by *query* methods.
* A change to the API, `QName` constructors now return errors instead of panic on invalid values.
* Added more constructors for literal values.
* Added support for `chrono::Duration` in literals as well as the std version as chrono supports the correct output form.
* Added `eq_` methods on `SubjectNode` and `ObjectNode` for simple testing of inner values.
* Added documentation and examples throughout.

**Version 0.1.15**

* Fixed Clippy suggestions.
* Removed Context from statements.
* Added value_factory method to Graph. 
* Placed all unit tests in tests folder.

**Version 0.1.14**

* Removed stand-alone named graph.
* Added DataSet as a way to associate names to graphs.
* Renamed CachingGraph to ValueFactory and made stand-alone.

**Version 0.1.13**

* Bug: fixed Literal constructors to produce an escape-safe literal form for strings.

**Version 0.1.12**

* Fixed: cargo fmt error.

**Version 0.1.11**

* Added: public types `StatementRef` and `StatementList` rather than having `Rc` obviously in all APIs.

**Version 0.1.10**

* **DEPRECATED** Support for [Datasets](https://www.w3.org/TR/rdf11-concepts/#section-dataset) and Quads by adding a context
  (type `ContextNode`) to `Statement`.

**Version 0.1.9**

* Support for [RDF*](https://w3c.github.io/rdf-star/) in `Statement`.
* Added additional constructors to `Statement`.
* Renamed Resource method `rdf_type` to `instance_of` for compatibility with RDF schema usage.
* Added `is_valid` associated function to `QName`.

**Version 0.1.8**

* Explicit exports over `pub use *`.

**Version 0.1.7**

* Split `Graph` into `Graph` and `MutableGraph`.
* Split `NamedGraph` into `NamedGraph` and `MutableNamedGraph`.
* Added `get_default_namespace` to the `PrefixMappings` trait as a helper function.
* Altered `PrefixMappings::compress` and `PrefixMappings::expand` to take references.

**Version 0.1.6**

* Explicit version management.

**Version 0.1.5**

* Updates for rdftk_memgraph to build.

**Version 0.1.4**

* Made all local dependencies only major/minor valued.

**Version 0.1.3**

* Moved all `IRI` to `IRIRef` on interfaces.
* Moved `Graph` and associated types into core and deprecated `rdftk_graph`.

**Version 0.1.2**

* Clean-up changes.

**Version 0.1.1**

* Added `From` to allow direct construction of a `SubjectNode` from an `IRI`.
* Fixed a bug in `QName` that dropped the ":" for non-prefixed values.
 
**Version 0.1.0**

* First release.

## TODO

TBD 

[![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
