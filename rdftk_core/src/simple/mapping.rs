/*!
Simple, in-memory implementation of the `PrefixMappings` trait.
*/

use crate::model::graph::mapping::DEFAULT_PREFIX;
use crate::model::graph::{PrefixMappingRef, PrefixMappings};
use crate::model::qname::QName;
use bimap::BiHashMap;
use rdftk_iri::{IriExtra, IriRef};
use std::cell::RefCell;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Create a new prefix mapping instance with no mappings.
///
pub fn empty_mappings() -> PrefixMappingRef {
    SimplePrefixMappings::default().into()
}

///
/// Create a new prefix mapping instance with the RDF, RDF Schema, and XML Namespace mappings.
///
pub fn common_mappings() -> PrefixMappingRef {
    let mapping = empty_mappings();
    {
        let mut mut_mapping = mapping.borrow_mut();
        mut_mapping.include_rdf();
        mut_mapping.include_rdfs();
        mut_mapping.include_xsd();
    }
    mapping
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `PrefixMappings` trait.
///
#[derive(Clone, Debug, Default)]
struct SimplePrefixMappings {
    map: BiHashMap<String, IriRef>,
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
    fn with_default(iri: IriRef) -> Self {
        let mut mut_self = Self::default();
        mut_self.set_default_namespace(iri);
        mut_self
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn get_default_namespace(&self) -> Option<&IriRef> {
        self.get_namespace(DEFAULT_PREFIX)
    }

    fn set_default_namespace(&mut self, iri: IriRef) {
        let _ = self.map.insert(DEFAULT_PREFIX.to_string(), iri);
    }

    fn get_namespace(&self, prefix: &str) -> Option<&IriRef> {
        self.map.get_by_left(prefix)
    }

    fn get_prefix(&self, namespace: &IriRef) -> Option<&String> {
        self.map.get_by_right(namespace)
    }

    fn mappings<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a String, &'a IriRef)> + 'a> {
        Box::new(self.map.iter())
    }

    fn insert(&mut self, prefix: &str, iri: IriRef) {
        let _ = self.map.insert(prefix.to_string(), iri);
    }

    fn remove(&mut self, prefix: &str) {
        let _ = self.map.remove_by_left(prefix);
    }

    fn clear(&mut self) {
        self.map.clear();
    }

    fn expand(&self, qname: &QName) -> Option<IriRef> {
        let default_ns = DEFAULT_PREFIX.to_string();
        match self.get_namespace(qname.prefix().as_ref().unwrap_or(&default_ns)) {
            None => None,
            Some(namespace) => namespace.make_name(qname.name()).map(IriRef::from),
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
            Some(prefix) => {
                if prefix == DEFAULT_PREFIX {
                    Some(QName::new(&name).unwrap())
                } else {
                    Some(QName::with_prefix(prefix, &name).unwrap())
                }
            }
        }
    }
}
