/*!
Traits which describe the core capabilities of a graph. Note that this crate does not provide an
implementation of these traits as they are very dependent on their usage for performance, and
any backing storage.

# Example

```rust
use rdftk_core::model::graph::Graph;
use rdftk_core::model::statement::Statement;

fn simple_graph_writer<G: Graph>(graph: &G)
where
    <G as Graph>::Statement: std::fmt::Display,
{
    for statement in graph.statements() {
        println!("{}", statement);
    }
}
```
*/

use crate::error::Error;
use crate::model::features::Featured;
use crate::model::statement::{BlankNode, ObjectNode, Statement, SubjectNode};
use crate::model::Provided;
use rdftk_iri::IriExtra;
use rdftk_iri::{Iri, Name};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Graph Names
// ------------------------------------------------------------------------------------------------

///
/// This type denotes the identifier for a graph in a data set; a graph name MUST be either an Iri
/// or a blank node.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GraphName {
    BNode(BlankNode),
    Iri(Iri),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Graphs
// ------------------------------------------------------------------------------------------------

///
/// A graph is an unordered list of statements and may include duplicates.
/// Note that this trait represents an immutable graph, a type should also implement the
/// `MutableGraph` trait for mutation.
///
pub trait Graph: Debug + Featured {
    type Literal: Literal + Eq + Hash;
    type Statement: Statement<Literal = Self::Literal> + Eq + Hash;

    ///
    /// Returns `true` if there are no statements in this graph, else `false`.
    ///
    fn is_empty(&self) -> bool;

    ///
    /// Return the number of statements in this graph.
    ///
    fn len(&self) -> usize;

    // --------------------------------------------------------------------------------------------
    // Name
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if this graph instance has a name.
    ///
    fn is_named(&self) -> bool {
        self.name().is_some()
    }

    ///
    /// Return the name of this graph.
    ///
    fn name(&self) -> Option<&GraphName>;

    ///
    /// Set the name of this graph.
    ///
    fn set_name(&mut self, name: GraphName);

    ///
    /// Remove the name of this graph.
    fn unset_name(&mut self);

    // --------------------------------------------------------------------------------------------
    // Query
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if this graph contains any statement with the provided subject, else `false`.
    ///
    fn contains_subject(&self, subject: &SubjectNode<Self::Literal, Self::Statement>) -> bool;

    ///
    /// Returns `true` if this graph contains the provided statement, else `false`.
    ///
    fn contains(&self, statement: &Self::Statement) -> bool {
        !self
            .matches(
                Some(statement.subject()),
                Some(statement.predicate()),
                Some(statement.object()),
            )
            .is_empty()
    }

    ///
    /// Returns `true` if this graph contains the any statement with the provided subject,
    /// predicate, and object, else `false`.
    ///
    fn contains_all(
        &self,
        subject: &SubjectNode<Self::Literal, Self::Statement>,
        predicate: &Iri,
        object: &ObjectNode<Self::Literal, Self::Statement>,
    ) -> bool {
        !self
            .matches(Some(subject), Some(predicate), Some(object))
            .is_empty()
    }

    ///
    /// Returns `true` if this graph contains the any statement with the provided subject,
    /// predicate, and object, else `false`.
    ///
    fn matches(
        &self,
        subject: Option<&SubjectNode<Self::Literal, Self::Statement>>,
        predicate: Option<&Iri>,
        object: Option<&ObjectNode<Self::Literal, Self::Statement>>,
    ) -> HashSet<&Self::Statement>;

    // --------------------------------------------------------------------------------------------
    // Iterators
    // --------------------------------------------------------------------------------------------

    ///
    /// Return an iterator over all the statements in the graph.
    ///
    fn statements(&self) -> impl Iterator<Item = &Self::Statement>;

    ///
    /// Return a set of all subjects in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    fn subjects(&self) -> HashSet<&SubjectNode<Self::Literal, Self::Statement>>;

    ///
    /// Return a set of all subjects that are not blank nodes
    ///
    fn node_subjects(&self) -> HashSet<&SubjectNode<Self::Literal, Self::Statement>> {
        self.subjects()
            .into_iter()
            .filter(|s| !s.is_blank())
            .collect()
    }

    ///
    /// Return a set of all subjects that are blank nodes
    ///
    fn blank_node_subjects(&self) -> HashSet<&SubjectNode<Self::Literal, Self::Statement>> {
        self.subjects()
            .into_iter()
            .filter(|s| s.is_blank())
            .collect()
    }

    ///
    /// Return a set of all predicate in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    fn predicates(&self) -> HashSet<&Iri>;

    ///
    /// Return a set of all predicate referenced by the provided subject in graph, note that
    /// this is a set so that it removes duplicates.
    ///
    fn predicates_for(
        &self,
        subject: &SubjectNode<Self::Literal, Self::Statement>,
    ) -> HashSet<&Iri>;

    ///
    /// Return a set of all objects in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    fn objects(&self) -> HashSet<&ObjectNode<Self::Literal, Self::Statement>>;

    ///
    /// Return a set of all objects referenced by the provided subject and predicate in the graph,
    /// note that this is a set so that it removes duplicates.
    ///
    fn objects_for(
        &self,
        subject: &SubjectNode<Self::Literal, Self::Statement>,
        predicate: &Iri,
    ) -> HashSet<&ObjectNode<Self::Literal, Self::Statement>>;

    // --------------------------------------------------------------------------------------------
    // Namespace Management
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns the set of prefix mappings held by the graph.
    ///
    fn prefix_mappings(&self) -> PrefixMapping;

    ///
    /// Set the prefix mappings held by the graph.
    ///
    fn set_prefix_mappings(&mut self, mappings: PrefixMapping);

    // --------------------------------------------------------------------------------------------
    // Mutators
    // --------------------------------------------------------------------------------------------

    ///
    /// Return an iterator over all the statements in the graph.
    ///
    fn statements_mut(&mut self) -> impl Iterator<Item = &mut Self::Statement>;

    ///
    /// Insert a new statement into the graph.
    ///
    fn insert(&mut self, statement: Self::Statement);

    ///
    /// Merge another graph into this one. Note that the graphs are required to have the same
    /// implementation type based in the type qualifiers for `StatementIter`.
    ///
    fn merge(&mut self, other: &Self)
    where
        Self: Sized;

    ///
    /// Remove any duplicates within the graph, replacing any number of identical statements with
    /// just one. This will return a list of all statements removed.
    ///
    /// This method does nothing if this graph has does not support the feature
    /// `FEATURE_GRAPH_DUPLICATES` and will therefore always return an empty list.
    ///
    fn dedup(&mut self) -> Vec<Self::Statement>;

    ///
    /// Remove any statement that matches the provided. If a graph has duplicates this method does
    /// not differentiate between them.
    ///
    fn remove(&mut self, statement: &Self::Statement);

    ///
    /// Remove all statements from this graph that have the provided subject.
    ///
    fn remove_all_for(
        &mut self,
        subject: &SubjectNode<Self::Literal, Self::Statement>,
    ) -> Vec<Self::Statement>;

    ///
    /// Remove all statements from this graph.
    ///
    fn clear(&mut self);

    ///
    /// Replace all blank nodes with new, unique Iris. This creates a new graph and leaves the initial
    /// graph unchanged. The base Iri is used to create identifiers, it's path will be replaced
    /// entirely by a well-known format.
    ///
    /// For example, given the following input graph with blank nodes:
    ///
    /// ```ttl
    /// <https://example.org/p/me> <https://example.org/v/name> _:B0f21 .
    /// _:B0f21 <https://example.org/v/firstName> "My" .
    /// _:B0f21 <https://example.org/v/lastName> "Name" .
    /// ```
    ///
    /// the call to `skolemize`,
    ///
    /// ```rust,ignore
    /// let base = Iri::from_str("https://example.com/me").unwrap();
    /// graph.skolemize(&base)
    /// ```
    ///
    /// results in a new graph containing replacement IRIs.
    ///
    /// ```ttl
    /// <https://example.org/p/me>
    ///   <https://example.org/v/name>
    ///   <https://example.com/.well-known/genid/62D22842-0D24-4911-AE7D-DF4DE06FD62F> .
    /// <https://example.com/.well-known/genid/62D22842-0D24-4911-AE7D-DF4DE06FD62F>
    ///   <https://example.org/v/firstName>
    ///   "My" .
    /// <https://example.com/.well-known/genid/62D22842-0D24-4911-AE7D-DF4DE06FD62F>
    ///   <https://example.org/v/lastName>
    ///   "Name" .
    /// ```
    fn skolemize<F1, F2>(
        self,
        base: &Iri,
        graph_factory: &F1,
        statement_factory: &F2,
    ) -> Result<Self, Error>
    where
        F1: GraphFactory<Literal = Self::Literal, Statement = Self::Statement, Graph = Self>,
        F2: StatementFactory<Literal = Self::Literal, Statement = Self::Statement>,
        Self: Sized,
    {
        let mut mapping: HashMap<BlankNode, Iri> = Default::default();

        let mut new_graph = graph_factory.graph();

        for statement in self.statements() {
            let mut new_statement = statement.clone();
            if let Some(blank) = new_statement.subject().as_blank() {
                if !mapping.contains_key(blank) {
                    let _ = mapping.insert(blank.clone(), base.genid()?);
                }
                let name = mapping.get(blank).unwrap().clone();
                let subject = statement_factory.named_subject(name);
                new_statement.set_subject(subject);
            }
            if let Some(blank) = new_statement.object().as_blank() {
                if !mapping.contains_key(blank) {
                    let _ = mapping.insert(blank.clone(), base.genid()?);
                }
                let name = mapping.get(blank).unwrap().clone();
                let object = statement_factory.named_object(name);
                new_statement.set_object(object);
            }
            new_graph.insert(new_statement);
        }

        Ok(new_graph)
    }
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Factories
// ------------------------------------------------------------------------------------------------

///
/// A graph factory provides an interface to create a new graph. This allows for implementations
/// where underlying shared resources are required and so may be owned by the factory.
///
/// The method for getting the initial factory instance is not specified here. By convention
/// implementors *may* provide a function `graph_factory` in the root module for their crate.
///
pub trait GraphFactory: Debug + Provided {
    type Literal: Literal;
    type Statement: Statement<Literal = Self::Literal>;
    type Graph: Graph<Statement = Self::Statement>;

    ///
    /// Create a new graph instance.
    ///
    fn graph(&self) -> Self::Graph;

    ///
    /// Create a new named graph instance.
    ///
    fn named_graph(&self, name: Option<GraphName>) -> Self::Graph;

    ///
    ///  Create a new graph instance from the given statements and prefix mappings.
    ///
    fn graph_from(
        &self,
        statements: &[Self::Statement],
        prefix_mappings: Option<PrefixMapping>,
    ) -> Self::Graph;

    ///
    ///  Create a new graph instance from the given statements and prefix mappings.
    ///
    fn named_graph_from(
        &self,
        name: Option<GraphName>,
        statements: &[Self::Statement],
        prefix_mappings: Option<PrefixMapping>,
    ) -> Self::Graph;
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Graph Names
// ------------------------------------------------------------------------------------------------

impl Display for GraphName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::BNode(node) => format!("_:{}", node),
                Self::Iri(iri) => format!("<{}>", iri),
            }
        )
    }
}

impl From<Name> for GraphName {
    fn from(name: Name) -> Self {
        Self::BNode(BlankNode::from(name))
    }
}

impl From<&Name> for GraphName {
    fn from(name: &Name) -> Self {
        Self::BNode(BlankNode::from(name))
    }
}

impl From<BlankNode> for GraphName {
    fn from(name: BlankNode) -> Self {
        Self::BNode(name)
    }
}

impl From<&BlankNode> for GraphName {
    fn from(name: &BlankNode) -> Self {
        Self::BNode(name.clone())
    }
}

impl From<Iri> for GraphName {
    fn from(name: Iri) -> Self {
        Self::Iri(name)
    }
}

impl From<&Iri> for GraphName {
    fn from(name: &Iri) -> Self {
        Self::Iri(name.clone())
    }
}

impl<L: Literal, T: Statement<Literal = L>> From<SubjectNode<L, T>> for GraphName {
    fn from(value: SubjectNode<L, T>) -> Self {
        match value {
            SubjectNode::Blank(v) => Self::BNode(v.clone()),
            SubjectNode::Resource(v) => Self::Iri(v.clone()),
            _ => unreachable!(),
        }
    }
}

impl GraphName {
    ///
    /// Construct a new graph name, as a blank node with a randomly assigned name.
    ///
    pub fn blank() -> Self {
        Self::BNode(BlankNode::generate())
    }

    ///
    /// Construct a new graph name, as a blank node with the specified name.
    ///
    pub fn blank_named<S>(name: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        Ok(Self::BNode(BlankNode::from_str(name.as_ref())?))
    }

    ///
    /// Construct a new graph name, with an Iri naming a resource.
    ///
    pub fn named(name: Iri) -> Self {
        Self::Iri(name)
    }

    ///
    /// Return `true` if this graph name is a blank node, else `false`.
    ///
    pub fn is_blank(&self) -> bool {
        matches!(self, Self::BNode(_))
    }

    ///
    /// Return a blank node string, if `self.is_blank()`, else `None`.
    ///
    pub fn as_blank(&self) -> Option<&BlankNode> {
        match &self {
            Self::BNode(s) => Some(s),
            _ => None,
        }
    }

    ///
    /// Return `true` if this graph name is an Iri, else `false`.
    ///
    pub fn is_iri(&self) -> bool {
        matches!(self, Self::Iri(_))
    }

    ///
    /// Return a named node Iri, if `self.is_iri()`, else `None`.
    ///
    pub fn as_iri(&self) -> Option<&Iri> {
        match &self {
            Self::Iri(u) => Some(u),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod mapping;
pub use mapping::PrefixMapping;

use super::literal::Literal;
use super::statement::StatementFactory;
