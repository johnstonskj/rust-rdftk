/*!
Implementation of the `Resource` pattern as a kind of statement builder.

# Example

TBD

*/

use crate::{Literal, Statement, SubjectNode};
use rdftk_iri::IRI;
use rdftk_names::rdf;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum ResourceObject {
    Resource(Resource),
    Literal(Literal),
}

#[derive(Clone, Debug)]
pub struct Resource {
    subject: SubjectNode,
    predicates: HashMap<IRI, RefCell<Vec<ResourceObject>>>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Into<Vec<Statement>> for Resource {
    fn into(self) -> Vec<Statement> {
        let mut sts = Vec::default();
        flatten(&self, &mut sts);
        sts
    }
}

impl Into<Vec<Rc<Statement>>> for Resource {
    fn into(self) -> Vec<Rc<Statement>> {
        let sts: Vec<Statement> = self.into();
        sts.iter().cloned().map(Rc::new).collect()
    }
}

impl Resource {
    pub fn new(subject: &SubjectNode) -> Self {
        Self {
            subject: subject.clone(),
            predicates: Default::default(),
        }
    }
    pub fn blank() -> Self {
        Self {
            subject: SubjectNode::blank(),
            predicates: Default::default(),
        }
    }
    pub fn blank_named(name: &str) -> Self {
        Self {
            subject: SubjectNode::blank_named(name),
            predicates: Default::default(),
        }
    }
    pub fn named(name: &IRI) -> Self {
        Self {
            subject: SubjectNode::named(name),
            predicates: Default::default(),
        }
    }
    pub fn literal(&mut self, predicate: &IRI, value: Literal) -> &mut Self {
        self.insert(predicate, ResourceObject::Literal(value))
    }
    pub fn resource_blank(&mut self, predicate: &IRI) -> &mut Self {
        self.insert(predicate, ResourceObject::Resource(Resource::blank()))
    }
    pub fn resource_blank_named(&mut self, predicate: &IRI, name: &str) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Resource(Resource::blank_named(name)),
        )
    }
    pub fn resource_named(&mut self, predicate: &IRI, name: &IRI) -> &mut Self {
        self.insert(predicate, ResourceObject::Resource(Resource::named(name)))
    }
    pub fn resource(&mut self, predicate: &IRI, resource: Resource) -> &mut Self {
        self.insert(predicate, ResourceObject::Resource(resource))
    }
    pub fn is_a(&mut self, name: &IRI) -> &mut Self {
        self.insert(
            &rdf::a_type(),
            ResourceObject::Resource(Resource::named(name)),
        )
    }
    fn insert(&mut self, predicate: &IRI, object: ResourceObject) -> &mut Self {
        if !self.predicates.contains_key(predicate) {
            self.predicates
                .insert(predicate.clone(), RefCell::default());
        }
        let values = self.predicates.get(predicate).unwrap();
        values.borrow_mut().push(object);
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn flatten(resource: &Resource, sts: &mut Vec<Statement>) {
    let subject = &resource.subject;
    for (predicate, objects) in &resource.predicates {
        let objects = objects.borrow();
        for object in objects.iter() {
            let statement = Statement::new(
                subject.clone(),
                predicate.clone(),
                match object {
                    ResourceObject::Resource(resource) => resource.subject.clone().into(),
                    ResourceObject::Literal(literal) => literal.clone().into(),
                },
            );
            sts.push(statement);
            if let ResourceObject::Resource(resource) = object {
                flatten(resource, sts);
            }
        }
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
    use rdftk_names::{dc::elements as dc, foaf, rdf};
    use std::str::FromStr;

    fn contact(name: &str) -> IRI {
        IRI::from_str(&format!(
            "http://www.w3.org/2000/10/swap/pim/contact#{}",
            name
        ))
        .unwrap()
    }

    #[test]
    fn test_wpedia_example_01() {
        let mut resource =
            Resource::named(&IRI::from_str("http://www.w3.org/People/EM/contact#me").unwrap());
        resource
            .literal(&contact("fullName"), "Eric Miller".into())
            .resource_named(
                &contact("mailbox"),
                &IRI::from_str("mailto:e.miller123(at)example").unwrap(),
            )
            .literal(&contact("personalTitle"), "Dr.".into())
            .is_a(&contact("Person"));
        let sts: Vec<Statement> = resource.into();
        for st in sts {
            println!("{}", st);
        }
    }

    #[test]
    fn test_wpedia_example_02() {
        let mut resource =
            Resource::named(&IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());
        resource
            .literal(&dc::publisher(), Literal::new("Wikipedia"))
            .literal(&dc::title(), Literal::new("Tony Benn"))
            .resource(
                &dc::description(),
                Resource::blank()
                    .resource_named(&rdf::a_type(), &foaf::person())
                    .literal(&foaf::name(), Literal::new("Tony Benn"))
                    .to_owned(),
            );
        let sts: Vec<Statement> = resource.into();
        for st in sts {
            println!("{}", st);
        }
    }
}
