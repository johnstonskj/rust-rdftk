/*!
A trait for prefix-mapping required by the `Graph` trait.

# Example

TBD

*/

#![allow(clippy::module_name_repetitions)]

use crate::QName;
use rdftk_iri::IRIRef;
use rdftk_names::{rdf, rdfs, xsd};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prefix {
    Default,
    Some(String),
}

pub trait PrefixMappings: Debug {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn get_namespace(&self, prefix: &Prefix) -> Option<&IRIRef>;

    fn get_prefix(&self, namespace: &IRIRef) -> Option<&Prefix>;

    fn prefixes(&self) -> Vec<&Prefix>;

    fn expand(&self, qname: QName) -> Option<IRIRef>;

    fn compress(&self, iri: IRIRef) -> Option<QName>;

    fn insert_default(&mut self, iri: IRIRef) -> &mut Self
    where
        Self: Sized;

    fn insert(&mut self, prefix: &str, iri: IRIRef) -> &mut Self
    where
        Self: Sized;

    fn include_xsd(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(xsd::default_prefix(), xsd::namespace_iri().clone())
    }

    fn include_rdf(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdf::default_prefix(), rdf::namespace_iri().clone())
    }

    fn include_rdfs(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdfs::default_prefix(), rdfs::namespace_iri().clone())
    }

    fn remove(&mut self, prefix: &Prefix);

    fn clear(&mut self);
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

impl Into<Option<String>> for Prefix {
    fn into(self) -> Option<String> {
        match self {
            Prefix::Default => None,
            Prefix::Some(v) => Some(v),
        }
    }
}
