/*!
A simple model for constructing SKOS thesauri. This is not a complete API in
that it's extensibility with OWL is limited.

Details TBD

# Example

TBD

*/

use crate::ns;
use crate::simple::properties::final_preferred_label;
use crate::simple::ToStatement;
use crate::simple::{Concept, Label, Labeled, LiteralProperty, Propertied, Resource, ToStatements};
use rdftk_core::{ObjectNode, Statement, SubjectNode};
use rdftk_iri::IRIRef;
use rdftk_names::rdf;
use std::cell::RefCell;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Member {
    Concept(Rc<RefCell<Concept>>),
    Collection(Rc<RefCell<Collection>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Collection {
    uri: IRIRef,
    ordered: bool,
    members: Vec<Member>,
    preferred_label: Option<String>,
    labels: Vec<Label>,
    properties: Vec<LiteralProperty>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Member {
    fn uri(&self) -> IRIRef {
        match self {
            Member::Concept(member) => member.borrow().uri().clone(),
            Member::Collection(member) => member.borrow().uri().clone(),
        }
    }

    pub fn is_concept(&self) -> bool {
        matches!(self, Self::Concept(_))
    }

    pub fn as_concept(&self) -> Option<&Rc<RefCell<Concept>>> {
        match self {
            Self::Concept(concept) => Some(concept),
            _ => None,
        }
    }

    pub fn is_collection(&self) -> bool {
        matches!(self, Self::Collection(_))
    }

    pub fn as_collection(&self) -> Option<&Rc<RefCell<Collection>>> {
        match self {
            Self::Collection(collection) => Some(collection),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Resource for Collection {
    fn uri(&self) -> &IRIRef {
        &self.uri
    }
}

impl Propertied for Collection {
    fn add_property(&mut self, property: LiteralProperty) {
        self.properties.push(property)
    }

    fn properties(&self) -> &Vec<LiteralProperty> {
        &self.properties
    }
}

impl Labeled for Collection {
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

impl ToStatements for Collection {
    fn to_statements(&self, in_scheme: Option<&ObjectNode>) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Default::default();
        let subject = SubjectNode::named(self.uri().clone());
        if self.ordered {
            statements.push(Statement::new(
                subject.clone(),
                rdf::a_type().clone(),
                ns::ordered_collection().into(),
            ));
        } else {
            statements.push(Statement::new(
                subject.clone(),
                rdf::a_type().clone(),
                ns::collection().into(),
            ));
        }
        if let Some(in_scheme) = in_scheme {
            statements.push(Statement::new(
                subject.clone(),
                ns::in_scheme().clone(),
                in_scheme.clone(),
            ));
        }

        if self.has_members() {
            if self.ordered {
                let mut list_node: SubjectNode = make_list_node(&mut statements, None);
                for (i, member) in self.members.iter().enumerate() {
                    add_to_list_node(&mut statements, &list_node, &member.uri());
                    if i < self.members.len() {
                        list_node = make_list_node(&mut statements, Some(list_node));
                    }
                }
                make_list_end(&mut statements, &list_node);
            } else {
                for member in &self.members {
                    statements.push(Statement::new(
                        subject.clone(),
                        ns::member().clone(),
                        member.uri().into(),
                    ));
                }
            }
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

impl Collection {
    pub(crate) fn new(uri: &IRIRef, ordered: bool) -> Self {
        Self {
            uri: uri.clone(),
            ordered,
            members: Default::default(),
            preferred_label: None,
            labels: Default::default(),
            properties: Default::default(),
        }
    }

    pub(crate) fn new_with_label(uri: &IRIRef, ordered: bool, text: &str, language: &str) -> Self {
        let mut collection = Self::new(uri, ordered);
        collection.add_label(Label::preferred(text, language));
        collection
    }

    // --------------------------------------------------------------------------------------------

    #[inline]
    fn add_member_collection(&mut self, collection: Rc<RefCell<Collection>>) {
        self.members.push(Member::Collection(collection));
    }

    pub fn sub_collection(&mut self, uri: &IRIRef, ordered: bool) -> Rc<RefCell<Collection>> {
        let member = Rc::from(RefCell::from(Self::new(uri, ordered)));
        self.add_member_collection(member.clone());
        member
    }

    pub fn sub_collection_labeled(
        &mut self,
        uri: &IRIRef,
        ordered: bool,
        label: &str,
        language: &str,
    ) -> Rc<RefCell<Collection>> {
        let member = Rc::from(RefCell::from(Self::new_with_label(
            uri, ordered, label, language,
        )));
        self.add_member_collection(member.clone());
        member
    }

    // --------------------------------------------------------------------------------------------

    #[inline]
    pub fn add_member_concept(&mut self, concept: Rc<RefCell<Concept>>) {
        self.members.push(Member::Concept(concept));
    }

    // --------------------------------------------------------------------------------------------

    pub fn is_ordered(&self) -> bool {
        self.ordered
    }

    pub fn has_members(&self) -> bool {
        !self.members.is_empty()
    }

    pub fn has_member(&self, uri: &IRIRef) -> bool {
        self.members.iter().any(|member| &member.uri() == uri)
    }

    pub fn members(&self) -> impl Iterator<Item = &Member> {
        self.members.iter()
    }

    pub fn collections_flattened(&self) -> Vec<Rc<RefCell<Collection>>> {
        self.members
            .iter()
            .filter_map(|member| member.as_collection())
            .map(|collection| {
                let mut subs = collection.borrow().collections_flattened();
                subs.push(collection.clone());
                subs
            })
            .flatten()
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make_list_node(statements: &mut Vec<Statement>, from: Option<SubjectNode>) -> SubjectNode {
    let new_node = SubjectNode::blank();
    if let Some(from) = from {
        statements.push(Statement::new(
            from,
            rdf::rest().clone(),
            new_node.clone().into(),
        ));
    }
    statements.push(Statement::new(
        new_node.clone(),
        rdf::a_type().clone(),
        rdf::list().clone().into(),
    ));
    new_node
}

fn add_to_list_node(statements: &mut Vec<Statement>, current: &SubjectNode, member_uri: &IRIRef) {
    statements.push(Statement::new(
        current.clone(),
        rdf::first().clone(),
        member_uri.into(),
    ));
}

fn make_list_end(statements: &mut Vec<Statement>, last: &SubjectNode) {
    statements.push(Statement::new(
        last.clone(),
        rdf::a_type().clone(),
        rdf::list().clone().into(),
    ));
    statements.push(Statement::new(
        last.clone(),
        rdf::rest().clone(),
        rdf::nil().clone().into(),
    ));
}
