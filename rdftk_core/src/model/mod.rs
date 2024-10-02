/*!
This module contains the traits and types used to describe an abstract DataSet, Graph, and Statement
RDF model.
*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A trait implemented by types that are constructed by providers. This allows for providers to
/// ensure that values belong to them.
///
/// Note, by convention the fully qualified crate/module name is used as a provider name.
///
pub trait Provided {
    ///
    /// Return the identifier for the provider associated with this instance.
    ///
    fn provider_id(&self) -> &'static str;
}

pub trait Implementation {
    type Literal: literal::Literal;
    type Statement: statement::Statement<Literal = Self::Literal>;
    type Graph: graph::Graph<Literal = Self::Literal, Statement = Self::Statement>;
    type DataSet: data_set::DataSet<Graph = Self::Graph>;

    fn data_set_factory(
        &self,
    ) -> &impl data_set::DataSetFactory<Graph = Self::Graph, DataSet = Self::DataSet>;

    fn graph_factory(
        &self,
    ) -> &impl graph::GraphFactory<
        Literal = Self::Literal,
        Statement = Self::Statement,
        Graph = Self::Graph,
    >;

    fn statement_factory(
        &self,
    ) -> &impl statement::StatementFactory<Literal = Self::Literal, Statement = Self::Statement>;

    fn literal_factory(&self) -> &impl literal::LiteralFactory<Literal = Self::Literal>;
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod data_set;

pub mod features;

pub mod graph;

pub mod literal;

pub mod statement;
