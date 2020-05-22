/*!
One-line description.

More detailed description, with

# Example

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
