use objio::HasOptions;
use super::syntax::{
    ATTRIBUTE_ABOUT, ATTRIBUTE_DATATYPE, ATTRIBUTE_NODE_ID, ATTRIBUTE_RESOURCE, DEFAULT_ENCODING,
    ELEMENT_DESCRIPTION, ELEMENT_RDF,
};
use crate::xml::syntax::ATTRIBUTE_XML_LANG_PREFIXED;
use crate::GraphWriter;
use objio::{impl_has_options, ObjectWriter};
use rdftk_core::error::{rdf_star_not_supported_error, Error};
use rdftk_core::model::graph::Graph;
use rdftk_core::model::statement::SubjectNode;
use rdftk_iri::Iri;
use rdftk_names::{dc, foaf, geo, owl, rdf, rdfs, xsd};
use std::collections::HashMap;
use std::io::Write;
use xml::common::XmlVersion;
use xml::writer::{EventWriter, XmlEvent};
use xml::EmitterConfig;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Determines the style of the generated XML.
///
#[derive(Clone, Debug, PartialEq)]
pub enum XmlStyle {
    /// Flatten the graph so all subjects are at the same level in the document.
    Flat,
    /// Nest blank nodes so that the document only has Iri subjects at the same level.
    Striped,
}

///
/// Options that control how the XML writer will render a graph.
///
#[derive(Clone, Debug)]
pub struct XmlOptions {
    /// Determines the style of the generated XML. Default is `Flat`.
    style: XmlStyle,
    /// Should the output be pretty-printed, including indentation. Default is `false`.
    pretty_print: bool,
    /// The encoding to specify in the XML declaration. Default is "utf-8".
    encoding: String,
}

///
/// A Writer to output RDF/XML.
///
#[derive(Debug)]
pub struct XmlWriter {
    mappings: HashMap<String, String>,
    options: XmlOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static::lazy_static! {
    static ref RDF_ABOUT: String = format!("{}:{}", rdf::default_prefix(), ATTRIBUTE_ABOUT);
    static ref RDF_DATATYPE: String = format!("{}:{}", rdf::default_prefix(), ATTRIBUTE_DATATYPE);
    static ref RDF_DESCRIPTION: String =
        format!("{}:{}", rdf::default_prefix(), ELEMENT_DESCRIPTION);
    static ref RDF_NODE_ID: String = format!("{}:{}", rdf::default_prefix(), ATTRIBUTE_NODE_ID);
    static ref RDF_RESOURCE: String = format!("{}:{}", rdf::default_prefix(), ATTRIBUTE_RESOURCE);
}

impl Default for XmlOptions {
    fn default() -> Self {
        Self {
            style: XmlStyle::Flat,
            pretty_print: false,
            encoding: String::from(DEFAULT_ENCODING),
        }
    }
}

impl XmlOptions {
    /// Create an option instance with `XmlStyle::Flat`.
    pub fn flat(self) -> Self {
        Self {
            style: XmlStyle::Flat,
            ..self
        }
    }

    /// Create an option instance with `XmlStyle::Striped`.
    pub fn striped(self) -> Self {
        Self {
            style: XmlStyle::Striped,
            ..self
        }
    }

    /// Set the option to emit pretty-printed XML.
    pub fn pretty(self) -> Self {
        Self {
            pretty_print: true,
            ..self
        }
    }

    /// Set the option to emit plain, non-indented, XML.
    pub fn plain(self) -> Self {
        Self {
            pretty_print: false,
            ..self
        }
    }

    /// Set the encoding string, this has no effect on the encoding being written.
    pub fn with_encoding<S>(self, encoding: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            encoding: encoding.into(),
            ..self
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for XmlWriter {
    fn default() -> Self {
        Self {
            mappings: Self::default_mappings(),
            options: Default::default(),
        }
    }
}

impl_has_options!(XmlWriter, XmlOptions);

impl XmlWriter {
    pub fn with_options(self, options: XmlOptions) -> Self {
        let mut self_mut = self;
        self_mut.set_options(options);
        self_mut
    }
}

impl ObjectWriter<Graph> for XmlWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &Graph) -> Result<(), Self::Error>
    where
        W: Write,
    {
        let config = EmitterConfig::new()
            .perform_indent(self.options.pretty_print)
            .normalize_empty_elements(self.options.pretty_print);
        let mut writer = config.create_writer(w);

        writer
            .write(XmlEvent::StartDocument {
                version: XmlVersion::Version11,
                encoding: Some(&self.options.encoding),
                standalone: None,
            })
            .map_err(xml_error)?;

        let container_name = format!("{}:{}", rdf::default_prefix(), ELEMENT_RDF);
        writer
            .write(
                XmlEvent::start_element(container_name.as_str())
                    .ns(rdf::default_prefix().as_ref(), rdf::namespace_str()),
            )
            .map_err(xml_error)?;

        let graph = graph.simplify()?;

        if self.options.style == XmlStyle::Flat {
            for subject in graph.subjects() {
                self.write_subject(&mut writer, &graph, subject, true)?;
            }
        } else {
            for subject in graph.subjects().iter().filter(|s| s.is_resource()) {
                self.write_subject(&mut writer, &graph, subject, false)?;
            }
        }

        writer
            .write(XmlEvent::end_element().name(container_name.as_str()))
            .map_err(xml_error)?;

        Ok(())
    }
}

impl GraphWriter for XmlWriter {}

impl XmlWriter {
    fn default_mappings() -> HashMap<String, String> {
        let mappings: HashMap<String, String> = [
            (
                dc::elements::namespace().to_string(),
                dc::elements::default_prefix().to_string(),
            ),
            (
                foaf::namespace().to_string(),
                foaf::default_prefix().to_string(),
            ),
            (
                geo::namespace().to_string(),
                geo::default_prefix().to_string(),
            ),
            (
                owl::namespace().to_string(),
                owl::default_prefix().to_string(),
            ),
            (
                rdf::namespace().to_string(),
                rdf::default_prefix().to_string(),
            ),
            (
                rdfs::namespace().to_string(),
                rdfs::default_prefix().to_string(),
            ),
            (
                xsd::namespace().to_string(),
                xsd::default_prefix().to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect();
        mappings
    }

    fn write_subject<W: Write>(
        &self,
        writer: &mut EventWriter<W>,
        graph: &Graph,
        subject: &SubjectNode,
        flat: bool,
    ) -> Result<(), Error> {
        if let Some(blank) = subject.as_blank() {
            if flat {
                writer
                    .write(
                        XmlEvent::start_element(RDF_DESCRIPTION.as_str())
                            .attr(RDF_NODE_ID.as_str(), blank.as_ref().as_ref()),
                    )
                    .map_err(xml_error)?;
            } else {
                writer
                    .write(XmlEvent::start_element(RDF_DESCRIPTION.as_str()))
                    .map_err(xml_error)?;
            }
        } else if let Some(subject) = subject.as_resource() {
            writer
                .write(
                    XmlEvent::start_element(RDF_DESCRIPTION.as_str())
                        .attr(RDF_ABOUT.as_str(), subject.as_ref()),
                )
                .map_err(xml_error)?;
        } else {
            return rdf_star_not_supported_error(super::NAME).into();
        }

        for predicate in graph.predicates_for(subject) {
            let (ns, mut name) = split_uri(predicate);

            for object in graph.objects_for(subject, predicate) {
                let mut element = if let Some(prefix) = self.mappings.get(&ns) {
                    name = format!("{}:{}", prefix, name);
                    XmlEvent::start_element(name.as_str()).ns(prefix, &ns)
                } else {
                    XmlEvent::start_element(name.as_str()).default_ns(&ns)
                };

                if let Some(iri) = object.as_resource() {
                    let iri = iri.to_string();
                    element = element.attr(RDF_RESOURCE.as_str(), &iri);
                    writer.write(element).map_err(xml_error)?;
                } else if let Some(blank) = object.as_blank() {
                    if flat {
                        element = element.attr(RDF_NODE_ID.as_str(), blank.as_ref().as_ref());
                        writer.write(element).map_err(xml_error)?;
                    } else {
                        writer.write(element).map_err(xml_error)?;
                        self.write_subject(writer, graph, &blank.clone().into(), flat)?;
                    }
                } else if let Some(literal) = object.as_literal() {
                    let language = literal
                        .language()
                        .map(|l| l.to_string())
                        .unwrap_or_default();
                    if !language.is_empty() {
                        element = element.attr(ATTRIBUTE_XML_LANG_PREFIXED, &language)
                    }
                    if let Some(data_type) = literal.data_type() {
                        let dt_iri = data_type.as_iri().to_string();
                        writer
                            .write(element.attr(RDF_DATATYPE.as_str(), &dt_iri))
                            .map_err(xml_error)?
                    } else {
                        writer.write(element).map_err(xml_error)?;
                    }
                    writer
                        .write(XmlEvent::Characters(literal.lexical_form()))
                        .map_err(xml_error)?;
                } else {
                    return rdf_star_not_supported_error(super::NAME).into();
                }
                writer
                    .write(XmlEvent::end_element().name(name.as_str()))
                    .map_err(xml_error)?;
            }
        }

        writer
            .write(XmlEvent::end_element().name(RDF_DESCRIPTION.as_str()))
            .map_err(xml_error)?;

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn xml_error(e: xml::writer::Error) -> Error {
    Error::Tokenizer {
        representation: super::NAME.into(),
        source: Box::new(e),
    }
}

fn split_uri(iri: &Iri) -> (String, String) {
    let iri = iri.to_string();
    let index = iri
        .chars()
        .rev()
        .enumerate()
        .find_map(|(i, c)| if c == '#' || c == '/' { Some(i) } else { None })
        .unwrap();
    assert_ne!(index, iri.len());
    let index = iri.len() - index;
    (iri[..index].to_string(), iri[index..].to_string())
}
