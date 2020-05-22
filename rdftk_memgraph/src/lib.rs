/*!
One-line description.

More detailed description, with

# Example

*/

use rdftk_core::{ObjectNode, Resource, Statement, SubjectNode};
use rdftk_graph::{Graph, NamedGraph, PrefixMapping};
use rdftk_iri::IRI;
use std::collections::HashSet;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct MemGraph {
    name: Option<IRI>,
    statements: Vec<Rc<Statement>>,
    mappings: Rc<Mappings>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for MemGraph {
    fn default() -> Self {
        Self {
            name: None,
            statements: Default::default(),
            mappings: Default::default(),
        }
    }
}

impl From<Vec<Statement>> for MemGraph {
    fn from(sts: Vec<Statement>) -> Self {
        Self::with(sts.into_iter().map(Rc::new).collect())
    }
}

impl From<Vec<Rc<Statement>>> for MemGraph {
    fn from(sts: Vec<Rc<Statement>>) -> Self {
        Self::with(sts)
    }
}

impl Graph for MemGraph {
    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn contains(&self, statement: &Statement) -> bool {
        self.statements.iter().any(|st| st.as_ref() == statement)
    }

    fn contains_all(&self, subject: &SubjectNode, predicate: &IRI, object: &ObjectNode) -> bool {
        self.statements.iter().any(|st| {
            st.subject() == subject && st.predicate() == predicate && st.object() == object
        })
    }

    fn statements(&self) -> Vec<Rc<Statement>> {
        self.statements.to_vec()
    }

    fn statements_for(&self, subject: &SubjectNode) -> Vec<Rc<Statement>> {
        self.statements
            .iter()
            .filter(|st| st.subject() == subject)
            .cloned()
            .collect()
    }

    fn subjects(&self) -> HashSet<&SubjectNode> {
        self.statements.iter().map(|st| st.subject()).collect()
    }

    fn predicates(&self) -> HashSet<&IRI> {
        self.statements.iter().map(|st| st.predicate()).collect()
    }

    fn predicates_for(&self, subject: &SubjectNode) -> HashSet<&IRI> {
        self.statements
            .iter()
            .filter_map(|st| {
                if st.subject() == subject {
                    Some(st.predicate())
                } else {
                    None
                }
            })
            .collect()
    }

    fn objects(&self) -> HashSet<&ObjectNode> {
        self.statements.iter().map(|st| st.object()).collect()
    }

    fn objects_for(&self, subject: &SubjectNode, predicate: &IRI) -> HashSet<&ObjectNode> {
        self.statements
            .iter()
            .filter_map(|st| {
                if st.subject() == subject && st.predicate() == predicate {
                    Some(st.object())
                } else {
                    None
                }
            })
            .collect()
    }

    fn resource_for(&self, subject: &SubjectNode) -> Resource {
        let mut resource = Resource::new(subject);
        for st in &self.statements {
            let object = st.object();
            if object.is_literal() {
                resource.literal(st.predicate(), object.as_literal().unwrap().clone());
            } else {
                resource.resource(st.predicate(), Resource::new(subject.into()));
            }
        }
        resource
    }

    fn insert(&mut self, statement: Statement) {
        self.statements.push(Rc::new(statement));
    }

    fn merge(&mut self, other: Rc<dyn Graph>) {
        for st in other.statements() {
            self.statements.push(st)
        }
    }

    fn de_duplicate(&mut self) {
        let mut sts: HashSet<Rc<Statement>> = self.statements.drain(..).collect();
        self.statements = sts.drain().collect()
    }

    fn remove(&mut self, statement: &Statement) {
        self.statements.retain(|st| st.as_ref() != statement);
    }

    fn remove_all_for(&mut self, subject: &SubjectNode) {
        self.statements.retain(|st| st.subject() != subject);
    }

    fn clear(&mut self) {
        self.statements.clear()
    }

    fn prefix_mapping(&self) -> Rc<dyn PrefixMapping> {
        self.mappings.clone()
    }
}

impl NamedGraph for MemGraph {
    fn name(&self) -> &Option<IRI> {
        &self.name
    }

    fn set_name(&mut self, name: IRI) -> Option<IRI> {
        let old = self.name.clone();
        self.name = Some(name);
        old
    }

    fn unset_name(&mut self) -> Option<IRI> {
        let old = self.name.clone();
        self.name = None;
        old
    }
}

impl MemGraph {
    pub fn with(sts: Vec<Rc<Statement>>) -> Self {
        Self {
            name: None,
            statements: sts,
            mappings: Default::default(),
        }
    }
    pub fn named(name: IRI) -> Self {
        Self {
            name: Some(name),
            statements: Default::default(),
            mappings: Default::default(),
        }
    }
    pub fn named_with(name: IRI, sts: Vec<Rc<Statement>>) -> Self {
        Self {
            name: Some(name),
            statements: sts,
            mappings: Default::default(),
        }
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

pub mod mapping;
pub use mapping::*;
