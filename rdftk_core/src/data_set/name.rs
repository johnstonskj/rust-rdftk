/*!
An extension to the core `Graph` to support named graphs. The semantics of a named graph has been
derived from [RDF 1.1 TriG](https://www.w3.org/TR/trig/), _RDF Dataset Language_, and
[RDF 1.1: On Semantics of RDF Datasets](https://www.w3.org/TR/rdf11-datasets/).

*/

use rdftk_iri::{IRIRef, IRI};
use std::fmt::{Display, Formatter};
use unique_id::sequence::SequenceGenerator as IDGenerator;
use unique_id::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Graph name MUST be either an IRI or a blank node.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphName {
    inner: Name,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Name {
    BNode(String),
    IRI(IRIRef),
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
                Name::IRI(iri) => format!("<{}>", iri),
            }
        )
    }
}

impl From<IRI> for GraphName {
    fn from(iri: IRI) -> Self {
        GraphName::named(iri.into())
    }
}

impl From<IRIRef> for GraphName {
    fn from(iri: IRIRef) -> Self {
        GraphName::named(iri)
    }
}

impl From<&IRIRef> for GraphName {
    fn from(iri: &IRIRef) -> Self {
        GraphName::named(iri.clone())
    }
}

impl GraphName {
    pub fn blank() -> Self {
        Self {
            inner: Name::BNode(new_blank_node_id()),
        }
    }

    pub fn blank_named(name: &str) -> Self {
        Self {
            inner: Name::BNode(name.to_string()),
        }
    }

    pub fn named(name: IRIRef) -> Self {
        Self {
            inner: Name::IRI(name),
        }
    }

    pub fn is_blank(&self) -> bool {
        matches!(self.inner, Name::BNode(_))
    }

    pub fn as_blank(&self) -> Option<&String> {
        match &self.inner {
            Name::BNode(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_iri(&self) -> bool {
        matches!(self.inner, Name::IRI(_))
    }

    pub fn as_iri(&self) -> Option<&IRIRef> {
        match &self.inner {
            Name::IRI(u) => Some(u),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn new_blank_node_id() -> String {
    format!("B{}", IDGenerator::default().next_id())
}
