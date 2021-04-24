/*!
![memgraph](https://img.shields.io/badge/RDFtk-memgraph-BD1B89?logo=data:image/png;base64,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)
This crate provides an implementation of the `Graph` traits from `rdftk_core::graph` for simple in-memory usage.

# Example

TBD

*/

use rdftk_core::graph::{Graph, MutableGraph, PrefixMappings};
use rdftk_core::statement::{StatementList, StatementRef};
use rdftk_core::{ObjectNode, Resource, Statement, SubjectNode};
use rdftk_iri::IRIRef;
use rdftk_names::rdf;
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
    statements: StatementList,
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

impl From<StatementList> for MemGraph {
    fn from(sts: StatementList) -> Self {
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

    fn contains_subject(&self, subject: &SubjectNode) -> bool {
        self.statements
            .iter()
            .any(|st| st.as_ref().subject() == subject)
    }

    fn contains_individual(&self, subject: &IRIRef) -> bool {
        let subject: SubjectNode = subject.clone().into();
        self.objects_for(&subject, rdf::a_type()).is_empty()
    }

    fn contains(&self, statement: &Statement) -> bool {
        self.statements.iter().any(|st| st.as_ref() == statement)
    }

    fn contains_all(&self, subject: &SubjectNode, predicate: &IRIRef, object: &ObjectNode) -> bool {
        self.statements.iter().any(|st| {
            st.subject() == subject && st.predicate() == predicate && st.object() == object
        })
    }

    fn statements(&self) -> StatementList {
        self.statements.to_vec()
    }

    fn statements_for(&self, subject: &SubjectNode) -> StatementList {
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

    fn dedup(&mut self) {
        let mut sts: HashSet<StatementRef> = self.statements.drain(..).collect();
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

impl MemGraph {
    pub fn with(&mut self, statements: StatementList) -> &mut Self {
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

pub mod data_set;

pub mod mapping;
pub use mapping::*;
