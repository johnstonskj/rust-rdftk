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

pub trait PrefixMapping {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn get_namespace(&self, prefix: Option<&str>) -> Option<&IRI>;

    fn get_prefix(&self, namespace: &IRI) -> Option<&String>;

    fn expand(&self, qname: QName) -> Option<IRI>;

    fn compress(&self, uri: IRI) -> Option<QName>;

    fn insert_default(&mut self, uri: &IRI) -> &mut Self
    where
        Self: Sized;

    fn insert(&mut self, prefix: &str, uri: &IRI) -> &mut Self
    where
        Self: Sized;

    fn include_xsd(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(xsd::PREFIX, &IRI::from_str(xsd::NAMESPACE).unwrap())
    }

    fn include_rdf(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdf::PREFIX, &IRI::from_str(rdf::NAMESPACE).unwrap())
    }

    fn include_rdfs(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdfs::PREFIX, &IRI::from_str(rdfs::NAMESPACE).unwrap())
    }

    fn remove(&mut self, prefix: &str);

    fn clear(&mut self);
}
