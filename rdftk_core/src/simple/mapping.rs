/*!
Simple, in-memory implementation of the `PrefixMappings` trait.
*/

use crate::model::graph::mapping::{PrefixMappingFactory, PrefixMappingFactoryRef, DEFAULT_PREFIX};
use crate::model::graph::{PrefixMappingRef, PrefixMappings};
use crate::model::qname::QName;
use bimap::BiHashMap;
use rdftk_iri::{Fragment, IRIRef};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Retrieve the `GraphFactory` factory for `simple::SimpleGraph` instances.
///
pub fn prefix_mapping_factory() -> PrefixMappingFactoryRef {
    FACTORY.clone()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct SimplePrefixMappingFactory {}

lazy_static! {
    static ref FACTORY: Arc<SimplePrefixMappingFactory> =
        Arc::new(SimplePrefixMappingFactory::default());
}

#[derive(Clone, Debug)]
struct SimplePrefixMappings {
    map: BiHashMap<String, IRIRef>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SimplePrefixMappingFactory {
    fn default() -> Self {
        Self {}
    }
}

impl PrefixMappingFactory for SimplePrefixMappingFactory {
    fn empty(&self) -> PrefixMappingRef {
        Rc::new(RefCell::new(SimplePrefixMappings {
            map: Default::default(),
        }))
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SimplePrefixMappings {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl From<SimplePrefixMappings> for PrefixMappingRef {
    fn from(v: SimplePrefixMappings) -> Self {
        Rc::new(RefCell::new(v))
    }
}

impl PrefixMappings for SimplePrefixMappings {
    fn with_default(iri: IRIRef) -> Self {
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

    fn get_default_namespace(&self) -> Option<&IRIRef> {
        self.get_namespace(DEFAULT_PREFIX)
    }

    fn set_default_namespace(&mut self, iri: IRIRef) {
        let _ = self.map.insert(DEFAULT_PREFIX.to_string(), iri);
    }

    fn get_namespace(&self, prefix: &str) -> Option<&IRIRef> {
        self.map.get_by_left(prefix)
    }

    fn get_prefix(&self, namespace: &IRIRef) -> Option<&String> {
        self.map.get_by_right(namespace)
    }

    fn mappings<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a String, &'a IRIRef)> + 'a> {
        Box::new(self.map.iter())
    }

    fn insert(&mut self, prefix: &str, iri: IRIRef) {
        let _ = self.map.insert(prefix.to_string(), iri);
    }

    fn remove(&mut self, prefix: &str) {
        let _ = self.map.remove_by_left(prefix);
    }

    fn clear(&mut self) {
        self.map.clear();
    }

    fn expand(&self, qname: &QName) -> Option<IRIRef> {
        let default_ns = DEFAULT_PREFIX.to_string();
        match self.get_namespace(&qname.prefix().as_ref().unwrap_or(&default_ns)) {
            None => None,
            Some(namespace) => {
                if namespace.has_fragment() {
                    Some(IRIRef::from(
                        namespace.with_new_fragment(qname.name().parse().unwrap()),
                    ))
                } else {
                    let mut path = namespace.path().clone();
                    if path.push(qname.name()).is_ok() {
                        Some(IRIRef::from(namespace.with_new_path(path)))
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn compress(&self, iri: &IRIRef) -> Option<QName> {
        let (iri, name) = if iri.has_fragment() {
            let fragment = iri.fragment();
            let fragment = fragment.as_ref().unwrap();
            (
                iri.with_new_fragment(Fragment::default()),
                fragment.value().clone(),
            )
        } else if iri.path().has_slug() {
            let mut path = iri.path().clone();
            let name = path.pop_slug();
            (iri.with_new_path(path), name.unwrap())
        } else {
            return None;
        };
        match self.get_prefix(&IRIRef::from(iri)) {
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
