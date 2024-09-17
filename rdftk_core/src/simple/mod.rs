/*!
This module contains the types implementing the abstract RDF model described in
[crate::model](../model/index.html).
*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The identifier for the simple model provider.
///
pub const PROVIDER_ID: &str = concat!(
    env!("CARGO_CRATE_NAME"),
    "::",
    module_path!(),
    "@",
    env!("CARGO_PKG_VERSION")
);

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod data_set;
pub use data_set::data_set_factory;

pub mod graph;
pub use graph::graph_factory;

pub mod indexed;

pub mod literal;

pub mod mapping;

pub mod resource;

pub mod statement;
