/*!
The [`Graph`] type implements an optionally named collection of statements.

# Example

```rust
use rdftk_core::model::graph::Graph;
use rdftk_core::model::statement::Statement;

fn simple_graph_writer(graph: &Graph)
{
    for statement in graph.statements() {
        println!("{}", statement);
    }
}
```
*/

use crate::error::Error;
use crate::model::features::{Featured, FEATURE_GRAPH_DUPLICATES, FEATURE_RDF_STAR};
use crate::model::statement::{BlankNode, ObjectNode, Statement, SubjectNode};
use bimap::BiHashMap;
use rdftk_iri::{Iri, IriExtra, Name, QName};
use rdftk_names::{dc, foaf, owl, rdf, rdfs, skos, xsd};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::iter::FusedIterator;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This type denotes the identifier for a graph in a data set; a graph name MUST be either an Iri
/// or a blank node.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GraphName {
    BNode(BlankNode),
    Iri(Iri),
}

// ------------------------------------------------------------------------------------------------

///
/// Implementation of a mapping from a prefix `Name` to an `Iri`. Prefix mappings are commonly used
/// in the serialization of graphs.
///
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PrefixMapping {
    map: BiHashMap<Option<Name>, Iri>,
}

// ------------------------------------------------------------------------------------------------

///
/// A graph is an unordered list of statements and may include duplicates.
/// Note that this trait represents an immutable graph, a type should also implement the
/// `MutableGraph` trait for mutation.
///
#[derive(Clone, Debug, Default)]
pub struct Graph {
    name: Option<GraphName>,
    statements: Statements,
    mappings: PrefixMapping,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum Statements {
    Unique(HashSet<Statement>),
    NonUnique(Vec<Statement>),
}

#[derive(Debug)]
enum StatementIter<'a> {
    Unique(std::collections::hash_set::Iter<'a, Statement>),
    NonUnique(std::slice::Iter<'a, Statement>),
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Graph Names
// ------------------------------------------------------------------------------------------------

impl Display for GraphName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::BNode(node) => format!("_:{}", node),
                Self::Iri(iri) => format!("<{}>", iri),
            }
        )
    }
}

impl From<Name> for GraphName {
    fn from(name: Name) -> Self {
        Self::BNode(BlankNode::from(name))
    }
}

impl From<&Name> for GraphName {
    fn from(name: &Name) -> Self {
        Self::BNode(BlankNode::from(name))
    }
}

impl From<BlankNode> for GraphName {
    fn from(name: BlankNode) -> Self {
        Self::BNode(name)
    }
}

impl From<&BlankNode> for GraphName {
    fn from(name: &BlankNode) -> Self {
        Self::BNode(name.clone())
    }
}

impl From<Iri> for GraphName {
    fn from(name: Iri) -> Self {
        Self::Iri(name)
    }
}

impl From<&Iri> for GraphName {
    fn from(name: &Iri) -> Self {
        Self::Iri(name.clone())
    }
}

impl From<SubjectNode> for GraphName {
    fn from(value: SubjectNode) -> Self {
        match value {
            SubjectNode::Blank(v) => Self::BNode(v.clone()),
            SubjectNode::Resource(v) => Self::Iri(v.clone()),
            _ => unreachable!(),
        }
    }
}

impl GraphName {
    ///
    /// Construct a new graph name, as a blank node with a randomly assigned name.
    ///
    pub fn blank() -> Self {
        Self::BNode(BlankNode::generate())
    }

    ///
    /// Construct a new graph name, as a blank node with the specified name.
    ///
    pub fn blank_named<S>(name: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        Ok(Self::BNode(BlankNode::from_str(name.as_ref())?))
    }

    ///
    /// Construct a new graph name, with an Iri naming a resource.
    ///
    pub fn named(name: Iri) -> Self {
        Self::Iri(name)
    }

    ///
    /// Return `true` if this graph name is a blank node, else `false`.
    ///
    pub fn is_blank(&self) -> bool {
        matches!(self, Self::BNode(_))
    }

    ///
    /// Return a blank node string, if `self.is_blank()`, else `None`.
    ///
    pub fn as_blank(&self) -> Option<&BlankNode> {
        match &self {
            Self::BNode(s) => Some(s),
            _ => None,
        }
    }

    ///
    /// Return `true` if this graph name is an Iri, else `false`.
    ///
    pub fn is_iri(&self) -> bool {
        matches!(self, Self::Iri(_))
    }

    ///
    /// Return a named node Iri, if `self.is_iri()`, else `None`.
    ///
    pub fn as_iri(&self) -> Option<&Iri> {
        match &self {
            Self::Iri(u) => Some(u),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Prefix Mappings
// ------------------------------------------------------------------------------------------------

impl PrefixMapping {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn common() -> Self {
        Self::default()
            .with_dc_elements()
            .with_owl()
            .with_rdf()
            .with_rdfs()
            .with_skos()
            .with_xsd()
    }

    ///
    /// Construct a new mapping instance with the provided default namespace.
    ///
    pub fn with_default(self, iri: Iri) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.set_default_namespace(iri);
        mut_self
    }

    ///
    /// Include the common "dc::terms" mapping.
    ///
    pub fn with_dc_terms(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_dc_terms();
        mut_self
    }

    ///
    /// Include the common "dc::elements" mapping.
    ///
    pub fn with_dc_elements(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_dc_elements();
        mut_self
    }

    ///
    /// Include the common "foaf" mapping.
    ///
    pub fn with_foaf(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_foaf();
        mut_self
    }

    ///
    /// Include the common "owl" mapping.
    ///
    pub fn with_owl(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_owl();
        mut_self
    }

    ///
    /// Include the common "rdf" mapping.
    ///
    pub fn with_rdf(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_rdf();
        mut_self
    }

    ///
    /// Include the common "rdfs" mapping.
    ///
    pub fn with_rdfs(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_rdfs();
        mut_self
    }

    ///
    /// Include the common "xsd" (XML Schema Data types) mapping.
    ///
    pub fn with_xsd(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_xsd();
        mut_self
    }

    ///
    /// Include the common "skos"  mapping.
    ///
    pub fn with_skos(self) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert_skos();
        mut_self
    }

    pub fn with(self, prefix: Name, iri: Iri) -> Self
    where
        Self: Sized,
    {
        let mut mut_self = self;
        mut_self.insert(prefix, iri);
        mut_self
    }

    // --------------------------------------------------------------------------------------------
    // Collection methods
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if there are no mappings in this instance, else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    ///
    /// Return the number of mappings in this instance.
    ///
    pub fn len(&self) -> usize {
        self.map.len()
    }

    ///
    /// Get the default namespace mapping, if present.
    ///
    pub fn get_default_namespace(&self) -> Option<&Iri> {
        self.map.get_by_left(&None)
    }

    ///
    /// Set the default namespace mapping.
    ///
    pub fn set_default_namespace(&mut self, iri: Iri) {
        let _ = self.map.insert(None, iri);
    }

    pub fn remove_default_namespace(&mut self) {
        let _ = self.map.remove_by_left(&None);
    }

    ///
    /// Get the namespace Iri associated with this provided prefix, if present.
    ///
    pub fn get_namespace(&self, prefix: &Name) -> Option<&Iri> {
        self.map.get_by_left(&Some(prefix.clone()))
    }

    ///
    /// Get the prefix associated with this provided namespace URI, if present.
    ///
    pub fn get_prefix(&self, namespace: &Iri) -> Option<&Option<Name>> {
        self.map.get_by_right(namespace)
    }

    ///
    /// Return an iterator over the contained mappings.
    ///
    pub fn mappings<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a Option<Name>, &'a Iri)> + 'a> {
        Box::new(self.map.iter())
    }

    ///
    /// Insert a mapping from the prefix string to the namespace Iri.
    ///
    pub fn insert(&mut self, prefix: Name, iri: Iri) {
        let _ = self.map.insert(Some(prefix), iri);
    }

    pub fn insert_owl(&mut self) {
        self.insert(owl::default_prefix().clone(), owl::namespace().clone());
    }

    pub fn insert_rdf(&mut self) {
        self.insert(rdf::default_prefix().clone(), rdf::namespace().clone());
    }

    pub fn insert_rdfs(&mut self) {
        self.insert(rdfs::default_prefix().clone(), rdfs::namespace().clone());
    }

    pub fn insert_xsd(&mut self) {
        self.insert(xsd::default_prefix().clone(), xsd::namespace().clone());
    }

    pub fn insert_foaf(&mut self) {
        self.insert(foaf::default_prefix().clone(), foaf::namespace().clone());
    }

    pub fn insert_dc_elements(&mut self) {
        self.insert(
            dc::elements::default_prefix().clone(),
            dc::elements::namespace().clone(),
        );
    }

    pub fn insert_dc_terms(&mut self) {
        self.insert(
            dc::terms::default_prefix().clone(),
            dc::terms::namespace().clone(),
        );
    }

    pub fn insert_skos(&mut self) {
        self.insert(skos::default_prefix().clone(), skos::namespace().clone());
    }

    pub fn insert_skos_xl(&mut self) {
        self.insert(
            skos::xl::default_prefix().clone(),
            skos::xl::namespace().clone(),
        );
    }

    pub fn insert_skos_iso(&mut self) {
        self.insert(
            skos::iso::default_prefix().clone(),
            skos::iso::namespace().clone(),
        );
    }

    ///
    /// Remove a mapping for the provided prefix. This operation has no effect if no mapping is present.
    ///
    pub fn remove(&mut self, prefix: &Name) {
        let _ = self.map.remove_by_left(&Some(prefix.clone()));
    }

    ///
    /// Remove all mappings from this instance.
    ///
    pub fn clear(&mut self) {
        self.map.clear();
    }

    // --------------------------------------------------------------------------------------------
    // QName Mapping
    // --------------------------------------------------------------------------------------------

    ///
    /// Expand a qname into an Iri, if possible.
    ///
    pub fn expand(&self, qname: &QName) -> Option<Iri> {
        let prefix = if let Some(prefix) = qname.prefix() {
            self.get_namespace(prefix)
        } else {
            self.get_default_namespace()
        };
        match prefix {
            None => None,
            Some(namespace) => namespace.make_name(qname.name().clone()),
        }
    }

    ///
    /// Compress an Iri into a qname, if possible.
    ///
    pub fn compress(&self, iri: &Iri) -> Option<QName> {
        let (iri, name) = if let Some((iri, name)) = iri.split() {
            (iri, name)
        } else {
            return None;
        };
        match self.get_prefix(&iri) {
            None => None,
            Some(None) => Some(QName::new_unqualified(name).unwrap()),
            Some(Some(prefix)) => Some(QName::new(prefix.clone(), name).unwrap()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Graphs
// ------------------------------------------------------------------------------------------------

impl Featured for Graph {
    fn supports_feature(&self, feature: &Iri) -> bool {
        (*feature == *FEATURE_GRAPH_DUPLICATES && self.statements.is_unique())
            || *feature == *FEATURE_RDF_STAR
    }
}

impl From<Statement> for Graph {
    fn from(value: Statement) -> Self {
        Self::from_iter([value])
    }
}

impl From<Vec<Statement>> for Graph {
    fn from(value: Vec<Statement>) -> Self {
        Graph::from_iter(value)
    }
}

impl FromIterator<Statement> for Graph {
    fn from_iter<T: IntoIterator<Item = Statement>>(iter: T) -> Self {
        Self {
            statements: Statements::NonUnique(iter.into_iter().collect()),
            ..Default::default()
        }
    }
}

impl Graph {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn named<N>(name: N) -> Self
    where
        N: Into<GraphName>,
    {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn unique() -> Self {
        Self {
            statements: Statements::Unique(Default::default()),
            ..Default::default()
        }
    }

    pub fn unique_named<N>(name: N) -> Self
    where
        N: Into<GraphName>,
    {
        Self {
            name: Some(name.into()),
            ..Self::unique()
        }
    }

    pub fn with_mappings(self, mappings: PrefixMapping) -> Self {
        Self { mappings, ..self }
    }

    pub fn with_statements(self, statements: Vec<Statement>) -> Self {
        Self {
            statements: match self.statements {
                Statements::Unique(_) => Statements::Unique(HashSet::from_iter(statements)),
                Statements::NonUnique(_) => Statements::NonUnique(statements),
            },
            ..self
        }
    }

    // --------------------------------------------------------------------------------------------
    // Cardinality
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if there are no statements in this graph, else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    ///
    /// Return the number of statements in this graph.
    ///
    pub fn len(&self) -> usize {
        self.statements.len()
    }

    // --------------------------------------------------------------------------------------------
    // Name
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if this graph instance has a name.
    ///
    pub fn is_named(&self) -> bool {
        self.name().is_some()
    }

    ///
    /// Return the name of this graph.
    ///
    pub fn name(&self) -> Option<&GraphName> {
        self.name.as_ref()
    }

    ///
    /// Set the name of this graph.
    ///
    pub fn set_name(&mut self, name: GraphName) {
        self.name = Some(name);
    }

    ///
    /// Remove the name of this graph.
    ///
    pub fn unset_name(&mut self) {
        self.name = None;
    }

    // --------------------------------------------------------------------------------------------
    // Query
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if this graph contains any statement with the provided subject, else `false`.
    ///
    pub fn contains_subject(&self, subject: &SubjectNode) -> bool {
        self.statements.iter().any(|st| st.subject() == subject)
    }

    ///
    /// Returns `true` if this graph contains the provided statement, else `false`.
    ///
    pub fn contains(&self, statement: &Statement) -> bool {
        !self
            .matches(
                Some(statement.subject()),
                Some(statement.predicate()),
                Some(statement.object()),
            )
            .is_empty()
    }

    ///
    /// Returns `true` if this graph contains the any statement with the provided subject,
    /// predicate, and object, else `false`.
    ///
    pub fn contains_all(
        &self,
        subject: &SubjectNode,
        predicate: &Iri,
        object: &ObjectNode,
    ) -> bool {
        !self
            .matches(Some(subject), Some(predicate), Some(object))
            .is_empty()
    }

    ///
    /// Returns `true` if this graph contains the any statement with the provided subject,
    /// predicate, and object, else `false`.
    ///
    pub fn matches(
        &self,
        subject: Option<&SubjectNode>,
        predicate: Option<&Iri>,
        object: Option<&ObjectNode>,
    ) -> HashSet<&Statement> {
        self.statements
            .iter()
            .filter(|st| {
                (subject.is_some() && st.subject() == subject.unwrap())
                    && (predicate.is_some() && st.predicate() == predicate.unwrap())
                    && (object.is_some() && st.object() == object.unwrap())
            })
            .collect()
    }

    // --------------------------------------------------------------------------------------------
    // Iterators
    // --------------------------------------------------------------------------------------------

    ///
    /// Return an iterator over all the statements in the graph.
    ///
    pub fn statements(&self) -> impl Iterator<Item = &Statement> {
        self.statements.iter()
    }

    ///
    /// Return a set of all subjects in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    pub fn subjects(&self) -> HashSet<&SubjectNode> {
        self.statements.iter().map(|st| st.subject()).collect()
    }

    ///
    /// Return a set of all subjects that are not blank nodes
    ///
    pub fn node_subjects(&self) -> HashSet<&SubjectNode> {
        self.subjects()
            .into_iter()
            .filter(|s| !s.is_blank())
            .collect()
    }

    ///
    /// Return a set of all subjects that are blank nodes
    ///
    pub fn blank_node_subjects(&self) -> HashSet<&SubjectNode> {
        self.subjects()
            .into_iter()
            .filter(|s| s.is_blank())
            .collect()
    }

    ///
    /// Return a set of all predicate in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    pub fn predicates(&self) -> HashSet<&Iri> {
        self.statements.iter().map(|st| st.predicate()).collect()
    }

    ///
    /// Return a set of all predicate referenced by the provided subject in graph, note that
    /// this is a set so that it removes duplicates.
    ///
    pub fn predicates_for(&self, subject: &SubjectNode) -> HashSet<&Iri> {
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

    ///
    /// Return a set of all objects in the graph, note that this is a set so that it removes
    /// duplicates.
    ///
    pub fn objects(&self) -> HashSet<&ObjectNode> {
        self.statements.iter().map(|st| st.object()).collect()
    }

    ///
    /// Return a set of all objects referenced by the provided subject and predicate in the graph,
    /// note that this is a set so that it removes duplicates.
    ///
    pub fn objects_for(&self, subject: &SubjectNode, predicate: &Iri) -> HashSet<&ObjectNode> {
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

    // --------------------------------------------------------------------------------------------
    // Namespace Management
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns the set of prefix mappings held by the graph.
    ///
    pub fn prefix_mappings(&self) -> &PrefixMapping {
        &self.mappings
    }

    ///
    /// Set the prefix mappings held by the graph.
    ///
    pub fn set_prefix_mappings(&mut self, mappings: PrefixMapping) {
        self.mappings = mappings;
    }

    // --------------------------------------------------------------------------------------------
    // Mutators
    // --------------------------------------------------------------------------------------------

    ///
    /// Insert a new statement into the graph.
    ///
    pub fn insert(&mut self, statement: Statement) {
        self.statements.insert(statement);
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Statement>,
    {
        self.statements.extend(iter)
    }

    ///
    /// Merge another graph into this one. Note that the graphs are required to have the same
    /// implementation type based in the type qualifiers for `StatementIter`.
    ///
    pub fn merge(&mut self, other: &Self) {
        other.statements().for_each(|st| self.insert(st.clone()))
    }

    ///
    /// Remove any duplicates within the graph, replacing any number of identical statements with
    /// just one. This will return a list of all statements removed.
    ///
    /// This method does nothing if this graph has does not support the feature
    /// `FEATURE_GRAPH_DUPLICATES` and will therefore always return an empty list.
    ///
    pub fn dedup(&mut self) -> Vec<Statement> {
        if self.statements.is_unique() {
            Default::default()
        } else {
            let (keep, discard) = self.statements.iter().fold(
                (HashSet::<Statement>::default(), Vec::default()),
                |(mut keep, mut discard), st| {
                    if keep.contains(st) {
                        discard.push(st.clone());
                    } else {
                        let _ = keep.insert(st.clone());
                    }
                    (keep, discard)
                },
            );
            self.statements = Statements::NonUnique(Vec::from_iter(keep));
            discard
        }
    }

    ///
    /// Remove any statement that matches the provided. If a graph has duplicates this method does
    /// not differentiate between them.
    ///
    pub fn remove(&mut self, statement: &Statement) {
        self.statements.remove(statement);
    }

    ///
    /// Remove all statements from this graph that have the provided subject.
    ///
    pub fn remove_all_for(&mut self, subject: &SubjectNode) -> Vec<Statement> {
        let (keep, discard) = self.statements.iter().fold(
            (Default::default(), Default::default()),
            |(mut keep, mut discard): (Vec<Statement>, Vec<Statement>), st| {
                if st.subject() == subject {
                    keep.push(st.clone());
                } else {
                    discard.push(st.clone());
                }
                (keep, discard)
            },
        );
        if self.statements.is_unique() {
            self.statements = Statements::Unique(HashSet::from_iter(keep));
        } else {
            self.statements = Statements::NonUnique(keep);
        }

        discard
    }

    ///
    /// Remove all statements from this graph.
    ///
    pub fn clear(&mut self) {
        self.statements.clear()
    }

    ///
    /// Return a new graph replacing all blank nodes with new, unique Iris. The base Iri is used to
    /// create identifiers, it's path will be replaced entirely by a well-known format.
    ///
    /// For example, given the following input graph with blank nodes:
    ///
    /// ```ttl
    /// <https://example.org/p/me> <https://example.org/v/name> _:B0f21 .
    /// _:B0f21 <https://example.org/v/firstName> "My" .
    /// _:B0f21 <https://example.org/v/lastName> "Name" .
    /// ```
    ///
    /// the call to `skolemize`,
    ///
    /// ```rust,ignore
    /// let base = Iri::from_str("https://example.com/me").unwrap();
    /// graph.skolemize(&base)
    /// ```
    ///
    /// results in a new graph containing replacement IRIs.
    ///
    /// ```ttl
    /// <https://example.org/p/me>
    ///   <https://example.org/v/name>
    ///   <https://example.com/.well-known/genid/62D22842-0D24-4911-AE7D-DF4DE06FD62F> .
    /// <https://example.com/.well-known/genid/62D22842-0D24-4911-AE7D-DF4DE06FD62F>
    ///   <https://example.org/v/firstName>
    ///   "My" .
    /// <https://example.com/.well-known/genid/62D22842-0D24-4911-AE7D-DF4DE06FD62F>
    ///   <https://example.org/v/lastName>
    ///   "Name" .
    /// ```
    ///
    pub fn skolemize(&self, base: &Iri) -> Result<Self, Error> {
        let mut mapping: HashMap<BlankNode, Iri> = Default::default();

        let mut new_graph = Self::default();

        for statement in self.statements() {
            let mut new_statement = statement.clone();
            if let Some(blank) = new_statement.subject().as_blank() {
                if !mapping.contains_key(blank) {
                    let _ = mapping.insert(blank.clone(), base.genid()?);
                }
                let name = mapping.get(blank).unwrap().clone();
                let subject = SubjectNode::from(name);
                new_statement.set_subject(subject);
            }
            if let Some(blank) = new_statement.object().as_blank() {
                if !mapping.contains_key(blank) {
                    let _ = mapping.insert(blank.clone(), base.genid()?);
                }
                let name = mapping.get(blank).unwrap().clone();
                let object = ObjectNode::from(name);
                new_statement.set_object(object);
            }
            new_graph.insert(new_statement);
        }

        Ok(new_graph)
    }

    ///
    /// Return a new graph with certain features flattened to simplify the graph to a strict triple
    /// structure. For example:
    ///
    /// 1. RDF* statements in subject and object nodes are reified into the graph.
    /// 2. RDF collection objects in object nodes are reified into the graph.
    ///
    pub fn simplify(&self) -> Result<Self, Error> {
        let mut new_graph = Self::default();
        for statement in self.statements() {
            if let Some(subject_statement) = statement.subject().as_statement() {
                let (subject, statements) = subject_statement.reify()?;
                new_graph.insert(Statement::new(
                    subject,
                    statement.predicate().clone(),
                    statement.object().clone(),
                ));
                new_graph.extend(statements);
            } else if let Some(object_statement) = statement.object().as_statement() {
                let (subject, statements) = object_statement.reify()?;
                new_graph.insert(Statement::new(
                    statement.subject().clone(),
                    statement.predicate().clone(),
                    subject.to_object(),
                ));
                new_graph.extend(statements);
            } else if let Some(object_collection) = statement.object().as_collection() {
                let (subject, statements) = object_collection.reify()?;
                new_graph.insert(Statement::new(
                    statement.subject().clone(),
                    statement.predicate().clone(),
                    subject.to_object(),
                ));
                new_graph.extend(statements);
            } else {
                new_graph.insert(statement.clone());
            }
        }

        Ok(new_graph)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > Statements Enum
// ------------------------------------------------------------------------------------------------

impl Default for Statements {
    fn default() -> Self {
        Self::NonUnique(Default::default())
    }
}

impl Statements {
    fn len(&self) -> usize {
        match self {
            Self::Unique(vs) => vs.len(),
            Self::NonUnique(vs) => vs.len(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Unique(vs) => vs.is_empty(),
            Self::NonUnique(vs) => vs.is_empty(),
        }
    }

    fn insert(&mut self, st: Statement) -> bool {
        match self {
            Self::Unique(vs) => vs.insert(st),
            Self::NonUnique(vs) => {
                vs.push(st);
                true
            }
        }
    }

    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Statement>,
    {
        match self {
            Self::Unique(vs) => vs.extend(iter),
            Self::NonUnique(vs) => vs.extend(iter),
        }
    }

    fn remove(&mut self, st: &Statement) -> bool {
        match self {
            Self::Unique(vs) => vs.remove(st),
            Self::NonUnique(vs) => {
                vs.retain(|e| e != st);
                true
            }
        }
    }

    fn clear(&mut self) {
        match self {
            Self::Unique(vs) => vs.clear(),
            Self::NonUnique(vs) => vs.clear(),
        }
    }

    fn is_unique(&self) -> bool {
        matches!(self, Self::Unique(_))
    }

    fn iter(&self) -> StatementIter<'_> {
        match self {
            Self::Unique(vs) => StatementIter::Unique(vs.iter()),
            Self::NonUnique(vs) => StatementIter::NonUnique(vs.iter()),
        }
    }

    #[allow(dead_code)]
    fn into_unique(self) -> Self {
        match self {
            Statements::Unique(_) => self,
            Statements::NonUnique(vs) => Statements::Unique(HashSet::from_iter(vs)),
        }
    }

    #[allow(dead_code)]
    fn into_non_unique(self) -> Self {
        match self {
            Statements::NonUnique(_) => self,
            Statements::Unique(vs) => Statements::NonUnique(Vec::from_iter(vs)),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > Statement Iterator
// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for StatementIter<'a> {
    type Item = &'a Statement;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StatementIter::Unique(vs) => vs.next(),
            StatementIter::NonUnique(vs) => vs.next(),
        }
    }
}

impl ExactSizeIterator for StatementIter<'_> {
    fn len(&self) -> usize {
        match self {
            StatementIter::Unique(vs) => vs.len(),
            StatementIter::NonUnique(vs) => vs.len(),
        }
    }
}

impl FusedIterator for StatementIter<'_> {}
