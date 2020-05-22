/*!
One-line description.

More detailed description, with

# Example

*/

#[macro_use]
extern crate error_chain;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Normalize {
    fn normalize(self) -> error::Result<Self>
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod parse;

pub mod error;

pub mod builder;

pub mod scheme;
pub use scheme::*;

pub mod authority;
pub use authority::*;

pub mod path;
pub use path::*;

pub mod query;
pub use query::*;

pub mod fragment;
pub use fragment::*;

#[allow(clippy::module_inception)]
pub mod iri;
pub use iri::*;
