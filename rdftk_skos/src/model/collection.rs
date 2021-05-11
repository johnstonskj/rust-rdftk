/*!
A simple model for constructing SKOS thesauri. This is not a complete API in
that it's extensibility with OWL is limited.

Details TBD

# Example

TBD

*/

use crate::model::properties::final_preferred_label;
use crate::model::ToStatement;
use crate::model::{Concept, Label, Labeled, LiteralProperty, Propertied, Resource, ToStatements};
use crate::ns;
use rdftk_core::statement::{ObjectNodeRef, StatementList, SubjectNodeRef};
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
    fn to_statements(&self, in_scheme: Option<&ObjectNodeRef>) -> StatementList {
        let mut statements: StatementList = Default::default();
        let subject = SubjectNode::named_ref(self.uri().clone());
        if self.ordered {
            statements.push(Statement::new_ref(
                subject.clone(),
                rdf::a_type().clone(),
                ObjectNode::named_ref(ns::ordered_collection().clone()),
            ));
        } else {
            statements.push(Statement::new_ref(
                subject.clone(),
                rdf::a_type().clone(),
                ObjectNode::named_ref(ns::collection().clone()),
            ));
        }
        if let Some(in_scheme) = in_scheme {
            statements.push(Statement::new_ref(
                subject.clone(),
                ns::in_scheme().clone(),
                in_scheme.clone(),
            ));
        }

        if self.has_members() {
            if self.ordered {
                let mut list_node = make_list_node(&mut statements, None);
                for (i, member) in self.members.iter().enumerate() {
                    add_to_list_node(&mut statements, &list_node, &member.uri());
                    if i < self.members.len() {
                        list_node = make_list_node(&mut statements, Some(list_node));
                    }
                }
                make_list_end(&mut statements, &list_node);
            } else {
                for member in &self.members {
                    statements.push(Statement::new_ref(
                        subject.clone(),
                        ns::member().clone(),
                        ObjectNode::named_ref(member.uri().clone()),
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

fn make_list_node(statements: &mut StatementList, from: Option<SubjectNodeRef>) -> SubjectNodeRef {
    let new_node = SubjectNode::blank_ref();
    if let Some(from) = from {
        statements.push(Statement::new_ref(
            from,
            rdf::rest().clone(),
            new_node.as_object(),
        ));
    }
    statements.push(Statement::new_ref(
        new_node.clone(),
        rdf::a_type().clone(),
        ObjectNode::named_ref(rdf::list().clone()),
    ));
    new_node
}

fn add_to_list_node(statements: &mut StatementList, current: &SubjectNodeRef, member_uri: &IRIRef) {
    statements.push(Statement::new_ref(
        current.clone(),
        rdf::first().clone(),
        ObjectNode::named_ref(member_uri.clone()),
    ));
}

fn make_list_end(statements: &mut StatementList, last: &SubjectNodeRef) {
    statements.push(Statement::new_ref(
        last.clone(),
        rdf::a_type().clone(),
        ObjectNode::named_ref(rdf::list().clone()),
    ));
    statements.push(Statement::new_ref(
        last.clone(),
        rdf::rest().clone(),
        ObjectNode::named_ref(rdf::nil().clone()),
    ));
}
