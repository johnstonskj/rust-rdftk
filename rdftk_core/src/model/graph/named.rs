/*!
An extension to the core `Graph` to support named graphs. The semantics of a named graph has been
derived from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_, and
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/).
*/

use crate::model::graph::Graph;
use crate::model::statement::{BlankNode, BlankNodeRef};
use rdftk_iri::{Iri, IriRef, Name as NodeName};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Graph Names
// ------------------------------------------------------------------------------------------------

///
/// This type denotes the identifier for a graph in a data set; a graph name MUST be either an Iri
/// or a blank node.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphName(Name);

///
/// The actual graph name storage type, reference counted for memory management.
///
pub type GraphNameRef = Rc<GraphName>;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Named Graphs
// ------------------------------------------------------------------------------------------------

///
/// A named graph has an associated IRI or blank node that provided an identity for the graph
/// itself.
///
pub trait NamedGraph: Graph {
    ///
    /// Returns `true` if this graph instance has a name.
    ///
    fn is_named(&self) -> bool {
        self.name().is_some()
    }

    ///
    /// Return the name of this graph.
    ///
    fn name(&self) -> Option<&GraphNameRef>;

    ///
    /// Set the name of this graph.
    ///
    fn set_name(&mut self, name: GraphNameRef);

    ///
    /// Remove the name of this graph.
    fn unset_name(&mut self);
}

///
/// The actual object storage type, reference counted for memory management.
///
pub type NamedGraphRef = Rc<RefCell<dyn NamedGraph>>;

// ------------------------------------------------------------------------------------------------
// Private Types ❱ Graph Names
// ------------------------------------------------------------------------------------------------

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Name {
    BNode(BlankNodeRef),
    Iri(IriRef),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Graph Names
// ------------------------------------------------------------------------------------------------

impl Display for GraphName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.0 {
                Name::BNode(node) => format!("_:{}", node),
                Name::Iri(iri) => format!("<{}>", iri),
            }
        )
    }
}

impl From<NodeName> for GraphName {
    fn from(name: NodeName) -> Self {
        GraphName(Name::BNode(BlankNode::from(name).into()))
    }
}

impl From<&NodeName> for GraphName {
    fn from(name: &NodeName) -> Self {
        GraphName(Name::BNode(BlankNode::from(name).into()))
    }
}

impl From<BlankNode> for GraphName {
    fn from(name: BlankNode) -> Self {
        GraphName(Name::BNode(name.into()))
    }
}

impl From<BlankNodeRef> for GraphName {
    fn from(name: BlankNodeRef) -> Self {
        GraphName(Name::BNode(name))
    }
}

impl From<&BlankNodeRef> for GraphName {
    fn from(name: &BlankNodeRef) -> Self {
        GraphName(Name::BNode(name.clone()))
    }
}

impl From<Iri> for GraphName {
    fn from(name: Iri) -> Self {
        GraphName(Name::Iri(name.into()))
    }
}

impl From<IriRef> for GraphName {
    fn from(name: IriRef) -> Self {
        GraphName(Name::Iri(name))
    }
}

impl From<&IriRef> for GraphName {
    fn from(name: &IriRef) -> Self {
        GraphName(Name::Iri(name.clone()))
    }
}

impl GraphName {
    ///
    /// Construct a new graph name, as a blank node with a randomly assigned name.
    ///
    pub fn blank() -> Self {
        Self::from(BlankNode::generate())
    }

    ///
    /// Construct a new graph name, as a blank node with the specified name.
    ///
    pub fn blank_named(name: NodeName) -> Self {
        Self::from(name)
    }

    ///
    /// Construct a new graph name, with an Iri naming a resource.
    ///
    pub fn named(name: IriRef) -> Self {
        Self::from(name)
    }

    ///
    /// Return `true` if this graph name is a blank node, else `false`.
    ///
    pub fn is_blank(&self) -> bool {
        matches!(self.0, Name::BNode(_))
    }

    ///
    /// Return a blank node string, if `self.is_blank()`, else `None`.
    ///
    pub fn as_blank(&self) -> Option<&BlankNode> {
        match &self.0 {
            Name::BNode(s) => Some(s),
            _ => None,
        }
    }

    ///
    /// Return `true` if this graph name is an Iri, else `false`.
    ///
    pub fn is_iri(&self) -> bool {
        matches!(self.0, Name::Iri(_))
    }

    ///
    /// Return a named node Iri, if `self.is_iri()`, else `None`.
    ///
    pub fn as_iri(&self) -> Option<&IriRef> {
        match &self.0 {
            Name::Iri(u) => Some(u),
            _ => None,
        }
    }
}
