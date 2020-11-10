/*!
An implementation of the `Graph` traits for simple in-memory cases.

# Example

TBD

*/

use rdftk_core::graph::{Graph, MutableGraph, MutableNamedGraph, NamedGraph, PrefixMappings};
use rdftk_core::{ObjectNode, Resource, Statement, SubjectNode};
use rdftk_iri::IRIRef;
use std::collections::HashSet;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A very simple in-memory implementation of the `Graph` and `NamedGraph` traits.
///
#[derive(Clone, Debug)]
pub struct MemGraph {
    name: Option<IRIRef>,
    statements: Vec<Rc<Statement>>,
    mappings: Rc<dyn PrefixMappings>,
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
            mappings: Rc::new(Mappings::default()),
        }
    }
}

impl From<Vec<Statement>> for MemGraph {
    fn from(sts: Vec<Statement>) -> Self {
        MemGraph::default()
            .with(sts.into_iter().map(Rc::new).collect())
            .to_owned()
    }
}

impl From<Vec<Rc<Statement>>> for MemGraph {
    fn from(sts: Vec<Rc<Statement>>) -> Self {
        MemGraph::default().with(sts).to_owned()
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

    fn contains_all(&self, subject: &SubjectNode, predicate: &IRIRef, object: &ObjectNode) -> bool {
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

    fn predicates(&self) -> HashSet<&IRIRef> {
        self.statements.iter().map(|st| st.predicate()).collect()
    }

    fn predicates_for(&self, subject: &SubjectNode) -> HashSet<&IRIRef> {
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

    fn objects_for(&self, subject: &SubjectNode, predicate: &IRIRef) -> HashSet<&ObjectNode> {
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
        let mut resource = Resource::new(subject.clone());
        for st in &self.statements_for(subject) {
            let object = st.object();
            if object.is_literal() {
                resource.literal(st.predicate().clone(), object.as_literal().unwrap().clone());
            } else {
                resource.resource(st.predicate().clone(), Resource::new(subject.clone()));
            }
        }
        resource
    }

    fn prefix_mappings(&self) -> Rc<dyn PrefixMappings> {
        self.mappings.clone()
    }
}

impl MutableGraph for MemGraph {
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
}

impl NamedGraph for MemGraph {
    fn name(&self) -> &Option<IRIRef> {
        &self.name
    }
}

impl MutableNamedGraph for MemGraph {
    fn set_name(&mut self, name: IRIRef) -> Option<IRIRef> {
        let old = self.name.clone();
        self.name = Some(name);
        old
    }

    fn unset_name(&mut self) -> Option<IRIRef> {
        let old = self.name.clone();
        self.name = None;
        old
    }
}

impl MemGraph {
    pub fn named(&mut self, name: IRIRef) -> &mut Self {
        self.name = Some(name);
        self
    }
    pub fn with(&mut self, statements: Vec<Rc<Statement>>) -> &mut Self {
        self.statements = statements;
        self
    }
    pub fn mappings(&mut self, mappings: Rc<dyn PrefixMappings>) -> &mut Self {
        self.mappings = mappings;
        self
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
