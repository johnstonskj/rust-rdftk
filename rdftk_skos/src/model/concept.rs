/*!
A simple model for constructing SKOS thesauri. This is not a complete API in
that it's extensibility with OWL is limited.

Details TBD

# Example

TBD

*/

use crate::model::properties::final_preferred_label;
use crate::model::{Label, Labeled, LiteralProperty, Propertied, Resource, ToStatements};
use crate::model::{ToStatement, ToUri};
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConceptRelation {
    Narrower,
    NarrowerPartitive,
    NarrowerInstantial,
    Broader,
    BroaderPartitive,
    BroaderInstantial,
    Related,
    InverseRelated,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Concept {
    uri: IriRef,
    concepts: Vec<(ConceptRelation, Rc<RefCell<Concept>>)>,
    external_relations: Vec<(IriRef, IriRef)>,
    preferred_label: Option<String>,
    labels: Vec<Label>,
    properties: Vec<LiteralProperty>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ConceptRelation {
    fn default() -> Self {
        Self::Narrower
    }
}

impl ToUri for ConceptRelation {
    fn to_uri(&self) -> IriRef {
        match self {
            Self::Narrower => ns::narrower(),
            Self::NarrowerPartitive => ns::iso::narrower_partitive(),
            Self::NarrowerInstantial => ns::iso::narrower_instantial(),
            Self::Broader => ns::broader(),
            Self::BroaderPartitive => ns::iso::broader_partitive(),
            Self::BroaderInstantial => ns::iso::broader_instantial(),
            Self::Related => ns::related(),
            Self::InverseRelated => ns::related(),
        }
        .clone()
    }
}

impl ConceptRelation {
    pub fn inverse(&self) -> Self {
        match self {
            Self::Narrower => Self::Broader,
            Self::NarrowerPartitive => Self::BroaderPartitive,
            Self::NarrowerInstantial => Self::BroaderInstantial,
            Self::Broader => Self::Narrower,
            Self::BroaderPartitive => Self::NarrowerPartitive,
            Self::BroaderInstantial => Self::BroaderInstantial,
            Self::Related => Self::InverseRelated,
            Self::InverseRelated => Self::Related,
        }
    }

    pub fn is_narrower(&self) -> bool {
        matches!(
            self,
            Self::Narrower | Self::NarrowerPartitive | Self::NarrowerInstantial
        )
    }

    pub fn is_broader(&self) -> bool {
        matches!(
            self,
            Self::Broader | Self::BroaderPartitive | Self::BroaderInstantial
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Resource for Concept {
    fn uri(&self) -> &IriRef {
        &self.uri
    }
}

impl Propertied for Concept {
    fn add_property(&mut self, property: LiteralProperty) {
        self.properties.push(property)
    }

    fn properties(&self) -> &Vec<LiteralProperty> {
        &self.properties
    }
}

impl Labeled for Concept {
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

impl ToStatements for Concept {
    fn to_statements(
        &self,
        in_scheme: Option<&ObjectNodeRef>,
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
                    statements.named_object(ns::concept().clone()),
                )
                .unwrap(),
        );
        if let Some(in_scheme) = in_scheme {
            statement_list.push(
                statements
                    .statement(subject.clone(), ns::in_scheme().clone(), in_scheme.clone())
                    .unwrap(),
            );
        }
        for (relation, to_concept) in &self.concepts {
            statement_list.push(
                statements
                    .statement(
                        subject.clone(),
                        relation.to_uri(),
                        statements.named_object(to_concept.borrow().uri().clone()),
                    )
                    .unwrap(),
            );
        }
        for (relation, to_concept) in &self.external_relations {
            statement_list.push(
                statements
                    .statement(
                        subject.clone(),
                        relation.clone(),
                        statements.named_object(to_concept.clone()),
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

impl Concept {
    pub(crate) fn new(uri: &IriRef) -> Self {
        Self {
            uri: uri.clone(),
            concepts: Default::default(),
            external_relations: Default::default(),
            preferred_label: None,
            labels: Default::default(),
            properties: Default::default(),
        }
    }

    pub(crate) fn new_with_label(uri: &IriRef, text: &str, language: &str) -> Self {
        let mut concept = Self::new(uri);
        concept.add_label(Label::preferred(text, language));
        concept
    }

    // --------------------------------------------------------------------------------------------

    pub fn add_related_concept(
        &mut self,
        relation: ConceptRelation,
        related: Rc<RefCell<Concept>>,
    ) {
        self.concepts.push((relation, related));
    }

    pub fn sub_concept(&mut self, uri: &IriRef) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new(uri)));
        self.add_related_concept(ConceptRelation::Narrower, new_concept.clone());
        new_concept
    }

    pub fn sub_concept_with_label(
        &mut self,
        uri: &IriRef,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new_with_label(uri, text, language)));
        self.add_related_concept(ConceptRelation::Narrower, new_concept.clone());
        new_concept
    }

    pub fn instance(&mut self, uri: &IriRef) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new(uri)));
        self.add_related_concept(ConceptRelation::NarrowerInstantial, new_concept.clone());
        new_concept
    }

    pub fn instance_with_label(
        &mut self,
        uri: &IriRef,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new_with_label(uri, text, language)));
        self.add_related_concept(ConceptRelation::NarrowerInstantial, new_concept.clone());
        new_concept
    }

    pub fn part(&mut self, uri: &IriRef) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new(uri)));
        self.add_related_concept(ConceptRelation::NarrowerPartitive, new_concept.clone());
        new_concept
    }

    pub fn part_with_label(
        &mut self,
        uri: &IriRef,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new_with_label(uri, text, language)));
        self.add_related_concept(ConceptRelation::NarrowerPartitive, new_concept.clone());
        new_concept
    }

    pub fn related(&mut self, uri: &IriRef) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new(uri)));
        self.add_related_concept(ConceptRelation::Related, new_concept.clone());
        new_concept
    }

    pub fn related_with_label(
        &mut self,
        uri: &IriRef,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new_with_label(uri, text, language)));
        self.add_related_concept(ConceptRelation::Related, new_concept.clone());
        new_concept
    }

    pub fn add_related(&mut self, concept: Rc<RefCell<Self>>) {
        self.add_related_concept(ConceptRelation::Related, concept)
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_concepts(&self) -> bool {
        !self.concepts.is_empty()
    }

    pub fn concepts(&self) -> impl Iterator<Item = &(ConceptRelation, Rc<RefCell<Concept>>)> {
        self.concepts.iter()
    }

    pub fn concepts_flattened(&self) -> Vec<Rc<RefCell<Concept>>> {
        self.concepts
            .iter()
            .filter_map(|(rel, concept)| match rel {
                ConceptRelation::Narrower
                | ConceptRelation::NarrowerPartitive
                | ConceptRelation::NarrowerInstantial => {
                    let mut subs = concept.borrow().concepts_flattened();
                    subs.push(concept.clone());
                    Some(subs)
                }
                _ => None,
            })
            .flatten()
            .collect()
    }

    // --------------------------------------------------------------------------------------------

    pub fn has_external_relations(&self) -> bool {
        !self.external_relations.is_empty()
    }

    pub fn external_relations(&self) -> impl Iterator<Item = &(IriRef, IriRef)> {
        self.external_relations.iter()
    }

    pub fn add_external_relation(&mut self, relation: IriRef, related: IriRef) {
        self.external_relations.push((relation, related));
    }
}
