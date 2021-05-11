/*!
One-line description.

More detailed description, with

# Example

# Specification


*/

use crate::{Individual, LabelProperty, Labeled, Resource, Subclassed, ToStatements};
use rdftk_core::{ObjectNode, Statement, SubjectNode};
use rdftk_iri::{IRIRef, IRI};
use rdftk_names::{rdf, rdfs};
use std::collections::HashMap;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types

#[derive(Clone, Debug)]
pub struct Vocabulary {
    uri: IRIRef,
    label_properties: Vec<LabelProperty>,
    classes: HashMap<IRIRef, Class>,
    properties: HashMap<IRIRef, Property>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    uri: IRIRef,
    label_properties: Vec<LabelProperty>,
    instance_of: Vec<IRIRef>,
    parents: Vec<IRIRef>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    uri: IRIRef,
    label_properties: Vec<LabelProperty>,
    instance_of: Vec<IRIRef>,
    parents: Vec<IRIRef>,
    domain: Vec<IRIRef>,
    range: Vec<IRIRef>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn rdf_schema() -> Vocabulary {
    let mut schema = Vocabulary::new(rdfs::namespace_iri().clone());
    let iri = IRIRef::from(IRI::from_str("https://www.w3.org/TR/rdf-schema").unwrap());
    schema.add_is_defined_by(iri.into());
    schema.add_comment("W3C Recommendation 25 February 2014".into());

    schema.add_class(Class::new(rdfs::resource().clone()));
    schema.add_class(Class::new(rdfs::class().clone()));
    schema.add_class(Class::new_subclass(
        rdfs::literal().clone(),
        rdfs::resource().clone(),
    ));
    schema.add_class(Class::new_subclass(
        rdfs::data_type().clone(),
        rdfs::class().clone(),
    ));
    schema.add_class(Class::new_instance_and_subclass(
        rdfs::lang_string().clone(),
        rdfs::data_type().clone(),
        rdfs::literal().clone(),
    ));
    schema.add_class(Class::new_instance_and_subclass(
        rdfs::html_literal().clone(),
        rdfs::data_type().clone(),
        rdfs::literal().clone(),
    ));
    schema.add_class(Class::new_instance_and_subclass(
        rdfs::xml_literal().clone(),
        rdfs::data_type().clone(),
        rdfs::literal().clone(),
    ));
    schema.add_class(Class::new_subclass(
        rdfs::property().clone(),
        rdfs::class().clone(),
    ));

    schema.add_property(Property::new_with(
        rdfs::range().clone(),
        rdfs::property().clone(),
        rdfs::class().clone(),
    ));
    schema.add_property(Property::new_with(
        rdfs::domain().clone(),
        rdfs::property().clone(),
        rdfs::class().clone(),
    ));
    schema.add_property(Property::new_with(
        rdf::a_type().clone(),
        rdfs::resource().clone(),
        rdfs::class().clone(),
    ));
    schema.add_property(Property::new_with(
        rdfs::subclass_of().clone(),
        rdfs::class().clone(),
        rdfs::class().clone(),
    ));
    schema.add_property(Property::new_with(
        rdfs::subproperty_of().clone(),
        rdfs::property().clone(),
        rdfs::property().clone(),
    ));
    schema.add_property(Property::new_with(
        rdfs::label().clone(),
        rdfs::resource().clone(),
        rdfs::literal().clone(),
    ));
    schema.add_property(Property::new_with(
        rdfs::comment().clone(),
        rdfs::resource().clone(),
        rdfs::literal().clone(),
    ));

    schema.add_class(Class::new(rdfs::container().clone()));
    schema.add_class(Class::new_subclass(
        rdf::bag().clone(),
        rdfs::container().clone(),
    ));
    schema.add_class(Class::new_subclass(
        rdf::seq().clone(),
        rdfs::container().clone(),
    ));
    schema.add_class(Class::new_subclass(
        rdf::alt().clone(),
        rdfs::container().clone(),
    ));

    schema.add_class(Class::new(rdf::list().clone()));
    schema.add_property(Property::new_with(
        rdf::first().clone(),
        rdf::list().clone(),
        rdfs::resource().clone(),
    ));
    schema.add_property(Property::new_with(
        rdf::rest().clone(),
        rdf::list().clone(),
        rdf::list().clone(),
    ));
    schema.add_class(Class::new_instance(rdf::nil().clone(), rdf::list().clone()));
    schema.add_class(Class::new_subclass(
        rdfs::container_membership_property().clone(),
        rdfs::property().clone(),
    ));

    schema.add_class(Class::new(rdf::statement().clone()));
    schema.add_property(Property::new_with(
        rdf::subject().clone(),
        rdf::statement().clone(),
        rdfs::resource().clone(),
    ));
    schema.add_property(Property::new_with(
        rdf::predicate().clone(),
        rdf::statement().clone(),
        rdfs::resource().clone(),
    ));
    schema.add_property(Property::new_with(
        rdf::object().clone(),
        rdf::statement().clone(),
        rdfs::resource().clone(),
    ));

    schema.add_property(Property::new_with(
        rdfs::see_also().clone(),
        rdfs::resource().clone(),
        rdfs::resource().clone(),
    ));
    schema.add_property(Property::new_with(
        rdfs::is_defined_by().clone(),
        rdfs::resource().clone(),
        rdfs::resource().clone(),
    ));
    schema.add_property(Property::new_with(
        rdf::value().clone(),
        rdfs::resource().clone(),
        rdfs::resource().clone(),
    ));
    schema.add_property(Property::new_with(
        rdfs::member().clone(),
        rdfs::resource().clone(),
        rdfs::resource().clone(),
    ));

    schema
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_subclassed!(Class);

impl Class {
    pub fn new(uri: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![rdfs::class().clone()],
            parents: vec![],
        }
    }

    pub fn new_instance(uri: IRIRef, instance_of: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![instance_of],
            parents: vec![],
        }
    }

    pub fn new_subclass(uri: IRIRef, parent: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![rdfs::class().clone()],
            parents: vec![parent],
        }
    }

    pub fn new_instance_and_subclass(uri: IRIRef, instance_of: IRIRef, parent: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![instance_of],
            parents: vec![parent],
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn instance(&self, uri: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![self.uri.clone()],
            parents: vec![],
        }
    }

    pub fn subclass(&self, uri: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![rdfs::class().clone()],
            parents: vec![self.uri.clone()],
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_subclassed!(Property);

impl Property {
    pub fn new(uri: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![rdfs::property().clone()],
            parents: vec![],
            domain: vec![],
            range: vec![],
        }
    }

    pub fn new_with(uri: IRIRef, domain: IRIRef, range: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![rdfs::property().clone()],
            parents: vec![],
            domain: vec![domain],
            range: vec![range],
        }
    }

    pub fn new_instance(uri: IRIRef, instance_of: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![instance_of],
            parents: vec![],
            domain: vec![],
            range: vec![],
        }
    }

    pub fn new_sub_property(uri: IRIRef, parent: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![rdfs::property().clone()],
            parents: vec![parent],
            domain: vec![],
            range: vec![],
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn instance(&self, uri: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![self.uri.clone()],
            parents: vec![],
            domain: vec![],
            range: vec![],
        }
    }

    pub fn sub_property(&self, uri: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            instance_of: vec![rdfs::property().clone()],
            parents: vec![self.uri.clone()],
            domain: vec![],
            range: vec![],
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn add_domain(&mut self, domain: IRIRef) {
        self.domain.push(domain);
    }

    pub fn remove_domain(&mut self, domain: &IRIRef) {
        self.domain.retain(|d| d != domain)
    }

    pub fn domain(&self) -> Vec<&IRIRef> {
        self.domain.iter().collect()
    }

    // --------------------------------------------------------------------------------------------

    pub fn add_range(&mut self, range: IRIRef) {
        self.range.push(range);
    }

    pub fn remove_range(&mut self, range: &IRIRef) {
        self.range.retain(|d| d != range)
    }

    pub fn range(&self) -> Vec<&IRIRef> {
        self.range.iter().collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl ToStatements for Vocabulary {
    fn to_statements(&self) -> Vec<Statement> {
        let mut results = Vec::default();

        to_label_statements(self, &mut results);

        for class in self.classes.values() {
            to_label_statements(class, &mut results);
            to_statements(class, &mut results);
        }

        for property in self.properties.values() {
            to_label_statements(property, &mut results);
            to_statements(property, &mut results);
            let subject = SubjectNode::named_ref(property.uri.clone());
            for uri in &property.domain {
                results.push(Statement::new(
                    subject.clone(),
                    rdfs::domain().clone(),
                    ObjectNode::named_ref(uri.clone()),
                ));
            }
            for uri in &property.range {
                results.push(Statement::new(
                    subject.clone(),
                    rdfs::range().clone(),
                    ObjectNode::named_ref(uri.clone()),
                ));
            }
        }

        results
    }
}

impl_labeled!(Vocabulary);

impl Vocabulary {
    pub fn new(uri: IRIRef) -> Self {
        Self {
            uri,
            label_properties: vec![],
            classes: Default::default(),
            properties: Default::default(),
        }
    }

    // ---------------------------------------------------------------------------------------------

    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.uri.clone(), class);
    }

    pub fn remove_class(&mut self, class: &Class) {
        self.classes.remove(&class.uri);
    }

    pub fn classes(&self) -> impl Iterator<Item = &Class> {
        self.classes.values()
    }

    // ---------------------------------------------------------------------------------------------

    pub fn add_property(&mut self, property: Property) {
        self.properties.insert(property.uri.clone(), property);
    }

    pub fn remove_property(&mut self, property: &Property) {
        self.properties.remove(&property.uri);
    }

    pub fn properties(&self) -> impl Iterator<Item = &Property> {
        self.properties.values()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn to_label_statements(thing: &dyn Labeled, results: &mut Vec<Statement>) {
    let subject = SubjectNode::named_ref(thing.uri().clone());
    for label in thing.label_properties() {
        match label {
            LabelProperty::Label(v) => results.push(Statement::new(
                subject.clone(),
                rdfs::label().clone(),
                ObjectNode::literal_ref(v.clone()),
            )),
            LabelProperty::Comment(v) => results.push(Statement::new(
                subject.clone(),
                rdfs::comment().clone(),
                ObjectNode::literal_ref(v.clone()),
            )),
            LabelProperty::SeeAlso(v) => results.push(Statement::new(
                subject.clone(),
                rdfs::see_also().clone(),
                ObjectNode::named_ref(v.clone()),
            )),
            LabelProperty::IsDefinedBy(v) => results.push(Statement::new(
                subject.clone(),
                rdfs::is_defined_by().clone(),
                ObjectNode::named_ref(v.clone()),
            )),
        }
    }
}

fn to_statements(thing: &dyn Subclassed, results: &mut Vec<Statement>) {
    let subject = SubjectNode::named_ref(thing.uri().clone());
    for parent in thing.instance_of() {
        results.push(Statement::new(
            subject.clone(),
            rdf::a_type().clone(),
            ObjectNode::named_ref(parent.clone()),
        ));
    }
    for parent in thing.parents() {
        results.push(Statement::new(
            subject.clone(),
            rdf::a_type().clone(),
            ObjectNode::named_ref(parent.clone()),
        ));
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        for statement in rdf_schema().to_statements() {
            println!("{}", statement);
        }
    }
}
