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
use rdftk_core::model::literal::{LanguageTag, LiteralFactoryRef};
use rdftk_core::model::statement::{
    ObjectNodeRef, StatementFactoryRef, StatementList, SubjectNodeRef,
};
use rdftk_iri::IriRef;
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
    uri: IriRef,
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
    fn uri(&self) -> IriRef {
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
    fn uri(&self) -> &IriRef {
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

impl ToStatements for Collection {
    fn to_statements(
        &self,
        in_scheme: Option<&ObjectNodeRef>,
        statements: &StatementFactoryRef,
        literals: &LiteralFactoryRef,
    ) -> StatementList {
        let mut statement_list: StatementList = Default::default();
        let subject = statements.named_subject(self.uri().clone());
        if self.ordered {
            statement_list.push(
                statements
                    .statement(
                        subject.clone(),
                        rdf::a_type().clone(),
                        statements.named_object(ns::ordered_collection().clone()),
                    )
                    .unwrap(),
            );
        } else {
            statement_list.push(
                statements
                    .statement(
                        subject.clone(),
                        rdf::a_type().clone(),
                        statements.named_object(ns::collection().clone()),
                    )
                    .unwrap(),
            );
        }
        if let Some(in_scheme) = in_scheme {
            statement_list.push(
                statements
                    .statement(subject.clone(), ns::in_scheme().clone(), in_scheme.clone())
                    .unwrap(),
            );
        }

        if self.has_members() {
            if self.ordered {
                let mut list_node = make_list_node(&mut statement_list, None, statements);
                for (i, member) in self.members.iter().enumerate() {
                    add_to_list_node(&mut statement_list, &list_node, &member.uri(), statements);
                    if i < self.members.len() {
                        list_node =
                            make_list_node(&mut statement_list, Some(list_node), statements);
                    }
                }
                make_list_end(&mut statement_list, &list_node, statements);
            } else {
                for member in &self.members {
                    statement_list.push(
                        statements
                            .statement(
                                subject.clone(),
                                ns::member().clone(),
                                statements.named_object(member.uri().clone()),
                            )
                            .unwrap(),
                    );
                }
            }
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

impl Collection {
    pub(crate) fn new(uri: &IriRef, ordered: bool) -> Self {
        Self {
            uri: uri.clone(),
            ordered,
            members: Default::default(),
            preferred_label: None,
            labels: Default::default(),
            properties: Default::default(),
        }
    }

    pub(crate) fn new_with_label(uri: &IriRef, ordered: bool, text: &str, language: &str) -> Self {
        let mut collection = Self::new(uri, ordered);
        collection.add_label(Label::preferred(text, language));
        collection
    }

    // --------------------------------------------------------------------------------------------

    #[inline]
    fn add_member_collection(&mut self, collection: Rc<RefCell<Collection>>) {
        self.members.push(Member::Collection(collection));
    }

    pub fn sub_collection(&mut self, uri: &IriRef, ordered: bool) -> Rc<RefCell<Collection>> {
        let member = Rc::from(RefCell::from(Self::new(uri, ordered)));
        self.add_member_collection(member.clone());
        member
    }

    pub fn sub_collection_labeled(
        &mut self,
        uri: &IriRef,
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

    pub fn has_member(&self, uri: &IriRef) -> bool {
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

fn make_list_node(
    statements: &mut StatementList,
    from: Option<SubjectNodeRef>,
    factory: &StatementFactoryRef,
) -> SubjectNodeRef {
    let new_node = factory.blank_subject_new();
    if let Some(from) = from {
        statements.push(
            factory
                .statement(
                    from,
                    rdf::rest().clone(),
                    factory.subject_as_object(new_node.clone()),
                )
                .unwrap(),
        );
    }
    statements.push(
        factory
            .statement(
                new_node.clone(),
                rdf::a_type().clone(),
                factory.named_object(rdf::list().clone()),
            )
            .unwrap(),
    );
    new_node
}

fn add_to_list_node(
    statements: &mut StatementList,
    current: &SubjectNodeRef,
    member_uri: &IriRef,
    factory: &StatementFactoryRef,
) {
    statements.push(
        factory
            .statement(
                current.clone(),
                rdf::first().clone(),
                factory.named_object(member_uri.clone()),
            )
            .unwrap(),
    );
}

fn make_list_end(
    statements: &mut StatementList,
    last: &SubjectNodeRef,
    factory: &StatementFactoryRef,
) {
    statements.push(
        factory
            .statement(
                last.clone(),
                rdf::a_type().clone(),
                factory.named_object(rdf::list().clone()),
            )
            .unwrap(),
    );
    statements.push(
        factory
            .statement(
                last.clone(),
                rdf::rest().clone(),
                factory.named_object(rdf::nil().clone()),
            )
            .unwrap(),
    );
}
