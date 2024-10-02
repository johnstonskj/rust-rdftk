/*!
This module contains the types implementing the abstract RDF model described in
[crate::model](../model/index.html).
*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use std::marker::PhantomData;

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

#[derive(Clone, Debug, Default)]
pub struct Implementation {
    private: PhantomData<u8>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl crate::model::Implementation for Implementation {
    type Literal = literal::SimpleLiteral;
    type Statement = statement::SimpleStatement;
    type Graph = graph::SimpleGraph;
    type DataSet = data_set::SimpleDataSet;

    fn data_set_factory(
        &self,
    ) -> &impl crate::model::data_set::DataSetFactory<Graph = Self::Graph, DataSet = Self::DataSet>
    {
        const FACTORY: data_set::SimpleDataSetFactory = data_set::SimpleDataSetFactory {};
        &FACTORY
    }

    fn graph_factory(
        &self,
    ) -> &impl crate::model::graph::GraphFactory<
        Literal = Self::Literal,
        Statement = Self::Statement,
        Graph = Self::Graph,
    > {
        const FACTORY: graph::SimpleGraphFactory = graph::SimpleGraphFactory {};
        &FACTORY
    }

    fn statement_factory(
        &self,
    ) -> &impl crate::model::statement::StatementFactory<
        Literal = Self::Literal,
        Statement = Self::Statement,
    > {
        const FACTORY: statement::SimpleStatementFactory = statement::SimpleStatementFactory {};
        &FACTORY
    }

    fn literal_factory(
        &self,
    ) -> &impl crate::model::literal::LiteralFactory<Literal = Self::Literal> {
        const FACTORY: literal::SimpleLiteralFactory = literal::SimpleLiteralFactory {};
        &FACTORY
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod data_set;

pub mod graph;

pub mod indexed;

pub mod literal;

pub mod resource;

pub mod statement;
