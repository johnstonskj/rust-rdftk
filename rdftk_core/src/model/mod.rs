/*!
This is the root of the RDF data model implementation.

This implementation contains a full stack of components across multiple specifications:

1. **DataSet** a container of named graphs and optionally one unnamed (the default) graph.
1. **Graph** an optionally named container of statements.
1. **Statement** the core triple of subject, predicate (IRI), object.
1. **SubjectNode** either an IRI, a blank node, or a nested statement.
1. **ObjectNode** either an IRI, a blank node, a nested statement, or a literal.
1. **BlankNode** an anonymous subject or object.
1. **Literal** values that are the object of a statement.

*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod data_set;

pub mod features;

pub mod graph;

pub mod literal;

pub mod statement;

pub mod resource;
