/*!
Traits which describe the capabilities of different `Graph` types.

# Example

TBD

*/

use crate::{ObjectNode, Resource, Statement, SubjectNode};
use rdftk_iri::IRIRef;
use std::collections::HashSet;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The core `Graph` type implemented by all model providers.
pub trait Graph {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn contains(&self, statement: &Statement) -> bool;

    fn contains_all(&self, subject: &SubjectNode, predicate: &IRIRef, object: &ObjectNode) -> bool;

    fn statements(&self) -> Vec<Rc<Statement>>;

    fn statements_for(&self, subject: &SubjectNode) -> Vec<Rc<Statement>>;

    fn subjects(&self) -> HashSet<&SubjectNode>;

    fn predicates(&self) -> HashSet<&IRIRef>;

    fn predicates_for(&self, subject: &SubjectNode) -> HashSet<&IRIRef>;

    fn objects(&self) -> HashSet<&ObjectNode>;

    fn objects_for(&self, subject: &SubjectNode, predicate: &IRIRef) -> HashSet<&ObjectNode>;

    fn resource_for(&self, subject: &SubjectNode) -> Resource;

    fn insert(&mut self, statement: Statement);

    fn merge(&mut self, other: Rc<dyn Graph>);

    fn de_duplicate(&mut self);

    fn remove(&mut self, statement: &Statement);

    fn remove_all_for(&mut self, subject: &SubjectNode);

    fn clear(&mut self);

    fn prefix_mappings(&self) -> Rc<dyn PrefixMappings>;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod named;
pub use named::*;

pub mod caching;
pub use caching::*;

pub mod mapping;
pub use mapping::*;
