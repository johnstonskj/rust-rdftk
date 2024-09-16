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
use rdftk_core::model::literal::{LanguageTag, LiteralFactoryRef};
use rdftk_core::model::statement::{ObjectNodeRef, StatementFactoryRef, StatementList};
use rdftk_iri::IriRef;
use rdftk_names::rdf;
use std::cell::RefCell;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Scheme {
    uri: IriRef,
    concepts: Vec<Concept>,
    collections: Vec<Collection>,
    labels: Vec<Label>,
    properties: Vec<LiteralProperty>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Resource for Scheme {
    fn uri(&self) -> &IriRef {
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

    fn get_preferred_label_for(&self, for_language: &Option<LanguageTag>) -> String {
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
    fn to_statements(
        &self,
        _: Option<&ObjectNodeRef>,
        statements: &StatementFactoryRef,
        literals: &LiteralFactoryRef,
    ) -> StatementList {
        let mut statement_list: StatementList = Default::default();
        let subject = statements.named_subject(self.uri().clone());
        statement_list.push(
            statements
                .statement(
                    subject.clone(),
                    rdf::a_type().clone(),
                    statements.named_object(ns::concept_scheme().clone()),
                )
                .unwrap(),
        );
        let top_concepts: Vec<String> = self
            .concepts
            .iter()
            .map(|concept| concept.borrow().uri().to_string())
            .collect();
        let in_scheme = statements.named_object(self.uri().clone());
        for member in &self.concepts_flattened() {
            statement_list.extend(
                member
                    .borrow()
                    .to_statements(Some(&in_scheme), statements, literals)
                    .drain(..),
            );
            if top_concepts.contains(&member.borrow().uri().to_string()) {
                statement_list.push(
                    statements
                        .statement(
                            statements.named_subject(member.borrow().uri().clone()),
                            ns::top_concept_of().clone(),
                            statements.subject_as_object(subject.clone()),
                        )
                        .unwrap(),
                );
                statement_list.push(
                    statements
                        .statement(
                            subject.clone(),
                            ns::has_top_concept().clone(),
                            statements.named_object(member.borrow().uri().clone()),
                        )
                        .unwrap(),
                );
            }
        }
        for member in &self.collections_flattened() {
            statement_list.extend(
                member
                    .borrow()
                    .to_statements(Some(&in_scheme), statements, literals)
                    .drain(..),
            );
            statement_list.push(
                statements
                    .statement(
                        statements.named_subject(member.borrow().uri().clone()),
                        ns::in_scheme().clone(),
                        statements.named_object(self.uri().clone().clone()),
                    )
                    .unwrap(),
            );
        }
        for label in self.labels() {
            statement_list.push(label.to_statement(&subject, statements, literals));
        }
        for property in self.properties() {
            statement_list.push(property.to_statement(&subject, statements, literals));
        }
        statement_list
    }
}

impl Scheme {
    pub fn new(uri: &IriRef) -> Self {
        Self {
            uri: uri.clone(),
            concepts: Default::default(),
            collections: Default::default(),
            preferred_label: None,
            labels: Default::default(),
            properties: Default::default(),
        }
    }

    pub fn new_with_label(uri: &IriRef, text: &str, language: &str) -> Self {
        let mut scheme = Self::new(uri);
        scheme.add_label(Label::preferred(text, language));
        scheme
    }

    // --------------------------------------------------------------------------------------------

    pub fn new_top_concept(&mut self, uri: &IriRef) -> Rc<RefCell<Concept>> {
        let concept = Rc::from(RefCell::from(Concept::new(uri)));
        self.add_top_concept(concept.clone());
        concept
    }

    pub fn new_top_concept_with_label(
        &mut self,
        uri: &IriRef,
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

    pub fn new_top_collection(&mut self, uri: &IriRef, ordered: bool) -> Rc<RefCell<Collection>> {
        let collection = Rc::from(RefCell::from(Collection::new(uri, ordered)));
        self.add_top_collection(collection.clone());
        collection
    }

    pub fn new_top_collection_with_label(
        &mut self,
        uri: &IriRef,
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
