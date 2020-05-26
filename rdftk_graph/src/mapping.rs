/*!
A trait for prefix-mapping required by the `Graph` trait.

# Example

TBD

*/

#![allow(clippy::module_name_repetitions)]

use rdftk_core::QName;
use rdftk_iri::IRI;
use rdftk_names::{rdf, rdfs, xsd};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prefix {
    Default,
    Some(String),
}

pub trait PrefixMappings {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn get_namespace(&self, prefix: &Prefix) -> Option<&IRI>;

    fn get_prefix(&self, namespace: &IRI) -> Option<&Prefix>;

    fn expand(&self, qname: QName) -> Option<IRI>;

    fn compress(&self, iri: IRI) -> Option<QName>;

    fn insert_default(&mut self, iri: IRI) -> &mut Self
    where
        Self: Sized;

    fn insert(&mut self, prefix: &str, iri: IRI) -> &mut Self
    where
        Self: Sized;

    fn include_xsd(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(xsd::PREFIX, IRI::from_str(xsd::NAMESPACE).unwrap())
    }

    fn include_rdf(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdf::PREFIX, IRI::from_str(rdf::NAMESPACE).unwrap())
    }

    fn include_rdfs(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdfs::PREFIX, IRI::from_str(rdfs::NAMESPACE).unwrap())
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
