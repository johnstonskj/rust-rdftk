/*!
Simple, in-memory implementation of the `Graph` and `GraphFactory` traits with support for
subject, predicate, and object, indices.
*/

use crate::model::features::{
    Featured, FEATURE_GRAPH_DUPLICATES, FEATURE_IDX_OBJECT, FEATURE_IDX_PREDICATE,
    FEATURE_IDX_SUBJECT, FEATURE_RDF_STAR,
};
use crate::model::graph::{Graph, GraphFactory, GraphName, PrefixMapping};
use crate::model::statement::Statement;
use crate::model::Provided;
use crate::simple::literal::SimpleLiteral;
use crate::simple::statement::{SimpleObjectNode, SimpleStatement, SimpleSubjectNode};
use rdftk_iri::Iri;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `Graph` trait.
///
#[derive(Clone, Debug)]
pub struct IndexedSimpleGraph {
    name: Option<GraphName>,
    statements: Vec<SimpleStatement>,
    mappings: PrefixMapping,
    s_index: HashMap<SimpleSubjectNode, Vec<SimpleStatement>>,
    p_index: HashMap<Iri, Vec<SimpleStatement>>,
    o_index: HashMap<SimpleObjectNode, Vec<SimpleStatement>>,
}

///
/// Simple, in-memory implementation of the `GraphFactory` trait.
///
#[derive(Clone, Debug, Default)]
pub struct IndexedSimpleGraphFactory {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for IndexedSimpleGraphFactory {
    fn provider_id(&self) -> &'static str {
        crate::simple::PROVIDER_ID
    }
}

impl GraphFactory for IndexedSimpleGraphFactory {
    type Literal = SimpleLiteral;
    type Statement = SimpleStatement;
    type Graph = IndexedSimpleGraph;

    fn graph(&self) -> Self::Graph {
        self.create(None, &[], None)
    }

    fn named_graph(&self, name: Option<GraphName>) -> Self::Graph {
        self.create(name, &[], None)
    }

    fn graph_from(
        &self,
        statements: &[Self::Statement],
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

impl IndexedSimpleGraphFactory {
    fn create(
        &self,
        name: Option<GraphName>,
        statements: &[SimpleStatement],
        prefix_mappings: Option<PrefixMapping>,
    ) -> IndexedSimpleGraph {
        let mut graph = IndexedSimpleGraph {
            name,
            statements: statements.to_vec(),
            mappings: prefix_mappings.unwrap_or_default(),
            s_index: Default::default(),
            p_index: Default::default(),
            o_index: Default::default(),
        };

        for st in statements {
            graph.insert(st.clone());
        }

        graph
    }
}

// ------------------------------------------------------------------------------------------------

impl Featured for IndexedSimpleGraph {
    fn supports_feature(&self, feature: &Iri) -> bool {
        feature == FEATURE_GRAPH_DUPLICATES.deref()
            || feature == FEATURE_RDF_STAR.deref()
            || feature == FEATURE_IDX_SUBJECT.deref()
            || feature == FEATURE_IDX_PREDICATE.deref()
            || feature == FEATURE_IDX_OBJECT.deref()
    }
}

impl Graph for IndexedSimpleGraph {
    type Literal = SimpleLiteral;
    type Statement = SimpleStatement;

    fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn contains_subject(&self, subject: &SimpleSubjectNode) -> bool {
        self.s_index.contains_key(subject)
    }

    fn matches(
        &self,
        subject: Option<&SimpleSubjectNode>,
        predicate: Option<&Iri>,
        object: Option<&SimpleObjectNode>,
    ) -> HashSet<&Self::Statement> {
        let s_sts: HashSet<&Self::Statement> = subject
            .map(|subject| {
                self.s_index
                    .get(subject)
                    .map(HashSet::from_iter)
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let p_sts: HashSet<&Self::Statement> = predicate
            .map(|predicate| {
                self.p_index
                    .get(predicate)
                    .map(HashSet::from_iter)
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let o_sts: HashSet<&Self::Statement> = object
            .map(|object| {
                self.o_index
                    .get(object)
                    .map(HashSet::from_iter)
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        s_sts
            .intersection(&p_sts)
            .cloned()
            .collect::<HashSet<&Self::Statement>>()
            .intersection(&o_sts)
            .cloned()
            .collect::<HashSet<&Self::Statement>>()
    }

    fn statements(&self) -> impl Iterator<Item = &Self::Statement> {
        self.statements.iter()
    }

    fn subjects(&self) -> HashSet<&SimpleSubjectNode> {
        self.s_index.keys().collect()
    }

    fn predicates(&self) -> HashSet<&Iri> {
        self.p_index.keys().collect()
    }

    fn predicates_for(&self, subject: &SimpleSubjectNode) -> HashSet<&Iri> {
        self.s_index
            .get(subject)
            .map(|sts| sts.iter().map(|st| st.predicate()).collect())
            .unwrap_or_default()
    }

    fn objects(&self) -> HashSet<&SimpleObjectNode> {
        self.o_index.keys().collect()
    }

    fn objects_for(
        &self,
        subject: &SimpleSubjectNode,
        predicate: &Iri,
    ) -> HashSet<&SimpleObjectNode> {
        self.matches(Some(subject), Some(predicate), None)
            .iter()
            .map(|st| st.object())
            .collect()
    }

    fn prefix_mappings(&self) -> PrefixMapping {
        self.mappings.clone()
    }

    fn set_prefix_mappings(&mut self, mappings: PrefixMapping) {
        self.mappings = mappings;
    }

    fn statements_mut(&mut self) -> impl Iterator<Item = &mut Self::Statement> {
        Box::new(self.statements.iter_mut())
    }

    fn insert(&mut self, statement: Self::Statement) {
        match self.s_index.get_mut(statement.subject()) {
            None => {
                let _ = self
                    .s_index
                    .insert(statement.subject().clone(), vec![statement.clone()]);
            }
            Some(sts) => {
                sts.push(statement.clone());
            }
        }
        match self.p_index.get_mut(statement.predicate()) {
            None => {
                let _ = self
                    .p_index
                    .insert(statement.predicate().clone(), vec![statement.clone()]);
            }
            Some(sts) => {
                sts.push(statement.clone());
            }
        }
        match self.o_index.get_mut(statement.object()) {
            None => {
                let _ = self
                    .o_index
                    .insert(statement.object().clone(), vec![statement.clone()]);
            }
            Some(sts) => {
                sts.push(statement.clone());
            }
        }
        self.statements.push(statement);
    }

    fn merge(&mut self, other: &Self)
    where
        Self: Sized,
    {
        other.statements().for_each(|st| self.insert(st.clone()))
    }

    fn dedup(&mut self) -> Vec<Self::Statement> {
        let (keep, discard) = self.statements.iter().fold(
            (
                HashSet::<Self::Statement>::default(),
                Vec::<Self::Statement>::default(),
            ),
            |(mut keep, mut discard), st| {
                if keep.contains(st) {
                    discard.push(st.clone());
                } else {
                    let _ = keep.insert(st.clone());
                }
                (keep, discard)
            },
        );
        self.statements = keep.into_iter().collect::<Vec<Self::Statement>>();
        for st in &discard {
            self.remove_indices_for(st);
        }
        discard
    }

    fn remove(&mut self, statement: &Self::Statement) {
        for (idx, st) in self.statements.iter().enumerate() {
            if st == statement {
                let _ = self.statements.remove(idx);
                self.remove_indices_for(statement);
                break;
            }
        }
    }

    fn remove_all_for(&mut self, subject: &SimpleSubjectNode) -> Vec<Self::Statement> {
        let sts: Option<Vec<Self::Statement>> = self.s_index.get(subject).map(|sts| sts.to_vec());
        if let Some(sts) = sts {
            for st in &sts {
                self.remove(st)
            }
            sts
        } else {
            Default::default()
        }
    }

    fn clear(&mut self) {
        self.statements.clear();
        self.s_index.clear();
        self.p_index.clear();
        self.o_index.clear();
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

impl IndexedSimpleGraph {
    fn remove_indices_for(&mut self, statement: &SimpleStatement) {
        Self::remove_from_index(statement, statement.subject(), &mut self.s_index);
        Self::remove_from_index(statement, statement.predicate(), &mut self.p_index);
        Self::remove_from_index(statement, statement.object(), &mut self.o_index);
    }

    fn remove_from_index<T: Eq + Hash>(
        statement: &SimpleStatement,
        key: &T,
        index: &mut HashMap<T, Vec<SimpleStatement>>,
    ) {
        let _ = index.get_mut(key).map(|sts| {
            sts.iter().position(|st| st == statement).map(|idx| {
                let _ = sts.remove(idx);
            })
        });
    }
}
