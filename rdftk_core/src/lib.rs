/*!
The core data model; concrete implementations for `Statement`s and `Literal`s, along with a 
concrete `Resource` type that provides a builder-like experience for models.

# Example

TBD

*/

#[macro_use]
extern crate error_chain;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod literal;
pub use literal::*;

pub mod qname;
pub use qname::*;

pub mod resource;
pub use resource::*;

pub mod statement;
pub use statement::*;
