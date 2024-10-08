/*!
* Implementation of the `Resource` pattern as a kind of statement builder. As a builder type the
* interface is only additive, no update or remove methods exist.
*
* Each `Resource` comprises a common subject value and a set of predicates each of which may have
* multiple associated values. It should therefore be noted that calling a predicate method will add a
* new value, and **not** replace any existing value. New resource instances can be used directly as
* the object of a predicate and so _nested_ or _related_ resources can be written inline. This is
* particulary useful where the object is a blank node.
*
* Additionally a `Predicate` builder is provided where it is more readable to add only the values
* individually rather than repeating the same predicate Iri as the `Resource` interface requires. The
* interface for `Predicate` is intended to be a precise sub-set of the `Resource` methods so that the
* same look and feel is maintained.
*
* # Example
*
* The following short example shows the use of `Resource`, and a nested resource, to build a small
* RDF model. Once a resource is created it can be converted into a vector of `Statement`s for either
* writing out or constructing a `Graph` instance.
*
* ```rust
* use rdftk_core::model::literal::Literal;
* use rdftk_core::model::resource::Resource;
* use rdftk_core::model::statement::Statement;
* use rdftk_iri::Iri;
* use rdftk_names::{dc::elements as dc, foaf, rdf};
* use std::str::FromStr;
*
* fn contact(name: &str) -> Iri {
*     Iri::from_str(&format!(
*         "http://www.w3.org/2000/10/swap/pim/contact#{}",
*         name
*     ))
*     .unwrap()
* }
*
* let resource =
*     Resource::individual(Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap())
*         .value_of(dc::publisher().clone(), Literal::plain("Wikipedia"))
*         .value_of(dc::title().clone(), Literal::plain("Tony Benn"))
*         .resource(
*             dc::description().clone(),
*             Resource::anonymous()
*                 .resource_named(rdf::a_type().clone(), foaf::person().clone())
*                 .value_of(foaf::name().clone(), Literal::plain("Tony Benn"))
*                 .to_owned(),
*         )
*         .to_owned();
*
* let sts: Vec<Statement> = resource.into();
* assert_eq!(sts.len(), 5);
*
* for st in sts {
*     println!("{}", st);
* }
* ```
*
* The output from the example above will look like this, although blank node identifiers _will_
* likely be different.
*
* ```text
* <http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/title> "Tony Benn" .
* _:B1 <http://xmlns.com/foaf/0.1/name> "Tony Benn" .
* _:B1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> .
* <http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/description> _:B1 .
* <http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/publisher> "Wikipedia" .
* ```
*
*/

use crate::model::literal::{DataType, Literal};
use crate::model::statement::BlankNode;
use crate::model::statement::Statement;
use language_tags::LanguageTag;
use rdftk_iri::Iri;
use rdftk_names::rdf;
use std::collections::HashMap;
use std::str::FromStr;

use super::statement::{ObjectNode, SubjectNode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The main resource builder type.
///
#[derive(Clone, Debug)]
pub struct Resource {
    subject: SubjectNode,
    predicate_objects: HashMap<Iri, Vec<ResourceObject>>,
}

///
/// A predicate builder type, optionally used for multi-valued predicates.
///
#[derive(Clone, Debug)]
pub struct Predicate {
    name: Iri,
    objects: Vec<ResourceObject>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum ResourceObject {
    Resource(Resource),
    Resources(Container<Resource>),
    Literal(Literal),
    Literals(Container<Literal>),
}

#[derive(Clone, Debug)]
enum ContainerKind {
    Alt,
    Bag,
    Seq,
    Other(Iri),
}

#[derive(Clone, Debug)]
struct Container<T> {
    kind: ContainerKind,
    values: Vec<T>,
}

// ------------------------------------------------------------------------------------------------
// Implementations > Resource
// ------------------------------------------------------------------------------------------------

impl From<Resource> for Vec<Statement> {
    fn from(resource: Resource) -> Self {
        let mut sts = Vec::default();
        flatten(&resource, &mut sts);
        sts
    }
}

impl Resource {
    ///
    /// Construct a new `Resource` with the subject cloned from an existing
    /// [`SubjectNode`](../statement/struct.SubjectNode.html).
    ///
    pub fn new(subject: SubjectNode) -> Self {
        Self {
            subject,
            predicate_objects: Default::default(),
        }
    }

    ///
    /// Construct a new `Resource` with a new blank node as the subject.
    ///
    pub fn anonymous() -> Self {
        Self::new(BlankNode::generate().into())
    }

    ///
    /// Construct a new `Resource` with a named blank node as the subject.
    ///
    pub fn semi_anonymous(name: BlankNode) -> Self {
        Self::new(name.into())
    }

    ///
    /// Construct a new `Resource` with the provided `Iri` as the subject.
    ///
    pub fn individual(name: Iri) -> Self {
        Self::new(name.into())
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if this instance is a resource in web terms, that is it's subject is an `Iri`.
    ///
    #[inline]
    pub fn is_a_resource(&self) -> bool {
        self.subject.is_resource()
    }

    ///
    /// Returns `true` if this instance is an individual in RDFS terms, that is it has at least one
    /// `rdf:type` predicate.
    ///
    #[inline]
    pub fn is_an_individual(&self) -> bool {
        self.predicate_objects.keys().any(|p| p == rdf::a_type())
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Add the `Predicate` instance to this resource.
    ///
    pub fn predicate(&mut self, predicate: Predicate) -> &mut Self {
        for object in predicate.objects.into_iter() {
            let _ = self.insert(predicate.name.clone(), object);
        }
        self
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn property(&mut self, predicate: Iri, value: Literal) -> &mut Self {
        self.literal(predicate, value)
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn value_of(&mut self, predicate: Iri, value: Literal) -> &mut Self {
        self.literal(predicate, value)
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal(&mut self, predicate: Iri, value: Literal) -> &mut Self {
        self.insert(predicate, ResourceObject::Literal(value))
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal_str(&mut self, predicate: Iri, value: &str) -> &mut Self {
        let value = Literal::plain(value);
        self.insert(predicate, ResourceObject::Literal(value))
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal_typed_str(
        &mut self,
        predicate: Iri,
        value: &str,
        data_type: DataType,
    ) -> &mut Self {
        let value = Literal::with_data_type(value, data_type);
        self.insert(predicate, ResourceObject::Literal(value))
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal_language_str(
        &mut self,
        predicate: Iri,
        value: &str,
        language: LanguageTag,
    ) -> &mut Self {
        let value = Literal::with_language(value, language);
        self.insert(predicate, ResourceObject::Literal(value))
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal_language_str_str(
        &mut self,
        predicate: Iri,
        value: &str,
        language: &str,
    ) -> &mut Self {
        let value = Literal::with_language(value, LanguageTag::from_str(language).unwrap());
        self.insert(predicate, ResourceObject::Literal(value))
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// The value of this property predicate is a container denoting the provided values as alternatives.
    ///
    pub fn property_alternatives(&mut self, predicate: Iri, values: &[Literal]) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Literals(Container {
                kind: ContainerKind::Alt,
                values: values.to_vec(),
            }),
        )
    }

    ///
    /// The value of this property predicate is a container denoting the provided values as an
    /// unordered bag.
    ///
    pub fn property_bag(&mut self, predicate: Iri, values: &[Literal]) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Literals(Container {
                kind: ContainerKind::Bag,
                values: values.to_vec(),
            }),
        )
    }

    ///
    /// The value of this property predicate is a container denoting the provided values as an ordered
    /// sequence.
    ///
    pub fn property_sequence(&mut self, predicate: Iri, values: &[Literal]) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Literals(Container {
                kind: ContainerKind::Seq,
                values: values.to_vec(),
            }),
        )
    }

    ///
    /// The value of this property predicate is a container of the provided values with a specified
    /// type.
    ///
    pub fn property_container(
        &mut self,
        predicate: Iri,
        values: &[Literal],
        kind: Iri,
    ) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Literals(Container {
                kind: ContainerKind::Other(kind),
                values: values.to_vec(),
            }),
        )
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Add a new resource predicate, a blank node, to this predicate.
    ///
    pub fn resource_blank_named(&mut self, predicate: Iri, name: &BlankNode) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Resource(Resource::semi_anonymous(name.clone())),
        )
    }

    ///
    /// Add a new resource predicate, an Iri, to this predicate.
    ///
    pub fn resource_named(&mut self, predicate: Iri, name: Iri) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Resource(Resource::individual(name)),
        )
    }

    ///
    /// Add a new resource predicate, another resource, to this predicate.
    ///
    pub fn resource(&mut self, predicate: Iri, resource: Resource) -> &mut Self {
        self.insert(predicate, ResourceObject::Resource(resource))
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// The value of this resource predicate is a container denoting the provided values as alternatives.
    ///
    pub fn resource_alternatives(&mut self, predicate: Iri, values: &[Resource]) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Resources(Container {
                kind: ContainerKind::Alt,
                values: values.to_vec(),
            }),
        )
    }

    ///
    /// The value of this resource predicate is a container denoting the provided values as an
    /// unordered bag.
    ///
    pub fn resource_bag(&mut self, predicate: Iri, values: &[Resource]) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Resources(Container {
                kind: ContainerKind::Bag,
                values: values.to_vec(),
            }),
        )
    }

    ///
    /// The value of this resource predicate is a container denoting the provided values as an ordered
    /// sequence.
    ///
    pub fn resource_sequence(&mut self, predicate: Iri, values: &[Resource]) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Resources(Container {
                kind: ContainerKind::Seq,
                values: values.to_vec(),
            }),
        )
    }

    ///
    /// The value of this resource predicate is a container of the provided values with a specified
    /// type.
    ///
    pub fn resource_container(
        &mut self,
        predicate: Iri,
        values: &[Resource],
        kind: Iri,
    ) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Resources(Container {
                kind: ContainerKind::Other(kind),
                values: values.to_vec(),
            }),
        )
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Set the RDF type (classifier) of this resource.
    ///
    pub fn instance_of(&mut self, name: Iri) -> &mut Self {
        self.insert(
            rdf::a_type().clone(),
            ResourceObject::Resource(Resource::individual(name)),
        )
    }

    // --------------------------------------------------------------------------------------------

    fn insert(&mut self, predicate: Iri, object: ResourceObject) -> &mut Self {
        if !self.predicate_objects.contains_key(&predicate) {
            let _ = self
                .predicate_objects
                .insert(predicate.clone(), Default::default());
        }
        let values = self.predicate_objects.get_mut(&predicate).unwrap();
        values.push(object);
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > Resource Objects
// ------------------------------------------------------------------------------------------------

impl ResourceObject {
    fn is_container(&self) -> bool {
        matches!(
            self,
            ResourceObject::Resources(_) | ResourceObject::Literals(_)
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > Predicate/Object Builder
// ------------------------------------------------------------------------------------------------

impl Predicate {
    ///
    /// Construct a new Predicate instance with the provided `Iri` name.
    ///
    pub fn new(name: Iri) -> Self {
        Self {
            name,
            objects: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Add a new property object, a literal value, to this predicate.
    ///
    pub fn property(&mut self, value: Literal) -> &mut Self {
        self.objects.push(ResourceObject::Literal(value));
        self
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// The value of this property object is a container denoting the provided values as alternatives.
    ///
    pub fn property_alternatives(&mut self, values: &[Literal]) -> &mut Self {
        self.objects.push(ResourceObject::Literals(Container {
            kind: ContainerKind::Alt,
            values: values.to_vec(),
        }));
        self
    }

    ///
    /// The value of this property object is a container denoting the provided values as an
    /// unordered bag.
    ///
    pub fn property_bag(&mut self, values: &[Literal]) -> &mut Self {
        self.objects.push(ResourceObject::Literals(Container {
            kind: ContainerKind::Bag,
            values: values.to_vec(),
        }));
        self
    }

    ///
    /// The value of this property object is a container denoting the provided values as an ordered
    /// sequence.
    ///
    pub fn property_sequence(&mut self, values: &[Literal]) -> &mut Self {
        self.objects.push(ResourceObject::Literals(Container {
            kind: ContainerKind::Seq,
            values: values.to_vec(),
        }));
        self
    }

    ///
    /// The value of this property object is a container of the provided values with a specified
    /// type.
    ///
    pub fn property_container(&mut self, values: &[Literal], kind: Iri) -> &mut Self {
        self.objects.push(ResourceObject::Literals(Container {
            kind: ContainerKind::Other(kind),
            values: values.to_vec(),
        }));
        self
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Add a new resource object, a blank node, to this predicate.
    ///
    pub fn resource_blank_named(&mut self, name: &BlankNode) -> &mut Self {
        self.objects
            .push(ResourceObject::Resource(Resource::semi_anonymous(
                name.clone(),
            )));
        self
    }

    ///
    /// Add a new resource object, an Iri, to this predicate.
    ///
    pub fn resource_named(&mut self, name: Iri) -> &mut Self {
        self.objects
            .push(ResourceObject::Resource(Resource::individual(name)));
        self
    }

    ///
    /// Add a new resource object, another resource, to this predicate.
    ///
    pub fn resource(&mut self, resource: Resource) -> &mut Self {
        self.objects.push(ResourceObject::Resource(resource));
        self
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// The value of this resource object is a container denoting the provided values as alternatives.
    ///
    pub fn resource_alternatives(&mut self, values: &[Resource]) -> &mut Self {
        self.objects.push(ResourceObject::Resources(Container {
            kind: ContainerKind::Alt,
            values: values.to_vec(),
        }));
        self
    }

    ///
    /// The value of this resource object is a container denoting the provided values as an
    /// unordered bag.
    ///
    pub fn resource_bag(&mut self, values: &[Resource]) -> &mut Self {
        self.objects.push(ResourceObject::Resources(Container {
            kind: ContainerKind::Bag,
            values: values.to_vec(),
        }));
        self
    }

    ///
    /// The value of this resource object is a container denoting the provided values as an ordered
    /// sequence.
    ///
    pub fn resource_sequence(&mut self, values: &[Resource]) -> &mut Self {
        self.objects.push(ResourceObject::Resources(Container {
            kind: ContainerKind::Seq,
            values: values.to_vec(),
        }));
        self
    }

    ///
    /// The value of this resource object is a container of the provided values with a specified
    /// type.
    ///
    pub fn resource_container(&mut self, values: &[Resource], kind: Iri) -> &mut Self {
        self.objects.push(ResourceObject::Resources(Container {
            kind: ContainerKind::Other(kind),
            values: values.to_vec(),
        }));
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn flatten(resource: &Resource, sts: &mut Vec<Statement>) {
    let subject = &resource.subject;
    for (predicate, objects) in &resource.predicate_objects {
        for object in objects.iter() {
            if object.is_container() {
                // <s> <p> {<kind>, [values]} becomes:
                //
                // <s> <p> _:b .
                // _b: rdf:type <kind>
                // _b: rdf:_1 value[1] .
                // _b: rdf:_n value[n] .
                let kind = match object {
                    ResourceObject::Resources(rc) => &rc.kind,
                    ResourceObject::Literals(lc) => &lc.kind,
                    _ => unreachable!(),
                };
                let container = SubjectNode::from(BlankNode::generate());
                sts.push(Statement::new(
                    subject.clone(),
                    predicate.clone(),
                    container.to_object(),
                ));
                sts.push(Statement::new(
                    container.clone(),
                    rdf::a_type().clone(),
                    ObjectNode::from(match kind {
                        ContainerKind::Alt => rdf::alt().clone(),
                        ContainerKind::Bag => rdf::bag().clone(),
                        ContainerKind::Seq => rdf::seq().clone(),
                        ContainerKind::Other(iri) => iri.clone(),
                    }),
                ));

                match object {
                    ResourceObject::Resources(rc) => {
                        for (index, resource) in rc.values.iter().enumerate() {
                            flatten(resource, sts);
                            sts.push(Statement::new(
                                container.clone(),
                                rdf::member(index),
                                resource.subject.to_object(),
                            ));
                        }
                    }
                    ResourceObject::Literals(lc) => {
                        for (index, literal) in lc.values.iter().enumerate() {
                            sts.push(Statement::new(
                                container.clone(),
                                rdf::member(index),
                                literal,
                            ));
                        }
                    }
                    _ => unreachable!(),
                };
            } else {
                let statement = Statement::new(
                    subject.clone(),
                    predicate.clone(),
                    match object {
                        ResourceObject::Resource(resource) => {
                            flatten(resource, sts);
                            subject.to_object()
                        }
                        ResourceObject::Literal(literal) => ObjectNode::from(literal.clone()),
                        _ => unreachable!(),
                    },
                );
                sts.push(statement);
            }
        }
    }
}
