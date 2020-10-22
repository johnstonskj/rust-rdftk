/*!
A simple model for constructing SKOS thesauri. This is not a complete API in
that it's extensibility with OWL is limited.

Details TBD

# Example

TBD

*/

use crate::ns;
use crate::simple::properties::final_preferred_label;
use crate::simple::{Label, Labeled, LiteralProperty, Named, Propertied, ToStatements};
use crate::simple::{ToStatement, ToURI};
use rdftk_core::{ObjectNode, Statement, SubjectNode};
use rdftk_iri::IRIRef;
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
    //    Other(IRIRef),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Concept {
    uri: IRIRef,
    concepts: Vec<(ConceptRelation, Rc<RefCell<Concept>>)>,
    //    inverse: Vec<(ConceptRelation, Rc<RefCell<Concept>>)>,
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

impl ToURI for ConceptRelation {
    fn to_uri(&self) -> IRIRef {
        match self {
            Self::Narrower => ns::narrower(),
            Self::NarrowerPartitive => ns::iso::narrower_partitive(),
            Self::NarrowerInstantial => ns::iso::narrower_instantial(),
            Self::Broader => ns::broader(),
            Self::BroaderPartitive => ns::iso::broader_partitive(),
            Self::BroaderInstantial => ns::iso::broader_instantial(),
            Self::Related => ns::related(),
            Self::InverseRelated => ns::related(),
            //            Self::Other(iri) => iri,
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
}

// ------------------------------------------------------------------------------------------------

impl Named for Concept {
    fn uri(&self) -> &IRIRef {
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

    fn preferred_label(&self, for_language: &str) -> String {
        if let Some(label) = &self.preferred_label {
            label.clone()
        } else {
            match final_preferred_label(self, for_language) {
                None => self.uri().to_string(),
                Some(s) => s.clone(),
            }
        }
    }

    fn has_labels(&self) -> bool {
        !self.labels.is_empty()
    }

    fn labels(&self) -> &Vec<Label> {
        &self.labels
    }
}

impl ToStatements for Concept {
    fn to_statements(&self, in_scheme: Option<&ObjectNode>) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Default::default();
        let subject = SubjectNode::named(self.uri().clone());
        statements.push(Statement::new(
            subject.clone(),
            rdf::a_type().clone(),
            ns::concept().into(),
        ));
        if let Some(in_scheme) = in_scheme {
            statements.push(Statement::new(
                subject.clone(),
                ns::in_scheme().clone(),
                in_scheme.clone(),
            ));
        }
        for (relation, to_concept) in &self.concepts {
            statements.push(Statement::new(
                subject.clone(),
                relation.to_uri(),
                to_concept.borrow().uri().clone().into(),
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

impl Concept {
    pub(crate) fn new(uri: &IRIRef) -> Self {
        Self {
            uri: uri.clone(),
            concepts: Default::default(),
            //            inverse: Default::default(),
            preferred_label: None,
            labels: Default::default(),
            properties: Default::default(),
        }
    }

    pub(crate) fn new_with_label(uri: &IRIRef, text: &str, language: &str) -> Self {
        let mut concept = Self::new(uri);
        concept.add_label(Label::preferred(text, language));
        concept
    }

    pub fn add_related_concept(
        &mut self,
        relation: ConceptRelation,
        related: Rc<RefCell<Concept>>,
    ) {
        self.concepts.push((relation, related));
    }

    // --------------------------------------------------------------------------------------------

    pub fn sub_concept(&mut self, uri: &IRIRef) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new(uri)));
        self.add_related_concept(ConceptRelation::Narrower, new_concept.clone());
        new_concept
    }

    pub fn sub_concept_with_label(
        &mut self,
        uri: &IRIRef,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new_with_label(uri, text, language)));
        self.add_related_concept(ConceptRelation::Narrower, new_concept.clone());
        new_concept
    }

    pub fn instance(&mut self, uri: &IRIRef) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new(uri)));
        self.add_related_concept(ConceptRelation::NarrowerInstantial, new_concept.clone());
        new_concept
    }

    pub fn instance_with_label(
        &mut self,
        uri: &IRIRef,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new_with_label(uri, text, language)));
        self.add_related_concept(ConceptRelation::NarrowerInstantial, new_concept.clone());
        new_concept
    }

    pub fn part(&mut self, uri: &IRIRef) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new(uri)));
        self.add_related_concept(ConceptRelation::NarrowerPartitive, new_concept.clone());
        new_concept
    }

    pub fn part_with_label(
        &mut self,
        uri: &IRIRef,
        text: &str,
        language: &str,
    ) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new_with_label(uri, text, language)));
        self.add_related_concept(ConceptRelation::NarrowerPartitive, new_concept.clone());
        new_concept
    }

    pub fn related(&mut self, uri: &IRIRef) -> Rc<RefCell<Self>> {
        let new_concept = Rc::from(RefCell::from(Self::new(uri)));
        self.add_related_concept(ConceptRelation::Related, new_concept.clone());
        new_concept
    }

    pub fn related_with_label(
        &mut self,
        uri: &IRIRef,
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
}
