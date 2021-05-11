/*!
A trait for the prefix mappings required by the `Graph` trait. Prefix mappings can be added to a
graph to provide more readable serialization forms.
*/

use crate::qname::QName;
use rdftk_iri::IRIRef;
use rdftk_names::{rdf, rdfs, xsd};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A prefix
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prefix {
    /// The default namespace pseudo-prefix.
    Default,
    /// The string prefix for a mapping.
    Some(String),
}

///
/// Prefix mappings are used in the serialization of graphs.
///
pub trait PrefixMappings: Debug {
    ///
    /// Returns `true` if there are no mappings in this instance, else `false`.
    ///
    fn is_empty(&self) -> bool;

    ///
    /// Return the number of mappings in this instance.
    ///
    fn len(&self) -> usize;

    ///
    /// Get the default namespace mapping, if present.
    ///
    fn get_default_namespace(&self) -> Option<&IRIRef> {
        self.get_namespace(&Prefix::Default)
    }

    ///
    /// Set the default namespace mapping.
    ///
    fn set_default_namespace(&mut self, iri: IRIRef) -> &mut Self
    where
        Self: Sized;

    ///
    /// Get the namespace IRI associated with this provided prefix, if present.
    ///
    fn get_namespace(&self, prefix: &Prefix) -> Option<&IRIRef>;

    ///
    /// Get the prefix associated with this provided namespace URI, if present.
    ///
    fn get_prefix(&self, namespace: &IRIRef) -> Option<&Prefix>;

    ///
    /// Return the set of prefixes in this mapping instance.
    ///
    fn prefixes(&self) -> Vec<&Prefix>;

    ///
    /// Insert a mapping from the prefix string to the namespace IRI.
    ///
    fn insert(&mut self, prefix: &str, iri: IRIRef) -> &mut Self
    where
        Self: Sized;

    ///
    /// Include the common "xsd" (XML Schema Data types) mapping.
    ///
    fn include_xsd(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(xsd::default_prefix(), xsd::namespace_iri().clone())
    }

    ///
    /// Include the common "rdf" mapping.
    ///
    fn include_rdf(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdf::default_prefix(), rdf::namespace_iri().clone())
    }

    ///
    /// Include the common "rdfs" mapping.
    ///
    fn include_rdfs(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdfs::default_prefix(), rdfs::namespace_iri().clone())
    }

    ///
    /// Remove a mapping for the provided prefix. This operation has no effect if no mapping is present.
    ///
    fn remove(&mut self, prefix: &Prefix);

    ///
    /// Remove all mappings from this instance.
    fn clear(&mut self);

    ///
    /// Expand a qname into an IRI, if possible.
    ///
    fn expand(&self, qname: &QName) -> Option<IRIRef>;

    ///
    /// Compress an IRI into a qname, if possible.
    fn compress(&self, iri: &IRIRef) -> Option<QName>;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Option<String>> for Prefix {
    fn from(opt: Option<String>) -> Self {
        match opt {
            None => Prefix::Default,
            Some(v) => Prefix::Some(v),
        }
    }
}

impl From<&Option<String>> for Prefix {
    fn from(opt: &Option<String>) -> Self {
        match opt {
            None => Prefix::Default,
            Some(v) => Prefix::Some(v.clone()),
        }
    }
}

impl From<Prefix> for Option<String> {
    fn from(v: Prefix) -> Self {
        match v {
            Prefix::Default => None,
            Prefix::Some(v) => Some(v),
        }
    }
}
