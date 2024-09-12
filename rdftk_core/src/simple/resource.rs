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
* use rdftk_core::simple::literal::literal_factory;
* use rdftk_core::simple::resource::Resource;
* use rdftk_iri::Iri;
* use rdftk_names::{dc::elements as dc, foaf, rdf};
* use std::str::FromStr;
* use rdftk_core::model::statement::StatementList;
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
*     Resource::named(Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap().into())
*         .value_of(dc::publisher().clone(), literal_factory().literal("Wikipedia"))
*         .value_of(dc::title().clone(), literal_factory().literal("Tony Benn"))
*         .resource(
*             dc::description().clone(),
*             Resource::blank()
*                 .resource_named(rdf::a_type().clone(), foaf::person().clone())
*                 .value_of(foaf::name().clone(), literal_factory().literal("Tony Benn"))
*                 .to_owned(),
*         )
*         .to_owned();
*
* let sts: StatementList = resource.into();
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

use crate::error::{provider_mismatch_error, Result};
use crate::model::literal::{DataType, LiteralFactoryRef, LiteralRef};
use crate::model::statement::{StatementFactoryRef, StatementList, SubjectNodeRef};
use crate::simple;
use crate::simple::literal::literal_factory;
use language_tags::LanguageTag;
use rdftk_iri::IriRef;
use rdftk_names::rdf;
use simple::statement::statement_factory;
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The main resource builder type.
///
#[derive(Clone, Debug)]
pub struct Resource {
    subject: SubjectNodeRef,
    statement_factory: StatementFactoryRef,
    literal_factory: LiteralFactoryRef,
    predicates: HashMap<IriRef, RefCell<Vec<ResourceObject>>>,
}

///
/// A predicate builder type, optionally used for multi-valued predicates.
///
#[derive(Clone, Debug)]
pub struct Predicate {
    name: IriRef,
    objects: Vec<ResourceObject>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum ResourceObject {
    Resource(Resource),
    Resources(Container<Resource>),
    Literal(LiteralRef),
    Literals(Container<LiteralRef>),
}

#[derive(Clone, Debug)]
enum ContainerKind {
    Alt,
    Bag,
    Seq,
    Other(IriRef),
}

#[derive(Clone, Debug)]
struct Container<T> {
    kind: ContainerKind,
    values: Vec<T>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Predicate {
    ///
    /// Construct a new Predicate instance with the provided `Iri` name.
    ///
    pub fn new(name: IriRef) -> Self {
        Self {
            name,
            objects: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Add a new property object, a literal value, to this predicate.
    ///
    pub fn property(&mut self, value: LiteralRef) -> &mut Self {
        self.objects.push(ResourceObject::Literal(value));
        self
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// The value of this property object is a container denoting the provided values as alternatives.
    ///
    pub fn property_alternatives(&mut self, values: &[LiteralRef]) -> &mut Self {
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
    pub fn property_bag(&mut self, values: &[LiteralRef]) -> &mut Self {
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
    pub fn property_sequence(&mut self, values: &[LiteralRef]) -> &mut Self {
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
    pub fn property_container(&mut self, values: &[LiteralRef], kind: IriRef) -> &mut Self {
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
    pub fn resource_blank_named(&mut self, name: &str) -> &mut Self {
        self.objects
            .push(ResourceObject::Resource(Resource::blank_named(name)));
        self
    }

    ///
    /// Add a new resource object, an Iri, to this predicate.
    ///
    pub fn resource_named(&mut self, name: IriRef) -> &mut Self {
        self.objects
            .push(ResourceObject::Resource(Resource::named(name)));
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
    pub fn resource_container(&mut self, values: &[Resource], kind: IriRef) -> &mut Self {
        self.objects.push(ResourceObject::Resources(Container {
            kind: ContainerKind::Other(kind),
            values: values.to_vec(),
        }));
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Resource> for StatementList {
    fn from(resource: Resource) -> Self {
        let mut sts: StatementList = Vec::default();
        flatten(&resource, &mut sts);
        sts
    }
}

impl Resource {
    ///
    /// Construct a new `Resource` with the subject cloned from an existing
    /// [`SubjectNode`](../statement/struct.SubjectNode.html).
    ///
    pub fn new(subject: SubjectNodeRef) -> Self {
        assert_eq!(subject.provider_id(), simple::PROVIDER_ID);
        Self {
            subject,
            statement_factory: statement_factory(),
            literal_factory: literal_factory(),
            predicates: Default::default(),
        }
    }

    ///
    /// Construct a new `Resource` with the subject cloned from an existing
    /// [`SubjectNode`](../statement/struct.SubjectNode.html).
    ///
    pub fn with_factories(
        subject: SubjectNodeRef,
        statement_factory: StatementFactoryRef,
        literal_factory: LiteralFactoryRef,
    ) -> Result<Self> {
        if statement_factory.provider_id() != literal_factory.provider_id() {
            provider_mismatch_error(
                statement_factory.provider_id(),
                literal_factory.provider_id(),
            )
            .into()
        } else if subject.provider_id() != statement_factory.provider_id() {
            provider_mismatch_error(subject.provider_id(), statement_factory.provider_id()).into()
        } else {
            Ok(Self {
                subject,
                statement_factory,
                literal_factory,
                predicates: Default::default(),
            })
        }
    }

    ///
    /// Construct a new `Resource` with a new blank node as the subject.
    ///
    pub fn blank() -> Self {
        let statement_factory = statement_factory();
        Self {
            subject: statement_factory.blank_subject_new(),
            statement_factory,
            literal_factory: literal_factory(),
            predicates: Default::default(),
        }
    }

    ///
    /// Construct a new `Resource` with a named blank node as the subject.
    ///
    pub fn blank_named(name: &str) -> Self {
        let statement_factory = statement_factory();
        Self {
            subject: statement_factory.blank_subject_named(name).unwrap(),
            statement_factory,
            literal_factory: literal_factory(),
            predicates: Default::default(),
        }
    }

    ///
    /// Construct a new `Resource` with the provided `Iri` as the subject.
    ///
    pub fn named(name: IriRef) -> Self {
        let statement_factory = statement_factory();
        Self {
            subject: statement_factory.named_subject(name),
            statement_factory,
            literal_factory: literal_factory(),
            predicates: Default::default(),
        }
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if this instance is a resource in web terms, that is it's subject is an `Iri`.
    ///
    #[inline]
    pub fn is_a_resource(&self) -> bool {
        self.subject.is_iri()
    }

    ///
    /// Returns `true` if this instance is an individual in RDFS terms, that is it has at least one
    /// `rdf:type` predicate.
    ///
    #[inline]
    pub fn is_an_individual(&self) -> bool {
        self.predicates.keys().any(|p| p == rdf::a_type())
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
    pub fn property(&mut self, predicate: IriRef, value: LiteralRef) -> &mut Self {
        self.literal(predicate, value)
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn value_of(&mut self, predicate: IriRef, value: LiteralRef) -> &mut Self {
        self.literal(predicate, value)
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal(&mut self, predicate: IriRef, value: LiteralRef) -> &mut Self {
        self.insert(predicate, ResourceObject::Literal(value))
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal_str(&mut self, predicate: IriRef, value: &str) -> &mut Self {
        let value = self.literal_factory.literal(value);
        self.insert(predicate, ResourceObject::Literal(value))
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal_typed_str(
        &mut self,
        predicate: IriRef,
        value: &str,
        data_type: DataType,
    ) -> &mut Self {
        let value = self.literal_factory.with_data_type(value, data_type);
        self.insert(predicate, ResourceObject::Literal(value))
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal_language_str(
        &mut self,
        predicate: IriRef,
        value: &str,
        language: LanguageTag,
    ) -> &mut Self {
        let value = self.literal_factory.with_language(value, language);
        self.insert(predicate, ResourceObject::Literal(value))
    }

    ///
    /// Add a new property predicate with a literal value to this resource.
    ///
    pub fn literal_language_str_str(
        &mut self,
        predicate: IriRef,
        value: &str,
        language: &str,
    ) -> &mut Self {
        let value = self
            .literal_factory
            .with_language(value, LanguageTag::from_str(language).unwrap());
        self.insert(predicate, ResourceObject::Literal(value))
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// The value of this property predicate is a container denoting the provided values as alternatives.
    ///
    pub fn property_alternatives(&mut self, predicate: IriRef, values: &[LiteralRef]) -> &mut Self {
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
    pub fn property_bag(&mut self, predicate: IriRef, values: &[LiteralRef]) -> &mut Self {
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
    pub fn property_sequence(&mut self, predicate: IriRef, values: &[LiteralRef]) -> &mut Self {
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
        predicate: IriRef,
        values: &[LiteralRef],
        kind: IriRef,
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
    pub fn resource_blank_named(&mut self, predicate: IriRef, name: &str) -> &mut Self {
        self.insert(
            predicate,
            ResourceObject::Resource(Resource::blank_named(name)),
        )
    }

    ///
    /// Add a new resource predicate, an Iri, to this predicate.
    ///
    pub fn resource_named(&mut self, predicate: IriRef, name: IriRef) -> &mut Self {
        self.insert(predicate, ResourceObject::Resource(Resource::named(name)))
    }

    ///
    /// Add a new resource predicate, another resource, to this predicate.
    ///
    pub fn resource(&mut self, predicate: IriRef, resource: Resource) -> &mut Self {
        self.insert(predicate, ResourceObject::Resource(resource))
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// The value of this resource predicate is a container denoting the provided values as alternatives.
    ///
    pub fn resource_alternatives(&mut self, predicate: IriRef, values: &[Resource]) -> &mut Self {
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
    pub fn resource_bag(&mut self, predicate: IriRef, values: &[Resource]) -> &mut Self {
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
    pub fn resource_sequence(&mut self, predicate: IriRef, values: &[Resource]) -> &mut Self {
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
        predicate: IriRef,
        values: &[Resource],
        kind: IriRef,
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
    pub fn instance_of(&mut self, name: IriRef) -> &mut Self {
        self.insert(
            rdf::a_type().clone(),
            ResourceObject::Resource(Resource::named(name)),
        )
    }

    // --------------------------------------------------------------------------------------------

    fn insert(&mut self, predicate: IriRef, object: ResourceObject) -> &mut Self {
        if !self.predicates.contains_key(&predicate) {
            let _ = self
                .predicates
                .insert(predicate.clone(), RefCell::default());
        }
        let values = self.predicates.get(&predicate).unwrap();
        values.borrow_mut().push(object);
        self
    }
}

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
// Private Functions
// ------------------------------------------------------------------------------------------------

fn flatten(resource: &Resource, sts: &mut StatementList) {
    let subject = &resource.subject;
    for (predicate, objects) in &resource.predicates {
        let objects = objects.borrow();
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
                let container = resource.statement_factory.blank_subject_new();
                sts.push(
                    resource
                        .statement_factory
                        .statement(
                            subject.clone(),
                            predicate.clone(),
                            resource
                                .statement_factory
                                .subject_as_object(container.clone()),
                        )
                        .unwrap(),
                );
                sts.push(
                    resource
                        .statement_factory
                        .statement(
                            container.clone(),
                            rdf::a_type().clone(),
                            resource.statement_factory.named_object(match kind {
                                ContainerKind::Alt => rdf::alt().clone(),
                                ContainerKind::Bag => rdf::bag().clone(),
                                ContainerKind::Seq => rdf::seq().clone(),
                                ContainerKind::Other(iri) => iri.clone(),
                            }),
                        )
                        .unwrap(),
                );

                match object {
                    ResourceObject::Resources(rc) => {
                        for (index, resource) in rc.values.iter().enumerate() {
                            flatten(resource, sts);
                            sts.push(
                                resource
                                    .statement_factory
                                    .statement(
                                        container.clone(),
                                        rdf::member(index),
                                        resource
                                            .statement_factory
                                            .subject_as_object(resource.subject.clone()),
                                    )
                                    .unwrap(),
                            );
                        }
                    }
                    ResourceObject::Literals(lc) => {
                        for (index, literal) in lc.values.iter().enumerate() {
                            sts.push(
                                resource
                                    .statement_factory
                                    .statement(
                                        container.clone(),
                                        rdf::member(index),
                                        resource.statement_factory.literal_object(literal.clone()),
                                    )
                                    .unwrap(),
                            );
                        }
                    }
                    _ => unreachable!(),
                };
            } else {
                let statement = resource
                    .statement_factory
                    .statement(
                        subject.clone(),
                        predicate.clone(),
                        match object {
                            ResourceObject::Resource(resource) => {
                                flatten(resource, sts);
                                resource
                                    .statement_factory
                                    .subject_as_object(resource.subject.clone())
                            }
                            ResourceObject::Literal(literal) => {
                                resource.statement_factory.literal_object(literal.clone())
                            }
                            _ => unreachable!(),
                        },
                    )
                    .unwrap();
                sts.push(statement);
            }
        }
    }
}
