/*!
A trait for the prefix mappings required by the `Graph` trait. Prefix mappings can be added to a
graph to provide more readable serialization forms.
*/

use bimap::BiHashMap;
use rdftk_iri::{Iri, IriExtra, Name, QName};
use rdftk_names::{dc, foaf, owl, rdf, rdfs, xsd};
use std::fmt::Debug;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Implementation of a mapping from a prefix `Name` to an `Iri`. Prefix mappings are commonly used
/// in the serialization of graphs.
///
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PrefixMapping {
    map: BiHashMap<Option<Name>, Iri>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Create a new prefix mapping instance with the RDF, RDF Schema, and XML Namespace mappings.
///
#[inline]
pub fn common_mappings() -> PrefixMapping {
    PrefixMapping::default()
        .with_owl()
        .with_rdf()
        .with_rdfs()
        .with_xsd()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PrefixMapping {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new mapping instance with the provided default namespace.
    ///
    pub fn with_default(self, iri: Iri) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.set_default_namespace(iri);
        mut_self
    }

    ///
    /// Include the common "dcterms" mapping.
    ///
    pub fn with_dcterms(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_dcterms();
        mut_self
    }

    ///
    /// Include the common "foaf" mapping.
    ///
    pub fn with_foaf(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_foaf();
        mut_self
    }

    ///
    /// Include the common "owl" mapping.
    ///
    pub fn with_owl(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_owl();
        mut_self
    }

    ///
    /// Include the common "rdf" mapping.
    ///
    pub fn with_rdf(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_rdf();
        mut_self
    }

    ///
    /// Include the common "rdfs" mapping.
    ///
    pub fn with_rdfs(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_rdfs();
        mut_self
    }

    ///
    /// Include the common "xsd" (XML Schema Data types) mapping.
    ///
    pub fn with_xsd(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_xsd();
        mut_self
    }

    // --------------------------------------------------------------------------------------------
    // Collection methods
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if there are no mappings in this instance, else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    ///
    /// Return the number of mappings in this instance.
    ///
    pub fn len(&self) -> usize {
        self.map.len()
    }

    ///
    /// Get the default namespace mapping, if present.
    ///
    pub fn get_default_namespace(&self) -> Option<&Iri> {
        self.map.get_by_left(&None)
    }

    ///
    /// Set the default namespace mapping.
    ///
    pub fn set_default_namespace(&mut self, iri: Iri) {
        let _ = self.map.insert(None, iri);
    }

    pub fn remove_default_namespace(&mut self) {
        let _ = self.map.remove_by_left(&None);
    }

    ///
    /// Get the namespace Iri associated with this provided prefix, if present.
    ///
    pub fn get_namespace(&self, prefix: &Name) -> Option<&Iri> {
        self.map.get_by_left(&Some(prefix.clone()))
    }

    ///
    /// Get the prefix associated with this provided namespace URI, if present.
    ///
    pub fn get_prefix(&self, namespace: &Iri) -> Option<&Option<Name>> {
        self.map.get_by_right(namespace)
    }

    ///
    /// Return an iterator over the contained mappings.
    ///
    pub fn mappings<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a Option<Name>, &'a Iri)> + 'a> {
        Box::new(self.map.iter())
    }

    ///
    /// Insert a mapping from the prefix string to the namespace Iri.
    ///
    pub fn insert(&mut self, prefix: Name, iri: Iri) {
        let _ = self.map.insert(Some(prefix), iri);
    }

    pub fn insert_owl(&mut self) {
        self.insert(owl::default_prefix().clone(), owl::namespace().clone());
    }

    pub fn insert_rdf(&mut self) {
        self.insert(rdf::default_prefix().clone(), rdf::namespace().clone());
    }

    pub fn insert_rdfs(&mut self) {
        self.insert(rdfs::default_prefix().clone(), rdfs::namespace().clone());
    }

    pub fn insert_xsd(&mut self) {
        self.insert(xsd::default_prefix().clone(), xsd::namespace().clone());
    }

    pub fn insert_foaf(&mut self) {
        self.insert(foaf::default_prefix().clone(), foaf::namespace().clone());
    }

    pub fn insert_dcterms(&mut self) {
        self.insert(
            dc::terms::default_prefix().clone(),
            dc::terms::namespace().clone(),
        );
    }

    ///
    /// Remove a mapping for the provided prefix. This operation has no effect if no mapping is present.
    ///
    pub fn remove(&mut self, prefix: &Name) {
        let _ = self.map.remove_by_left(&Some(prefix.clone()));
    }

    ///
    /// Remove all mappings from this instance.
    ///
    pub fn clear(&mut self) {
        self.map.clear();
    }

    // --------------------------------------------------------------------------------------------
    // QName Mapping
    // --------------------------------------------------------------------------------------------

    ///
    /// Expand a qname into an Iri, if possible.
    ///
    pub fn expand(&self, qname: &QName) -> Option<Iri> {
        let prefix = if let Some(prefix) = qname.prefix() {
            self.get_namespace(prefix)
        } else {
            self.get_default_namespace()
        };
        match prefix {
            None => None,
            Some(namespace) => namespace.make_name(qname.name().clone()),
        }
    }

    ///
    /// Compress an Iri into a qname, if possible.
    ///
    pub fn compress(&self, iri: &Iri) -> Option<QName> {
        let (iri, name) = if let Some((iri, name)) = iri.split() {
            (iri, name)
        } else {
            return None;
        };
        match self.get_prefix(&iri) {
            None => None,
            Some(None) => Some(QName::new_unqualified(name).unwrap()),
            Some(Some(prefix)) => Some(QName::new(prefix.clone(), name).unwrap()),
        }
    }
}
