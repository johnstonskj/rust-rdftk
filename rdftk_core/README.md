# RDFtk: Core

The core data model; concrete implementations for `Statement`s and `Literal`s, along with a concrete `Resource` type 
that provides a builder-like experience for models.

[![crates.io](https://img.shields.io/crates/v/rdftk_core.svg)](https://crates.io/crates/rdftk_core)
[![docs.rs](https://docs.rs/rdftk_core/badge.svg)](https://docs.rs/rdftk_core)

## Example

```rust
use rdftk_core::{Literal, Statement, SubjectNode};
use rdftk_iri::IRI;
use std::rc::Rc;
use std::str::FromStr;

pub fn make_statements() -> Vec<Rc<Statement>> {
    let mut statements: Vec<Rc<Statement>> = Default::default();
    
    statements.push(Rc::new(Statement::new(
        SubjectNode::named(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()),
        IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap(),
        Literal::new("Tony Benn").into(),
    )));
    // ...
    statements
}
```

## Changes

**Version 0.1.0**

* First release.

## TODO

TBD 

[![RDF](https://www.w3.org/Icons/SW/Buttons/sw-rdf-blue.png)](http://www.w3.org/2001/sw/wiki/RDF)
