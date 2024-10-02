/*!
Simple, in-memory implementation of the `Graph` and `GraphFactory` traits.
*/

use crate::model::features::{Featured, FEATURE_GRAPH_DUPLICATES, FEATURE_RDF_STAR};
use crate::model::graph::{Graph, GraphFactory, GraphName, PrefixMapping};
use crate::model::statement::Statement;
use crate::model::Provided;
use crate::simple::literal::SimpleLiteral;
use crate::simple::statement::{SimpleObjectNode, SimpleStatement, SimpleSubjectNode};
use rdftk_iri::Iri;
use std::collections::HashSet;
use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `Graph` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleGraph {
    name: Option<GraphName>,
    statements: Vec<SimpleStatement>,
    mappings: PrefixMapping,
}

///
/// Simple, in-memory implementation of the `GraphFactory` trait.
///
#[derive(Clone, Debug, Default)]
pub struct SimpleGraphFactory {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleGraphFactory {
    fn provider_id(&self) -> &'static str {
        super::PROVIDER_ID
    }
}

impl GraphFactory for SimpleGraphFactory {
    type Literal = SimpleLiteral;
    type Statement = SimpleStatement;
    type Graph = SimpleGraph;

    fn graph(&self) -> SimpleGraph {
        self.create(None, &[], None)
    }

    fn named_graph(&self, name: Option<GraphName>) -> SimpleGraph {
        self.create(name, &[], None)
    }

    fn graph_from(
        &self,
        statements: &[SimpleStatement],
        prefix_mappings: Option<PrefixMapping>,
    ) -> Self::Graph {
        self.create(None, statements, prefix_mappings)
    }

    fn named_graph_from(
        &self,
        name: Option<GraphName>,
        statements: &[SimpleStatement],
        prefix_mappings: Option<PrefixMapping>,
    ) -> Self::Graph {
        self.create(name, statements, prefix_mappings)
    }
}

impl SimpleGraphFactory {
    fn create(
        &self,
        name: Option<GraphName>,
        statements: &[SimpleStatement],
        prefix_mappings: Option<PrefixMapping>,
    ) -> SimpleGraph {
        let mut graph = SimpleGraph {
            name,
            statements: statements.to_vec(),
            mappings: prefix_mappings.unwrap_or_default(),
        };

        for st in statements {
            graph.insert(st.clone());
        }

        graph
    }
}

// ----------------------------------------------------------------------------------------

impl Featured for SimpleGraph {
    fn supports_feature(&self, feature: &Iri) -> bool {
        feature == FEATURE_GRAPH_DUPLICATES.deref() || feature == FEATURE_RDF_STAR.deref()
    }
}

impl Graph for SimpleGraph {
    type Literal = SimpleLiteral;
    type Statement = SimpleStatement;

    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn contains_subject(&self, subject: &SimpleSubjectNode) -> bool {
        self.statements.iter().any(|st| st.subject() == subject)
    }

    fn matches(
        &self,
        subject: Option<&SimpleSubjectNode>,
        predicate: Option<&Iri>,
        object: Option<&SimpleObjectNode>,
    ) -> HashSet<&Self::Statement> {
        self.statements
            .iter()
            .filter(|st| {
                (subject.is_some() && st.subject() == subject.unwrap())
                    && (predicate.is_some() && st.predicate() == predicate.unwrap())
                    && (object.is_some() && st.object() == object.unwrap())
            })
            .collect()
    }

    fn statements(&self) -> impl Iterator<Item = &SimpleStatement> {
        Box::new(self.statements.iter())
    }

    fn subjects(&self) -> HashSet<&SimpleSubjectNode> {
        self.statements.iter().map(|st| st.subject()).collect()
    }

    fn predicates(&self) -> HashSet<&Iri> {
        self.statements.iter().map(|st| st.predicate()).collect()
    }

    fn predicates_for(&self, subject: &SimpleSubjectNode) -> HashSet<&Iri> {
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

    fn objects(&self) -> HashSet<&SimpleObjectNode> {
        self.statements.iter().map(|st| st.object()).collect()
    }

    fn objects_for(
        &self,
        subject: &SimpleSubjectNode,
        predicate: &Iri,
    ) -> HashSet<&SimpleObjectNode> {
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

    fn prefix_mappings(&self) -> PrefixMapping {
        self.mappings.clone()
    }

    fn set_prefix_mappings(&mut self, mappings: PrefixMapping) {
        self.mappings = mappings;
    }

    fn statements_mut(&mut self) -> impl Iterator<Item = &mut Self::Statement> {
        self.statements.iter_mut()
    }

    fn insert(&mut self, statement: SimpleStatement) {
        self.statements.push(statement);
    }

    fn merge(&mut self, other: &Self) {
        other.statements().for_each(|st| self.insert(st.clone()))
    }

    fn dedup(&mut self) -> Vec<SimpleStatement> {
        let (keep, discard) = self.statements.iter().fold(
            (HashSet::<SimpleStatement>::default(), Vec::default()),
            |(mut keep, mut discard), st| {
                if keep.contains(st) {
                    discard.push(st.clone());
                } else {
                    let _ = keep.insert(st.clone());
                }
                (keep, discard)
            },
        );
        self.statements = Vec::from_iter(keep);
        discard
    }

    fn remove(&mut self, statement: &SimpleStatement) {
        for (idx, st) in self.statements.iter().enumerate() {
            if st == statement {
                let _ = self.statements.remove(idx);
                break;
            }
        }
    }

    fn remove_all_for(&mut self, subject: &SimpleSubjectNode) -> Vec<SimpleStatement> {
        let (keep, discard) = self.statements.iter().fold(
            (Default::default(), Default::default()),
            |(mut keep, mut discard): (Vec<SimpleStatement>, Vec<SimpleStatement>), st| {
                if st.subject() == subject {
                    keep.push(st.clone());
                } else {
                    discard.push(st.clone());
                }
                (keep, discard)
            },
        );
        self.statements = keep;
        discard
    }

    fn clear(&mut self) {
        self.statements.clear()
    }

    fn name(&self) -> Option<&GraphName> {
        self.name.as_ref()
    }

    fn set_name(&mut self, name: GraphName) {
        self.name = Some(name);
    }

    fn unset_name(&mut self) {
        self.name = None;
    }
}
