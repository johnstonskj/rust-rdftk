/*!
Provides the `XmlReader` implementation of the `GraphReader` trait.

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::xml::syntax::{
    ATTRIBUTE_ABOUT, ATTRIBUTE_ID, ATTRIBUTE_NODE_ID, ATTRIBUTE_PARSE_TYPE, ATTRIBUTE_RESOURCE,
    ATTRIBUTE_XML_BASE, ATTRIBUTE_XML_LANG, ELEMENT_DESCRIPTION, ELEMENT_RDF,
    PARSE_TYPE_COLLECTION, PARSE_TYPE_LITERAL, PARSE_TYPE_RESOURCE, XML_NAMESPACE,
};
use crate::GraphReader;
use rdftk_core::error::{ErrorKind, Result};
use rdftk_core::model::graph::{GraphFactoryRef, GraphRef};
use rdftk_core::model::literal::{DataType, LanguageTag};
use rdftk_core::model::statement::SubjectNodeRef;
use rdftk_iri::{Iri, IriRef};
use rdftk_names::rdf;
use std::io::Read;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use xml::{EventReader, EventWriter};

///
/// An implementation of the GraphReader trait to read resources in the XML representation.
///
#[derive(Clone, Debug)]
pub struct XmlReader {}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct ExpectedName {
    local_name: String,
    namespace: String,
}

#[derive(Clone, Debug)]
enum SubjectType {
    BlankNamed(String),
    Resource(IriRef),
    RelativeResource(String),
}

#[derive(Clone, Debug)]
enum ParseType {
    XmlLiteral,
    Resource,
    Collection,
}

#[derive(Clone, Debug)]
struct Attributes<'a> {
    subject_type: Option<SubjectType>,
    parse_type: Option<ParseType>,
    uri_base: Option<IriRef>,
    data_type: Option<IriRef>,
    language: Option<LanguageTag>,
    resource: Option<IriRef>,
    inner: Vec<&'a OwnedAttribute>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for XmlReader {
    fn default() -> Self {
        Self {}
    }
}

impl GraphReader for XmlReader {
    fn read<R>(&self, r: &mut R, factory: GraphFactoryRef) -> Result<GraphRef>
    where
        R: Read,
    {
        let mut event_reader = EventReader::new(r);
        parse_document(&mut event_reader, factory)
    }
}

// ------------------------------------------------------------------------------------------------

impl ExpectedName {
    pub(crate) fn new(local_name: &str, namespace: &str) -> Self {
        Self {
            local_name: local_name.to_string(),
            namespace: namespace.to_string(),
        }
    }

    pub(crate) fn matches(&self, other: &OwnedName) -> bool {
        self.local_name == other.local_name && Some(self.namespace.clone()) == other.namespace
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

macro_rules! trace_event {
    ($fn_name:expr => $event:expr) => {
        log::trace!("XmlReader::{} event: {:?}", $fn_name, $event);
    };
    ($fn_name:expr => ignore $event:expr) => {
        log::trace!("XmlReader::{} ignoring event: {:?}", $fn_name, &$event);
    };
}

macro_rules! error_event {
    (parse => $fn_name:expr, $error:expr) => {
        let inner: rdftk_core::error::Error =
            ErrorKind::Msg(format!("error parsing XML: {:?}", $error,)).into();
        error_event!($fn_name, inner);
    };
    //(unexpected => $fn_name:expr, $event:expr) => {
    //    let inner: rdftk_core::error::Error =
    //        ErrorKind::Msg(format!("unexpected XML Event: {:?}", $event,)).into();
    //    error_event!(inner);
    //};
    (state => $fn_name:expr, $msg:expr) => {
        let inner: rdftk_core::error::Error =
            ErrorKind::Msg(format!("invalid state: {}", $msg,)).into();
        error_event!($fn_name, inner);
    };
    //(unsupported => $fn_name:expr, $feature:expr) => {
    //    let inner: rdftk_core::error::Error =
    //        ErrorKind::Msg(format!("unsupported feature/capability: {}", $feature)).into();
    //    error_event!($fn_name, inner);
    //};
    ($fn_name:expr, $inner:expr) => {
        log::error!("XmlReader::{} {}", $fn_name, $inner);
        return Err(rdftk_core::error::Error::with_chain(
            $inner,
            ErrorKind::ReadWrite(super::NAME.to_string()),
        ));
    };
}

fn parse_document<R: Read>(
    event_reader: &mut EventReader<&mut R>,
    factory: GraphFactoryRef,
) -> Result<GraphRef> {
    let mut graph = factory.graph();
    let rdf_element = ExpectedName::new(ELEMENT_RDF, rdf::namespace_str());

    loop {
        let event = event_reader.next();
        match &event {
            Ok(XmlEvent::EndDocument) => {
                trace_event!("parse_document" => event);
                return Ok(graph);
            }
            Ok(XmlEvent::StartElement {
                name,
                namespace: _,
                attributes,
            }) => {
                trace_event!("parse_document" => event);
                let attributes = parse_attributes(attributes)?;
                if rdf_element.matches(&name) {
                    let _ = parse_subject_element(
                        event_reader,
                        &attributes.uri_base,
                        None,
                        &mut graph,
                    )?;
                } else {
                    error_event!(state => "parse_document", "expecting rdf:RDF");
                }
            }
            Ok(_) => {
                trace_event!("parse_document" => ignore event);
            }
            Err(e) => {
                error_event!(parse => "parse_document", e);
            }
        }
    }
}

fn parse_subject_element<R: Read>(
    event_reader: &mut EventReader<&mut R>,
    xml_base: &Option<IriRef>,
    _subject: Option<&SubjectNodeRef>,
    graph: &mut GraphRef,
) -> Result<Option<SubjectNodeRef>> {
    let description_element = ExpectedName::new(ELEMENT_DESCRIPTION, rdf::namespace_str());
    let mut subject: Option<SubjectNodeRef> = None;
    loop {
        let event = event_reader.next();
        match &event {
            Ok(XmlEvent::StartElement {
                name,
                namespace: _,
                attributes,
            }) => {
                trace_event!("parse_subject_element" => event);
                let attributes = parse_attributes(attributes)?;
                let subject_node = match &attributes.subject_type {
                    None => {
                        // SPEC: §2.1 Introduction
                        graph.borrow().statement_factory().blank_subject_new()
                    }
                    Some(SubjectType::Resource(subject)) => {
                        // SPEC: §2.2 Node Elements and Property Elements
                        graph
                            .borrow()
                            .statement_factory()
                            .named_subject(subject.clone())
                    }
                    Some(SubjectType::RelativeResource(subject)) => {
                        // SPEC: 2.14 Abbreviating URIs: rdf:ID and xml:base
                        let uri = format!("{}{}", xml_base.as_ref().unwrap(), subject);
                        graph
                            .borrow()
                            .statement_factory()
                            .named_subject(value_to_iri(&uri)?)
                    }
                    Some(SubjectType::BlankNamed(subject)) => {
                        // SPEC: §2.10 Identifying Blank Nodes: rdf:nodeID
                        graph
                            .borrow()
                            .statement_factory()
                            .blank_subject_named(subject)
                            .unwrap()
                    }
                };
                if !description_element.matches(&name) {
                    // SPEC: §2.13 Typed Node Elements
                    let statement_factory = graph.borrow().statement_factory();
                    let mut graph = graph.borrow_mut();
                    graph.insert(
                        statement_factory
                            .statement(
                                subject_node.clone(),
                                rdf::a_type().clone(),
                                statement_factory.named_object(name_to_iri(&name)?),
                            )
                            .unwrap(),
                    );
                }
                parse_predicate_attributes(&attributes.inner, &subject_node, graph)?;
                parse_predicate_element(
                    event_reader,
                    if attributes.uri_base.is_some() {
                        &attributes.uri_base
                    } else {
                        xml_base
                    },
                    &subject_node,
                    graph,
                )?;
                // set outer loop value
                subject = Some(subject_node);
            }
            Ok(XmlEvent::EndElement { .. }) => {
                trace_event!("parse_subject_element" => event);
                break;
            }
            Ok(_) => {
                trace_event!("parse_subject_element" => ignore event);
            }
            Err(e) => {
                error_event!(parse => "parse_subject_element", e);
            }
        }
    }
    Ok(subject)
}

#[inline]
fn name_to_iri(name: &OwnedName) -> Result<IriRef> {
    Ok(IriRef::new(Iri::from_str(&format!(
        "{}{}",
        name.namespace.as_ref().unwrap(),
        name.local_name
    ))?))
}

#[inline]
fn value_to_iri(name: &str) -> Result<IriRef> {
    Ok(IriRef::new(Iri::from_str(name)?))
}

fn parse_predicate_attributes(
    attributes: &[&OwnedAttribute],
    subject: &SubjectNodeRef,
    graph: &mut GraphRef,
) -> Result<()> {
    // SPEC: §2.5 Property Attributes
    // SPEC: §2.12 Omitting Nodes: Property Attributes on an empty Property Element
    for attribute in attributes {
        log::trace!(
            "XmlReader::parse_predicate_attributes attribute: {:?}",
            attribute
        );
        let statement_factory = graph.borrow().statement_factory();
        let literal_factory = graph.borrow().literal_factory();
        let mut graph = graph.borrow_mut();
        graph.insert(
            statement_factory
                .statement(
                    subject.clone(),
                    name_to_iri(&attribute.name)?,
                    statement_factory.literal_object(literal_factory.literal(&attribute.value)),
                )
                .unwrap(),
        );
    }
    Ok(())
}

fn parse_predicate_element<R: Read>(
    event_reader: &mut EventReader<&mut R>,
    xml_base: &Option<IriRef>,
    subject: &SubjectNodeRef,
    graph: &mut GraphRef,
) -> Result<()> {
    let mut no_child_elements = false;
    loop {
        let event = event_reader.next();
        match &event {
            Ok(XmlEvent::StartElement {
                name,
                namespace: _,
                attributes,
            }) => {
                trace_event!("parse_predicate_element" => event);
                if no_child_elements {
                    error_event!(state => "parse_predicate_element", "child elements not allowed here");
                }
                let attributes = parse_attributes(attributes)?;
                if let Some(resource) = attributes.resource {
                    // SPEC: §2.4 Empty Property Elements
                    let statement_factory = graph.borrow().statement_factory();
                    let mut graph = graph.borrow_mut();
                    graph.insert(
                        statement_factory
                            .statement(
                                subject.clone(),
                                name_to_iri(&name)?,
                                statement_factory.named_object(resource),
                            )
                            .unwrap(),
                    );
                    // set outer loop value
                    no_child_elements = true;
                } else {
                    let statement_factory = graph.borrow().statement_factory();
                    let literal_factory = graph.borrow().literal_factory();
                    match attributes.parse_type {
                        None => {
                            if let Some(content) =
                                parse_object_element(event_reader, xml_base, graph)?
                            {
                                let literal = if let Some(data_type) = attributes.data_type {
                                    // SPEC: §2.9 Typed Literals: rdf:datatype
                                    literal_factory
                                        .with_data_type(&content, DataType::from(data_type))
                                } else if let Some(language) = attributes.language {
                                    // SPEC: §2.7 Languages: xml:lang
                                    literal_factory.with_language(&content, language)
                                } else {
                                    literal_factory.literal(&content)
                                };
                                graph.borrow_mut().insert(
                                    statement_factory
                                        .statement(
                                            subject.clone(),
                                            name_to_iri(&name)?,
                                            statement_factory.literal_object(literal),
                                        )
                                        .unwrap(),
                                );
                            }
                        }
                        Some(ParseType::XmlLiteral) => {
                            // SPEC: §2.8 XML Literals: rdf:parseType="Literal"
                            let content = parse_xml_literal_element(event_reader)?
                                .replace('<', "&lt;")
                                .replace('>', "&gt;");
                            graph.borrow_mut().insert(
                                statement_factory
                                    .statement(
                                        subject.clone(),
                                        name_to_iri(&name)?,
                                        statement_factory.literal_object(
                                            literal_factory
                                                .with_data_type(&content, DataType::XmlLiteral),
                                        ),
                                    )
                                    .unwrap(),
                            );
                        }
                        Some(ParseType::Resource) => {
                            // SPEC: §2.11 Omitting Blank Nodes: rdf:parseType="Resource"
                            let subject_node = statement_factory.blank_subject_new();
                            //parse_predicate_attributes(&attributes.inner, &subject_node, graph)?;
                            let _subject = parse_subject_element(
                                event_reader,
                                if attributes.uri_base.is_some() {
                                    &attributes.uri_base
                                } else {
                                    xml_base
                                },
                                Some(&subject_node),
                                graph,
                            )?;
                        }
                        Some(ParseType::Collection) => {
                            // SPEC: §2.16 Collections: rdf:parseType="Collection"
                            todo!()
                        }
                    }
                }
            }
            Ok(XmlEvent::EndElement { .. }) => {
                trace_event!("parse_predicate_element" => event);
                return Ok(());
            }
            Ok(_) => {
                trace_event!("parse_predicate_element" => ignore event);
            }
            Err(e) => {
                error_event!(parse => "parse_predicate_element", e);
            }
        }
    }
}

fn parse_object_element<R: Read>(
    event_reader: &mut EventReader<&mut R>,
    xml_base: &Option<IriRef>,
    graph: &mut GraphRef,
) -> Result<Option<String>> {
    let mut content = String::new();
    let mut has_elements = false;
    let mut has_characters = false;
    loop {
        let event = event_reader.next();
        match &event {
            Ok(XmlEvent::StartElement {
                name,
                namespace: _,
                attributes,
            }) => {
                trace_event!("parse_content_element" => event);
                if has_characters {
                    error_event!(state => "parse_object_element", &format!("found XML content, parseType != Literal ({:?})", name));
                }
                // set outer loop value
                has_elements = true;
                let attributes = parse_attributes(attributes)?;
                let subject_node = graph.borrow().statement_factory().blank_subject_new();
                let _subject = parse_subject_element(
                    event_reader,
                    if attributes.uri_base.is_some() {
                        &attributes.uri_base
                    } else {
                        xml_base
                    },
                    Some(&subject_node),
                    graph,
                )?;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                trace_event!("parse_content_element" => event);
                return Ok(Some(content));
            }
            Ok(XmlEvent::CData(value)) => {
                trace_event!("parse_content_element" => event);
                if has_elements {
                    error_event!(state => "parse_object_element", &format!("found character content after element(s)"));
                }
                // set outer loop value
                has_characters = true;
                content.push_str(&value);
            }
            Ok(XmlEvent::Characters(value)) => {
                trace_event!("parse_content_element" => event);
                if has_elements {
                    error_event!(state => "parse_object_element", "found character content after element(s)");
                }
                // set outer loop value
                has_characters = true;
                content.push_str(&value);
            }
            Ok(_) => {
                trace_event!("parse_object_element" => ignore event);
            }
            Err(e) => {
                error_event!(parse => "parse_object_element", e);
            }
        }
    }
}

fn parse_xml_literal_element<R: Read>(event_reader: &mut EventReader<&mut R>) -> Result<String> {
    let mut content: Vec<u8> = Vec::new();
    let mut writer_config = xml::EmitterConfig::new();
    writer_config.write_document_declaration = false;
    writer_config.normalize_empty_elements = true;
    let mut writer = EventWriter::new_with_config(&mut content, writer_config);
    let mut opened: u32 = 0;
    loop {
        let event = event_reader.next();
        match &event {
            Ok(XmlEvent::StartElement { .. }) => {
                trace_event!("parse_content_element" => event);
                if let Some(event) = event.unwrap().as_writer_event() {
                    writer.write(event).unwrap()
                }
                opened += 1;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                trace_event!("parse_content_element" => event);
                if let Some(event) = event.unwrap().as_writer_event() {
                    writer.write(event).unwrap()
                }
                opened -= 1;
                if opened == 0 {
                    return Ok(String::from_utf8(content).unwrap());
                }
            }
            Ok(event) => {
                // We only leave trim_whitespace=false so that these events are also written out.
                if let Some(event) = event.as_writer_event() {
                    writer.write(event).unwrap()
                }
            }
            Err(e) => {
                error_event!(parse => "parse_content_element", e);
            }
        }
    }
}

fn parse_attributes<'a>(attributes: &'a [OwnedAttribute]) -> Result<Attributes<'a>> {
    let mut response = Attributes {
        subject_type: None,
        parse_type: None,
        uri_base: None,
        data_type: None,
        language: None,
        resource: None,
        inner: Default::default(),
    };

    for attribute in attributes {
        if attribute.name.namespace == Some(XML_NAMESPACE.to_string()) {
            if attribute.name.local_name == ATTRIBUTE_XML_BASE {
                response.uri_base = Some(value_to_iri(&attribute.value)?);
            } else if attribute.name.local_name == ATTRIBUTE_XML_LANG {
                response.language = Some(LanguageTag::from_str(&attribute.value)?);
            }
        } else if attribute.name.namespace == Some(rdf::namespace_str().to_string()) {
            if attribute.name.local_name == ATTRIBUTE_ABOUT {
                response.subject_type =
                    Some(SubjectType::Resource(value_to_iri(&attribute.value)?));
            } else if attribute.name.local_name == ATTRIBUTE_NODE_ID {
                response.subject_type = Some(SubjectType::BlankNamed(attribute.value.to_string()));
            } else if attribute.name.local_name == ATTRIBUTE_ID {
                response.subject_type =
                    Some(SubjectType::RelativeResource(attribute.value.to_string()));
            } else if attribute.name.local_name == ATTRIBUTE_RESOURCE {
                response.resource = Some(value_to_iri(&attribute.value)?);
            } else if attribute.name.local_name == ATTRIBUTE_PARSE_TYPE {
                if attribute.value == PARSE_TYPE_LITERAL {
                    response.parse_type = Some(ParseType::XmlLiteral);
                } else if attribute.value == PARSE_TYPE_RESOURCE {
                    response.parse_type = Some(ParseType::Resource);
                } else if attribute.value == PARSE_TYPE_COLLECTION {
                    response.parse_type = Some(ParseType::Collection);
                } else {
                    panic!();
                }
            }
        } else {
            response.inner.push(attribute);
        }
    }

    log::trace!("parse_attributes -> {:?}", response);

    Ok(response)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
