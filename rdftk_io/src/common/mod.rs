/*!
One-line description.

More detailed description, with

# Example

*/

use rdftk_core::{model::graph::GraphFactoryRef, simple::graph_factory};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ReaderOptions {
    factory: GraphFactoryRef,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ReaderOptions {
    fn default() -> Self {
        Self {
            factory: graph_factory(),
        }
    }
}

impl ReaderOptions {
    pub fn with_factory(self, factory: GraphFactoryRef) -> Self {
        let mut self_mut = self;
        self_mut.factory = factory;
        self_mut
    }

    pub fn set_factory(&mut self, factory: GraphFactoryRef) {
        self.factory = factory;
    }

    pub fn factory(&self) -> &GraphFactoryRef {
        &self.factory
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub(crate) mod indenter;

#[macro_use]
pub(crate) mod parser_error;

pub(crate) mod parser;
