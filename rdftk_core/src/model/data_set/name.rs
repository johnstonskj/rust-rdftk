/*!
An extension to the core `Graph` to support named graphs. The semantics of a named graph has been
derived from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_, and
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/).
*/

use rdftk_iri::{Iri, IriRef};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use unique_id::sequence::SequenceGenerator as IDGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This type denotes the identifier for a graph in a data set; a graph name MUST be either an Iri
/// or a blank node.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphName {
    inner: Name,
}

///
/// The actual graph name storage type, reference counted for memory management.
///
pub type GraphNameRef = Rc<GraphName>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Name {
    BNode(String),
    Iri(IriRef),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for GraphName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.inner {
                Name::BNode(node) => format!("_:{}", node),
                Name::Iri(iri) => format!("<{}>", iri),
            }
        )
    }
}

impl From<Iri> for GraphName {
    fn from(iri: Iri) -> Self {
        GraphName::named(iri.into())
    }
}

impl From<IriRef> for GraphName {
    fn from(iri: IriRef) -> Self {
        GraphName::named(iri)
    }
}

impl From<&IriRef> for GraphName {
    fn from(iri: &IriRef) -> Self {
        GraphName::named(iri.clone())
    }
}

impl GraphName {
    ///
    /// Construct a new graph name, as a blank node with a randomly assigned name.
    ///
    pub fn blank() -> Self {
        Self {
            inner: Name::BNode(new_blank_node_id()),
        }
    }

    ///
    /// Construct a new graph name reference, as a blank node with a randomly assigned name.
    ///
    pub fn blank_ref() -> GraphNameRef {
        Rc::from(Self::blank())
    }

    ///
    /// Construct a new graph name, as a blank node with the specified name.
    ///
    pub fn blank_named(name: &str) -> Self {
        Self {
            inner: Name::BNode(name.to_string()),
        }
    }

    ///
    /// Construct a new graph name, with an Iri naming a resource.
    ///
    pub fn named(name: IriRef) -> Self {
        Self {
            inner: Name::Iri(name),
        }
    }

    ///
    /// Construct a new graph name reference, with an Iri naming a resource.
    ///
    pub fn named_ref(name: IriRef) -> GraphNameRef {
        Rc::from(Self::named(name))
    }

    ///
    /// Return `true` if this graph name is a blank node, else `false`.
    ///
    pub fn is_blank(&self) -> bool {
        matches!(self.inner, Name::BNode(_))
    }

    ///
    /// Return a blank node string, if `self.is_blank()`, else `None`.
    ///
    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Name::BNode(s) => Some(s),
            _ => None,
        }
    }

    ///
    /// Return `true` if this graph name is an Iri, else `false`.
    ///
    pub fn is_iri(&self) -> bool {
        matches!(self.inner, Name::Iri(_))
    }

    ///
    /// Return a named node Iri, if `self.is_iri()`, else `None`.
    ///
    pub fn as_iri(&self) -> Option<&IriRef> {
        match &self.inner {
            Name::Iri(u) => Some(u),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn new_blank_node_id() -> String {
    format!("B{}", IDGenerator.next_id())
}
