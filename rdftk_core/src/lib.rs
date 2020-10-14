/*!
The core data model; concrete implementations for `Statement`s and `Literal`s, along with a
concrete `Resource` type that provides a builder-like experience for models.

# Example

```rust
use rdftk_core::{Literal, Statement, SubjectNode};
use rdftk_iri::IRI;
use std::rc::Rc;
use std::str::FromStr;

let mut statements: Vec<Rc<Statement>> = Default::default();

statements.push(Rc::new(Statement::new(
    SubjectNode::named(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap().into()),
    IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap().into(),
    Literal::new("Tony Benn").into(),
)));
```

*/

#[macro_use]
extern crate error_chain;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod graph;
pub use graph::Graph;

pub mod literal;
pub use literal::*;

pub mod qname;
pub use qname::*;

pub mod resource;
pub use resource::*;

pub mod statement;
pub use statement::*;
