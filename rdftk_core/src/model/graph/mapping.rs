/*!
A trait for the prefix mappings required by the `Graph` trait. Prefix mappings can be added to a
graph to provide more readable serialization forms.
*/

use crate::model::qname::QName;
use rdftk_iri::IriRef;
use rdftk_names::{rdf, rdfs, xsd};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The prefix used to denote the default namespace in the prefix mapping.
///
pub const DEFAULT_PREFIX: &str = "";

///
/// Prefix mappings are used in the serialization of graphs.
///
pub trait PrefixMappings: Debug {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    ///
    /// Construct a new mapping instance with the provided default namespace.
    ///
    fn with_default(iri: IriRef) -> Self
    where
        Self: Sized;

    ///
    /// Include the common "xsd" (XML Schema Data types) mapping.
    ///
    fn include_xsd(&mut self) {
        self.insert(xsd::default_prefix(), xsd::namespace_iri().clone());
    }

    ///
    /// Include the common "rdf" mapping.
    ///
    fn include_rdf(&mut self) {
        self.insert(rdf::default_prefix(), rdf::namespace_iri().clone());
    }

    ///
    /// Include the common "rdfs" mapping.
    ///
    fn include_rdfs(&mut self) {
        self.insert(rdfs::default_prefix(), rdfs::namespace_iri().clone());
    }

    // --------------------------------------------------------------------------------------------
    // Collection methods
    // --------------------------------------------------------------------------------------------

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
    fn get_default_namespace(&self) -> Option<&IriRef>;

    ///
    /// Set the default namespace mapping.
    ///
    fn set_default_namespace(&mut self, iri: IriRef);

    ///
    /// Get the namespace Iri associated with this provided prefix, if present.
    ///
    fn get_namespace(&self, prefix: &str) -> Option<&IriRef>;

    ///
    /// Get the prefix associated with this provided namespace URI, if present.
    ///
    fn get_prefix(&self, namespace: &IriRef) -> Option<&String>;

    ///
    /// Return an iterator over the contained mappings.
    ///
    fn mappings<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a String, &'a IriRef)> + 'a>;

    ///
    /// Insert a mapping from the prefix string to the namespace Iri.
    ///
    fn insert(&mut self, prefix: &str, iri: IriRef);

    ///
    /// Remove a mapping for the provided prefix. This operation has no effect if no mapping is present.
    ///
    fn remove(&mut self, prefix: &str);

    ///
    /// Remove all mappings from this instance.
    ///
    fn clear(&mut self);

    // --------------------------------------------------------------------------------------------
    // QName Mapping
    // --------------------------------------------------------------------------------------------

    ///
    /// Expand a qname into an Iri, if possible.
    ///
    fn expand(&self, qname: &QName) -> Option<IriRef>;

    ///
    /// Compress an Iri into a qname, if possible.
    ///
    fn compress(&self, iri: &IriRef) -> Option<QName>;
}

///
/// The actual object storage type, reference counted for memory management.
///
pub type PrefixMappingRef = Rc<RefCell<dyn PrefixMappings>>;
