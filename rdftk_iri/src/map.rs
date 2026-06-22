//!
//! This module provides the `IriPrefixMap` type used to maintain mappings between `Namespace`
//! prefix names and absolute `Iri` values.
//!

use crate::{
    LocalName,
    iri::{Iri, IriExtra},
    pname::Namespace,
    vocab::{
        VOCABULARY_OWL, VOCABULARY_RDF, VOCABULARY_RDF_SCHEMA, VOCABULARY_XML_SCHEMA, Vocabulary,
    },
};
use bimap::BiBTreeMap;
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Implementation of a mapping from a prefix `Name` to an `Iri`. Prefix mappings are commonly used
/// in the serialization of graphs.
///
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IriPrefixMap {
    map: BiBTreeMap<Namespace, Iri>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Prefix Mappings
// ------------------------------------------------------------------------------------------------

impl Display for IriPrefixMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Prefixes(")?;
        for (prefix, iri) in &self.map {
            writeln!(f, "    {prefix}  {iri} ,")?;
        }
        writeln!(f, ")")
    }
}

impl IriPrefixMap {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new map with the four core RDF+OWL namespaces.
    ///
    /// The map will be created with the following mappings:
    ///
    /// | Prefix | Namespace                                      | Vocabulary                |
    /// | ------ | ---------------------------------------------- | ------------------------- |
    /// | owl    | <http://www.w3.org/2002/07/owl#>               | [`VOCABULARY_OWL`]        |
    /// | rdf    | <http://www.w3.org/1999/02/22-rdf-syntax-ns#>  | [`VOCABULARY_RDF`]        |
    /// | rdfs   | <http://www.w3.org/2000/01/rdf-schema#>        | [`VOCABULARY_RDF_SCHEMA`] |
    /// | xsd    | <http://www.w3.org/2001/XMLSchema#>            | [`VOCABULARY_XML_SCHEMA`] |
    ///
    pub fn common() -> Self {
        Self::default()
            .with_vocabulary(&VOCABULARY_RDF)
            .with_vocabulary(&VOCABULARY_RDF_SCHEMA)
            .with_vocabulary(&VOCABULARY_XML_SCHEMA)
            .with_vocabulary(&VOCABULARY_OWL)
    }

    ///
    /// Construct a new mapping instance with the provided default namespace.
    ///
    pub fn with_default(mut self, iri: Iri) -> Self {
        self.set_default_namespace(iri);
        self
    }

    ///
    /// Construct a new mapping instance with the provided mapping.
    ///
    pub fn with(mut self, prefix: Namespace, iri: Iri) -> Self {
        self.insert(prefix, iri);
        self
    }

    ///
    /// Construct a new mapping instance with a mapping for the provided vocabulary.
    ///
    pub fn with_vocabulary(self, vocabulary: &Vocabulary) -> Self {
        Self::with(
            self,
            vocabulary.prefix_as_namespace(),
            vocabulary.iri_as_iri(),
        )
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
        self.map.get_by_left(&Namespace::new_default())
    }

    ///
    /// Set the default namespace mapping.
    ///
    pub fn set_default_namespace(&mut self, iri: Iri) {
        let _ = self.map.insert(Namespace::new_default(), iri);
    }

    ///
    /// Remove any mapping for the default namespace, if present.
    ///
    pub fn remove_default_namespace(&mut self) {
        let _ = self.map.remove_by_left(&Namespace::new_default());
    }

    ///
    /// Get the namespace Iri associated with this provided prefix, if present.
    ///
    pub fn get_namespace(&self, prefix: &Namespace) -> Option<&Iri> {
        self.map.get_by_left(prefix)
    }

    ///
    /// Get the prefix associated with this provided namespace URI, if present.
    ///
    pub fn get_prefix(&self, namespace: &Iri) -> Option<&Namespace> {
        self.map.get_by_right(namespace)
    }

    ///
    /// Return an iterator over the contained mappings.
    ///
    pub fn mappings(&self) -> impl Iterator<Item = (&Namespace, &Iri)> {
        self.map.iter()
    }

    ///
    /// Return an iterator over the contained mappings.
    ///
    pub fn prefixes(&self) -> impl Iterator<Item = &Namespace> {
        self.map.left_values()
    }

    ///
    /// Return an iterator over the contained mappings.
    ///
    pub fn iris(&self) -> impl Iterator<Item = &Iri> {
        self.map.right_values()
    }

    ///
    /// Insert a mapping from the prefix string to the namespace Iri.
    ///
    pub fn insert(&mut self, prefix: Namespace, iri: Iri) {
        let _ = self.map.insert(prefix, iri);
    }

    pub fn insert_vocabulary(&mut self, vocabulary: &Vocabulary) {
        self.insert(vocabulary.prefix_as_namespace(), vocabulary.iri_as_iri());
    }

    ///
    /// Remove a mapping for the provided prefix. This operation has no effect if no mapping is present.
    ///
    pub fn remove(&mut self, prefix: &Namespace) {
        let _ = self.map.remove_by_left(prefix);
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
    pub fn expand(&self, local_name: &LocalName) -> Option<Iri> {
        match self
            .get_namespace(local_name.namespace())
            .map(|ns| ns.make_name(local_name.name().clone()))
        {
            Some(expanded) => expanded,
            None => None,
        }
    }

    ///
    /// Compress an Iri into a qname, if possible.
    ///
    pub fn compress(&self, iri: &Iri) -> Option<LocalName> {
        let (iri, name) = if let Some((iri, name)) = iri.split() {
            (iri, name)
        } else {
            return None;
        };
        match self.get_prefix(&iri) {
            None => None,
            Some(prefix) => Some(LocalName::new(prefix.clone(), name)),
        }
    }
}
