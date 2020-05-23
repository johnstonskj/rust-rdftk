/*!
Implementation of the `PrefixMapping` trait for `MemGraph`.

# Example

TBD

*/

use rdftk_core::QName;
use rdftk_graph::PrefixMapping;
use rdftk_iri::IRI;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Mappings {
    forward: HashMap<String, IRI>,
    reverse: HashMap<IRI, String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const DEFAULT_PREFIX: &str = "";

impl Default for Mappings {
    fn default() -> Self {
        Self {
            forward: Default::default(),
            reverse: Default::default(),
        }
    }
}

impl PrefixMapping for Mappings {
    fn is_empty(&self) -> bool {
        assert_eq!(self.forward.len(), self.reverse.len());
        self.forward.is_empty()
    }

    fn len(&self) -> usize {
        assert_eq!(self.forward.len(), self.reverse.len());
        self.forward.len()
    }

    fn get_namespace(&self, prefix: Option<&str>) -> Option<&IRI> {
        self.forward.get(match prefix {
            None => DEFAULT_PREFIX,
            Some(prefix) => prefix,
        })
    }

    fn get_prefix(&self, namespace: &IRI) -> Option<&String> {
        self.reverse.get(namespace)
    }

    fn expand(&self, _qname: QName) -> Option<IRI> {
        unimplemented!()
    }

    fn compress(&self, _uri: IRI) -> Option<QName> {
        unimplemented!()
    }

    fn insert_default(&mut self, uri: &IRI) -> &mut Self {
        self.forward.insert(DEFAULT_PREFIX.to_string(), uri.clone());
        self.reverse.insert(uri.clone(), DEFAULT_PREFIX.to_string());
        self
    }

    fn insert(&mut self, prefix: &str, uri: &IRI) -> &mut Self {
        assert!(!prefix.is_empty());
        self.forward.insert(prefix.to_string(), uri.clone());
        self.reverse.insert(uri.clone(), prefix.to_string());
        self
    }

    fn remove(&mut self, prefix: &str) {
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
    pub fn with_default(uri: &IRI) -> Self {
        let mut new = Mappings::default();
        new.insert_default(uri);
        new
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
