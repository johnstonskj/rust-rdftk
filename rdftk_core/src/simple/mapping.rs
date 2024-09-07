/*!
Simple, in-memory implementation of the `PrefixMappings` trait.
*/

use crate::model::graph::{PrefixMappingRef, PrefixMappings};
use crate::model::qname::QName;
use bimap::BiHashMap;
use rdftk_iri::{IriExtra, IriRef, Name};
use std::cell::RefCell;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Create a new prefix mapping instance with default mappings.
///
#[inline]
pub fn default_mappings() -> PrefixMappingRef {
    SimplePrefixMappings::default().into()
}

///
/// Create a new prefix mapping instance with the RDF, RDF Schema, and XML Namespace mappings.
///
#[inline]
pub fn common_mappings() -> PrefixMappingRef {
    SimplePrefixMappings::default()
        .with_owl()
        .with_rdf()
        .with_rdfs()
        .with_xsd()
        .into()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `PrefixMappings` trait.
///
#[derive(Clone, Debug, Default)]
struct SimplePrefixMappings {
    map: BiHashMap<Option<Name>, IriRef>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<SimplePrefixMappings> for PrefixMappingRef {
    fn from(v: SimplePrefixMappings) -> Self {
        Rc::new(RefCell::new(v))
    }
}

impl PrefixMappings for SimplePrefixMappings {
    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn get_default_namespace(&self) -> Option<&IriRef> {
        self.map.get_by_left(&None)
    }

    fn set_default_namespace(&mut self, iri: IriRef) {
        let _ = self.map.insert(None, iri);
    }

    fn remove_default_namespace(&mut self) {
        let _ = self.map.remove_by_left(&None);
    }

    fn get_namespace(&self, prefix: &Name) -> Option<&IriRef> {
        self.map.get_by_left(&Some(prefix.clone()))
    }

    fn get_prefix(&self, namespace: &IriRef) -> Option<&Option<Name>> {
        self.map.get_by_right(namespace)
    }

    fn mappings<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a Option<Name>, &'a IriRef)> + 'a> {
        Box::new(self.map.iter())
    }

    fn insert(&mut self, prefix: Name, iri: IriRef) {
        let _ = self.map.insert(Some(prefix), iri);
    }

    fn remove(&mut self, prefix: &Name) {
        let _ = self.map.remove_by_left(&Some(prefix.clone()));
    }

    fn clear(&mut self) {
        self.map.clear();
    }

    fn expand(&self, qname: &QName) -> Option<IriRef> {
        let prefix = if let Some(prefix) = qname.prefix() {
            self.get_namespace(prefix)
        } else {
            self.get_default_namespace()
        };
        match prefix {
            None => None,
            Some(namespace) => namespace.make_name(qname.name().clone()).map(IriRef::from),
        }
    }

    fn compress(&self, iri: &IriRef) -> Option<QName> {
        let (iri, name) = if let Some((iri, name)) = iri.split() {
            (iri, name)
        } else {
            return None;
        };
        match self.get_prefix(&IriRef::from(iri)) {
            None => None,
            Some(None) => Some(QName::new_unqualified(name).unwrap()),
            Some(Some(prefix)) => Some(QName::new(prefix.clone(), name).unwrap()),
        }
    }
}
