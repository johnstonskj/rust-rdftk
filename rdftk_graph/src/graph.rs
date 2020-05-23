/*!
The core `Graph` type implemented by all model providers.

# Example

TBD

*/

#![allow(clippy::module_name_repetitions)]

use crate::PrefixMapping;
use rdftk_core::{ObjectNode, Resource, Statement, SubjectNode};
use rdftk_iri::IRI;
use std::collections::HashSet;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Graph {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn contains(&self, statement: &Statement) -> bool;

    fn contains_all(&self, subject: &SubjectNode, predicate: &IRI, object: &ObjectNode) -> bool;

    fn statements(&self) -> Vec<Rc<Statement>>;

    fn statements_for(&self, subject: &SubjectNode) -> Vec<Rc<Statement>>;

    fn subjects(&self) -> HashSet<&SubjectNode>;

    fn predicates(&self) -> HashSet<&IRI>;

    fn predicates_for(&self, subject: &SubjectNode) -> HashSet<&IRI>;

    fn objects(&self) -> HashSet<&ObjectNode>;

    fn objects_for(&self, subject: &SubjectNode, predicate: &IRI) -> HashSet<&ObjectNode>;

    fn resource_for(&self, subject: &SubjectNode) -> Resource;

    fn insert(&mut self, statement: Statement);

    fn merge(&mut self, other: Rc<dyn Graph>);

    fn de_duplicate(&mut self);

    fn remove(&mut self, statement: &Statement);

    fn remove_all_for(&mut self, subject: &SubjectNode);

    fn clear(&mut self);

    fn prefix_mapping(&self) -> Rc<dyn PrefixMapping>;
}
