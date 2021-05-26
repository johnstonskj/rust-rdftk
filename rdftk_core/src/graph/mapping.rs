/*!
A trait for the prefix mappings required by the `Graph` trait. Prefix mappings can be added to a
graph to provide more readable serialization forms.
*/

use crate::qname::QName;
use rdftk_iri::{Fragment, IRIRef};
use rdftk_names::{rdf, rdfs, xsd};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

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
/// A simple bi-directional hash implementation between prefixes and IRI references.
///
#[derive(Clone, Debug)]
pub struct PrefixMappings {
    forward: HashMap<Prefix, IRIRef>,
    reverse: HashMap<IRIRef, Prefix>,
}

///
/// The type that Graph uses to expose it's mappings.
///
pub type PrefixMappingRef = Rc<RefCell<PrefixMappings>>;

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

// ------------------------------------------------------------------------------------------------

impl Default for PrefixMappings {
    fn default() -> Self {
        Self {
            forward: Default::default(),
            reverse: Default::default(),
        }
    }
}

///
/// Prefix mappings are used in the serialization of graphs.
///
impl PrefixMappings {
    ///
    /// Construct a new mapping instance with the provided default namespace.
    ///
    pub fn with_default(iri: IRIRef) -> Self {
        let mut new = PrefixMappings::default();
        let _ = new.set_default_namespace(iri);
        new
    }

    ///
    /// Returns `true` if there are no mappings in this instance, else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        assert_eq!(self.forward.len(), self.reverse.len());
        self.forward.is_empty()
    }

    ///
    /// Return the number of mappings in this instance.
    ///
    pub fn len(&self) -> usize {
        assert_eq!(self.forward.len(), self.reverse.len());
        self.forward.len()
    }

    ///
    /// Get the default namespace mapping, if present.
    ///
    pub fn get_default_namespace(&self) -> Option<&IRIRef> {
        self.get_namespace(&Prefix::Default)
    }

    ///
    /// Set the default namespace mapping.
    ///
    pub fn set_default_namespace(&mut self, iri: IRIRef) -> &mut Self {
        let _ = self.forward.insert(Prefix::Default, iri.clone());
        let _ = self.reverse.insert(iri, Prefix::Default);
        self
    }

    ///
    /// Get the namespace IRI associated with this provided prefix, if present.
    ///
    pub fn get_namespace(&self, prefix: &Prefix) -> Option<&IRIRef> {
        self.forward.get(prefix)
    }

    ///
    /// Get the prefix associated with this provided namespace URI, if present.
    ///
    pub fn get_prefix(&self, namespace: &IRIRef) -> Option<&Prefix> {
        self.reverse.get(namespace)
    }

    ///
    /// Return the set of prefixes in this mapping instance.
    ///
    pub fn prefixes(&self) -> Vec<&Prefix> {
        self.forward.keys().collect()
    }

    ///
    /// Insert a mapping from the prefix string to the namespace IRI.
    ///
    pub fn insert(&mut self, prefix: &str, iri: IRIRef) -> &mut Self {
        assert!(!prefix.is_empty());
        let prefix = Prefix::Some(prefix.to_string());
        let _ = self.forward.insert(prefix.clone(), iri.clone());
        let _ = self.reverse.insert(iri, prefix);
        self
    }

    ///
    /// Include the common "xsd" (XML Schema Data types) mapping.
    ///
    pub fn include_xsd(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(xsd::default_prefix(), xsd::namespace_iri().clone())
    }

    ///
    /// Include the common "rdf" mapping.
    ///
    pub fn include_rdf(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdf::default_prefix(), rdf::namespace_iri().clone())
    }

    ///
    /// Include the common "rdfs" mapping.
    ///
    pub fn include_rdfs(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self.insert(rdfs::default_prefix(), rdfs::namespace_iri().clone())
    }

    ///
    /// Remove a mapping for the provided prefix. This operation has no effect if no mapping is present.
    ///
    pub fn remove(&mut self, prefix: &Prefix) {
        let existing = self.forward.get(prefix).cloned();
        match &existing {
            None => (),
            Some(namespace) => {
                let _ = self.forward.remove(prefix);
                let _ = self.reverse.remove(namespace);
            }
        }
    }

    ///
    /// Remove all mappings from this instance.
    ///
    pub fn clear(&mut self) {
        self.forward.clear();
        self.reverse.clear();
    }

    ///
    /// Expand a qname into an IRI, if possible.
    ///
    pub fn expand(&self, qname: &QName) -> Option<IRIRef> {
        match self.get_namespace(&(qname.prefix().into())) {
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

    ///
    /// Compress an IRI into a qname, if possible.
    ///
    pub fn compress(&self, iri: &IRIRef) -> Option<QName> {
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
            Some(prefix) => match prefix {
                Prefix::Default => Some(QName::new(&name).unwrap()),
                Prefix::Some(prefix) => Some(QName::with_prefix(prefix, &name).unwrap()),
            },
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rdftk_iri::IRI;
    use std::str::FromStr;

    fn make_mappings() -> PrefixMappings {
        let mut mappings = PrefixMappings::default();
        let _ = mappings.include_xsd();
        let _ = mappings.include_rdf();
        let _ = mappings.include_rdfs();
        let _ = mappings.set_default_namespace(IRIRef::from(
            IRI::from_str("http://xmlns.com/foaf/0.1/").unwrap(),
        ));
        mappings
    }

    #[test]
    fn test_construct_mappings() {
        let mappings = make_mappings();

        assert_eq!(mappings.len(), 4);

        assert!(mappings
            .get_namespace(&Prefix::Some("xsd".to_string()))
            .is_some());
        assert!(mappings
            .get_namespace(&Prefix::Some("rdf".to_string()))
            .is_some());
        assert!(mappings
            .get_namespace(&Prefix::Some("rdfs".to_string()))
            .is_some());
        assert!(mappings.get_namespace(&Prefix::Default).is_some());
    }

    #[test]
    fn test_mapping_expand() {
        let mut mappings = make_mappings();
        let _ = mappings.insert(
            "foo",
            IRIRef::from(IRI::from_str("http://example.com/schema/foo/1.0").unwrap()),
        );

        assert_eq!(
            mappings.expand(&QName::new_unchecked(Some("rdf"), "Bag")),
            Some(IRIRef::from(
                IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag").unwrap()
            ))
        );
        assert_eq!(
            mappings.expand(&QName::new_unchecked(None, "knows")),
            Some(IRIRef::from(
                IRI::from_str("http://xmlns.com/foaf/0.1/knows").unwrap()
            ))
        );
        assert_eq!(
            mappings.expand(&QName::new_unchecked(Some("foo"), "Bar")),
            Some(IRIRef::from(
                IRI::from_str("http://example.com/schema/foo/1.0/Bar").unwrap()
            ))
        );

        assert_eq!(
            mappings.expand(&QName::new_unchecked(Some("rdfx"), "Bag")),
            None
        );
    }

    #[test]
    fn test_mapping_compress() {
        let mappings = make_mappings();

        assert_eq!(
            mappings.compress(&IRIRef::from(
                IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag").unwrap()
            )),
            Some(QName::new_unchecked(Some("rdf"), "Bag"))
        );
        assert_eq!(
            mappings.compress(&IRIRef::from(
                IRI::from_str("http://xmlns.com/foaf/0.1/knows").unwrap()
            )),
            Some(QName::new_unchecked(None, "knows"))
        );
        assert_eq!(
            mappings.compress(&IRIRef::from(
                IRI::from_str("http://www.w3.org/2003/01/geo/wgs84_pos#SpatialThing").unwrap()
            )),
            None
        );
    }
}
