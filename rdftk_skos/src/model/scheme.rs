/*!
A simple model for constructing SKOS thesauri. This is not a complete API in
that it's extensibility with OWL is limited.

Details TBD

# Example

TBD

*/

use crate::model::properties::final_preferred_label;
use crate::model::ToStatement;
use crate::model::{
    Collection, Concept, Label, Labeled, LiteralProperty, Propertied, Resource, ToStatements,
};
use crate::ns;
use rdftk_core::{ObjectNode, Statement, SubjectNode};
use rdftk_iri::IRIRef;
use rdftk_names::rdf;
use std::cell::RefCell;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Scheme {
    uri: IRIRef,
    concepts: Vec<Rc<RefCell<Concept>>>,
    collections: Vec<Rc<RefCell<Collection>>>,
    preferred_label: Option<String>,
    labels: Vec<Label>,
    properties: Vec<LiteralProperty>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Resource for Scheme {
    fn uri(&self) -> &IRIRef {
        &self.uri
    }
}

impl Propertied for Scheme {
    fn add_property(&mut self, property: LiteralProperty) {
        self.properties.push(property)
    }

    fn properties(&self) -> &Vec<LiteralProperty> {
        &self.properties
    }
}

impl Labeled for Scheme {
    fn add_label(&mut self, label: Label) {
        self.labels.push(label)
    }

    fn has_labels(&self) -> bool {
        !self.labels.is_empty()
    }

    fn labels(&self) -> &Vec<Label> {
        &self.labels
    }

    fn preferred_label(&self, for_language: &str) -> String {
        if let Some(label) = &self.preferred_label {
            label.clone()
        } else {
            match final_preferred_label(self, for_language) {
                None => self.uri().to_string(),
                Some(s) => s,
            }
        }
    }
}

impl ToStatements for Scheme {
    fn to_statements(&self, _: Option<&ObjectNode>) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Default::default();
        let subject = SubjectNode::named(self.uri().clone());
        statements.push(Statement::new(
            subject.clone(),
            rdf::a_type().clone(),
            ns::concept_scheme().into(),
        ));
        let top_concepts: Vec<String> = self
            .concepts
            .iter()
            .map(|concept| concept.borrow().uri().to_string())
            .collect();
        let in_scheme = ObjectNode::named(self.uri().clone());
        for member in &self.concepts_flattened() {
            statements.extend(member.borrow().to_statements(Some(&in_scheme)).drain(..));
            if top_concepts.contains(&member.borrow().uri().to_string()) {
                statements.push(Statement::new(
                    SubjectNode::named(member.borrow().uri().clone()),
                    ns::top_concept_of().clone(),
                    subject.clone().into(),
                ));
                statements.push(Statement::new(
                    subject.clone(),
                    ns::has_top_concept().clone(),
                    ObjectNode::named(member.borrow().uri().clone()),
                ));
            }
        }
        for member in &self.collections_flattened() {
            statements.extend(member.borrow().to_statements(Some(&in_scheme)).drain(..));
            statements.push(Statement::new(
                SubjectNode::named(member.borrow().uri().clone()),
                ns::in_scheme().clone(),
                ObjectNode::named(self.uri().clone().clone()),
            ));
        }
        for label in self.labels() {
            statements.push(label.to_statement(&subject));
        }
        for property in self.properties() {
            statements.push(property.to_statement(&subject));
        }
        statements
    }
}

impl Scheme {
    pub fn new(uri: &IRIRef) -> Self {
        Self {
            uri: uri.clone(),
            concepts: Default::default(),
            collections: Default::default(),
            preferred_label: None,
            labels: Default::default(),
            properties: Default::default(),
        }
    }

    pub fn new_with_label(uri: &IRIRef, text: &str, language: &str) -> Self {
        let mut scheme = Self::new(uri);
        scheme.add_label(Label::preferred(text, language));
        scheme
    }

    // --------------------------------------------------------------------------------------------

    pub fn new_top_concept(&mut self, uri: &IRIRef) -> Rc<RefCell<Concept>> {
        let concept = Rc::from(RefCell::from(Concept::new(uri)));
        self.add_top_concept(concept.clone());
        concept
    }

    pub fn new_top_concept_with_label(
        &mut self,
        uri: &IRIRef,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Concept>> {
        let concept = Rc::from(RefCell::from(Concept::new_with_label(uri, text, language)));
        self.add_top_concept(concept.clone());
        concept
    }

    #[inline]
    fn add_top_concept(&mut self, concept: Rc<RefCell<Concept>>) {
        self.concepts.push(concept);
    }

    pub fn has_top_concepts(&self) -> bool {
        !self.concepts.is_empty()
    }

    pub fn top_concepts(&self) -> impl Iterator<Item = &Rc<RefCell<Concept>>> {
        self.concepts.iter()
    }

    pub fn concepts_flattened(&self) -> Vec<Rc<RefCell<Concept>>> {
        self.concepts
            .iter()
            .map(|concept| {
                let mut subs = concept.borrow().concepts_flattened();
                subs.push(concept.clone());
                subs
            })
            .flatten()
            .collect()
    }

    // --------------------------------------------------------------------------------------------

    pub fn new_top_collection(&mut self, uri: &IRIRef, ordered: bool) -> Rc<RefCell<Collection>> {
        let collection = Rc::from(RefCell::from(Collection::new(uri, ordered)));
        self.add_top_collection(collection.clone());
        collection
    }

    pub fn new_top_collection_with_label(
        &mut self,
        uri: &IRIRef,
        ordered: bool,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Collection>> {
        let collection = Rc::from(RefCell::from(Collection::new_with_label(
            uri, ordered, text, language,
        )));
        self.add_top_collection(collection.clone());
        collection
    }

    #[inline]
    fn add_top_collection(&mut self, collection: Rc<RefCell<Collection>>) {
        self.collections.push(collection);
    }

    pub fn has_top_collections(&self) -> bool {
        !self.collections.is_empty()
    }

    pub fn top_collections(&self) -> impl Iterator<Item = &Rc<RefCell<Collection>>> {
        self.collections.iter()
    }

    pub fn collections_flattened(&self) -> Vec<Rc<RefCell<Collection>>> {
        self.collections
            .iter()
            .map(|collection| {
                let mut subs = collection.borrow().collections_flattened();
                subs.push(collection.clone());
                subs
            })
            .flatten()
            .collect()
    }
}
