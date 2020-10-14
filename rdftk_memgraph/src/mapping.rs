/*!
Implementation of the `PrefixMapping` trait for `MemGraph`.

# Example

TBD

*/

use rdftk_core::graph::{Prefix, PrefixMappings};
use rdftk_core::QName;
use rdftk_iri::{Fragment, IRIRef};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Mappings {
    forward: HashMap<Prefix, IRIRef>,
    reverse: HashMap<IRIRef, Prefix>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Mappings {
    fn default() -> Self {
        Self {
            forward: Default::default(),
            reverse: Default::default(),
        }
    }
}

impl PrefixMappings for Mappings {
    fn is_empty(&self) -> bool {
        assert_eq!(self.forward.len(), self.reverse.len());
        self.forward.is_empty()
    }

    fn len(&self) -> usize {
        assert_eq!(self.forward.len(), self.reverse.len());
        self.forward.len()
    }

    fn get_namespace(&self, prefix: &Prefix) -> Option<&IRIRef> {
        self.forward.get(prefix)
    }

    fn get_prefix(&self, namespace: &IRIRef) -> Option<&Prefix> {
        self.reverse.get(namespace)
    }

    fn prefixes(&self) -> Vec<&Prefix> {
        self.forward.keys().collect()
    }

    fn expand(&self, qname: QName) -> Option<IRIRef> {
        match self.get_namespace(&(qname.prefix().into())) {
            None => None,
            Some(namespace) => {
                if namespace.has_fragment() {
                    Some(IRIRef::from(
                        namespace.with_new_fragment(Some(qname.name().parse().unwrap())),
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

    fn compress(&self, iri: IRIRef) -> Option<QName> {
        let (iri, name) = if iri.has_fragment() {
            let fragment = iri.fragment();
            let fragment = fragment.as_ref().unwrap();
            (
                iri.with_new_fragment(Some(Fragment::default())),
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
                Prefix::Default => Some(QName::new(&name)),
                Prefix::Some(prefix) => Some(QName::with_prefix(prefix, &name)),
            },
        }
    }

    fn insert_default(&mut self, iri: IRIRef) -> &mut Self {
        self.forward.insert(Prefix::Default, iri.clone());
        self.reverse.insert(iri, Prefix::Default);
        self
    }

    fn insert(&mut self, prefix: &str, iri: IRIRef) -> &mut Self {
        assert!(!prefix.is_empty());
        let prefix = Prefix::Some(prefix.to_string());
        self.forward.insert(prefix.clone(), iri.clone());
        self.reverse.insert(iri, prefix);
        self
    }

    fn remove(&mut self, prefix: &Prefix) {
        let existing = self.forward.get(prefix).cloned();
        match &existing {
            None => (),
            Some(namespace) => {
                self.forward.remove(prefix);
                self.reverse.remove(namespace);
            }
        }
    }

    fn clear(&mut self) {
        self.forward.clear();
        self.reverse.clear();
    }
}

impl Mappings {
    pub fn with_default(iri: IRIRef) -> Self {
        let mut new = Mappings::default();
        new.insert_default(iri);
        new
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

    fn make_mappings() -> Mappings {
        let mut mappings = Mappings::default();
        mappings.include_xsd();
        mappings.include_rdf();
        mappings.include_rdfs();
        mappings.insert_default(IRIRef::from(
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
        mappings.insert(
            "foo",
            IRIRef::from(IRI::from_str("http://example.com/schema/foo/1.0").unwrap()),
        );

        assert_eq!(
            mappings.expand(QName::with_prefix("rdf", "Bag")),
            Some(IRIRef::from(
                IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag").unwrap()
            ))
        );
        assert_eq!(
            mappings.expand(QName::new("knows")),
            Some(IRIRef::from(
                IRI::from_str("http://xmlns.com/foaf/0.1/knows").unwrap()
            ))
        );
        assert_eq!(
            mappings.expand(QName::with_prefix("foo", "Bar")),
            Some(IRIRef::from(
                IRI::from_str("http://example.com/schema/foo/1.0/Bar").unwrap()
            ))
        );

        assert_eq!(mappings.expand(QName::with_prefix("rdfx", "Bag")), None);
    }

    #[test]
    fn test_mapping_compress() {
        let mappings = make_mappings();

        assert_eq!(
            mappings.compress(IRIRef::from(
                IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag").unwrap()
            )),
            Some(QName::with_prefix("rdf", "Bag"))
        );
        assert_eq!(
            mappings.compress(IRIRef::from(
                IRI::from_str("http://xmlns.com/foaf/0.1/knows").unwrap()
            )),
            Some(QName::new("knows"))
        );
        assert_eq!(
            mappings.compress(IRIRef::from(
                IRI::from_str("http://www.w3.org/2003/01/geo/wgs84_pos#SpatialThing").unwrap()
            )),
            None
        );
    }
}
