/*!
Provides the type `XmlWriter` for writing in the [RDF 1.1 XML Syntax](https://www.w3.org/TR/rdf-syntax-grammar/)
format.

This writer has a number of options, it can be written in a plain, streaming, form or alternatively
pretty-printed  for readability. It is also possible to pick one of the type styles described
in the specification, "flat" or "striped".

# Example

```rust
use rdftk_io::xml::writer::{XmlOptions, XmlWriter};
use rdftk_io::write_graph_to_string;
# use rdftk_memgraph::simple::MemGraph;
# let graph = rdftk_memgraph::simple::graph_factory().new_graph();

let options: XmlOptions = XmlOptions::flat().pretty().clone();

let writer = XmlWriter::new(options);

println!("{}", write_graph_to_string(&writer, &graph).unwrap());
```

*/

use super::syntax::{
    ATTRIBUTE_ABOUT, ATTRIBUTE_DATATYPE, ATTRIBUTE_NODE_ID, ATTRIBUTE_RESOURCE, ATTRIBUTE_XML_LANG,
    DEFAULT_ENCODING, ELEMENT_DESCRIPTION, ELEMENT_RDF,
};
use crate::error::{ErrorKind, Result};
use crate::GraphWriter;
use rdftk_core::graph::GraphRef;
use rdftk_core::statement::SubjectNodeRef;
use rdftk_core::{Graph, SubjectNode};
use rdftk_iri::IRIRef;
use rdftk_names::{dc, foaf, geo, owl, rdf, rdfs, xsd};
use std::cell::Ref;
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
    /// Nest blank nodes so that the document only has IRI subjects at the some level.
    Striped,
}

///
/// Options that control how the XML writer will render a graph.
///
#[derive(Clone, Debug)]
pub struct XmlOptions {
    /// Determines the style of the generated XML. Default is `Flat`.
    pub style: XmlStyle,
    /// Should the output be pretty-printed, including indentation. Default is `false`.
    pub pretty: bool,
    /// The encoding to specify in the XML declaration. Default is "utf-8".
    pub encoding: String,
}

///
/// A Writer to output RDF/XML.
///
#[derive(Debug)]
pub struct XmlWriter {
    mappings: HashMap<String, String>,
    options: XmlOptions,
}

lazy_static! {
    static ref RDF_ABOUT: String = format!("{}:{}", rdf::default_prefix(), ATTRIBUTE_ABOUT);
    static ref RDF_DATATYPE: String = format!("{}:{}", rdf::default_prefix(), ATTRIBUTE_DATATYPE);
    static ref RDF_DESCRIPTION: String =
        format!("{}:{}", rdf::default_prefix(), ELEMENT_DESCRIPTION);
    static ref RDF_NODE_ID: String = format!("{}:{}", rdf::default_prefix(), ATTRIBUTE_NODE_ID);
    static ref RDF_RESOURCE: String = format!("{}:{}", rdf::default_prefix(), ATTRIBUTE_RESOURCE);
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for XmlOptions {
    fn default() -> Self {
        Self {
            style: XmlStyle::Flat,
            pretty: false,
            encoding: String::from(DEFAULT_ENCODING),
        }
    }
}

impl XmlOptions {
    /// Create an option instance with `XmlStyle::Flat`.
    pub fn flat() -> Self {
        Self {
            style: XmlStyle::Flat,
            pretty: false,
            encoding: String::from(DEFAULT_ENCODING),
        }
    }

    /// Create an option instance with `XmlStyle::Striped`.
    pub fn striped() -> Self {
        Self {
            style: XmlStyle::Striped,
            pretty: false,
            encoding: String::from(DEFAULT_ENCODING),
        }
    }

    /// Set the option to emit pretty-printed XML.
    pub fn pretty(&mut self) -> &mut Self {
        self.pretty = true;
        self
    }

    /// Set the option to emit plain, non-indented, XML.
    pub fn plain(&mut self) -> &mut Self {
        self.pretty = false;
        self
    }

    /// Set the encoding string, this has no effect on the encoding being written.
    pub fn encoding(&mut self, encoding: &str) -> &mut Self {
        self.encoding = encoding.to_string();
        self
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

impl GraphWriter for XmlWriter {
    fn write(&self, w: &mut impl Write, graph: &GraphRef) -> Result<()> {
        let graph = graph.borrow();

        let config = EmitterConfig::new()
            .perform_indent(self.options.pretty)
            .normalize_empty_elements(self.options.pretty);
        let mut writer = config.create_writer(w);

        writer.write(XmlEvent::StartDocument {
            version: XmlVersion::Version11,
            encoding: Some(&self.options.encoding),
            standalone: None,
        })?;

        let container_name = format!("{}:{}", rdf::default_prefix(), ELEMENT_RDF);
        writer.write(
            XmlEvent::start_element(container_name.as_str())
                .ns(rdf::default_prefix(), rdf::namespace_str()),
        )?;

        if self.options.style == XmlStyle::Flat {
            for subject in graph.subjects() {
                self.write_subject(&mut writer, &graph, subject, true)?;
            }
        } else {
            for subject in graph.subjects().iter().filter(|s| s.is_iri()) {
                self.write_subject(&mut writer, &graph, subject, false)?;
            }
        }

        writer.write(XmlEvent::end_element().name(container_name.as_str()))?;

        Ok(())
    }
}

impl XmlWriter {
    /// Create a new writer with the specified options, over-writing the default.
    pub fn new(options: XmlOptions) -> Self {
        Self {
            mappings: Self::default_mappings(),
            options,
        }
    }

    fn default_mappings() -> HashMap<String, String> {
        let mappings: HashMap<String, String> = [
            (
                dc::elements::namespace_iri().to_string(),
                dc::elements::default_prefix().to_string(),
            ),
            (
                foaf::namespace_iri().to_string(),
                foaf::default_prefix().to_string(),
            ),
            (
                geo::namespace_iri().to_string(),
                geo::default_prefix().to_string(),
            ),
            (
                owl::namespace_iri().to_string(),
                owl::default_prefix().to_string(),
            ),
            (
                rdf::namespace_iri().to_string(),
                rdf::default_prefix().to_string(),
            ),
            (
                rdfs::namespace_iri().to_string(),
                rdfs::default_prefix().to_string(),
            ),
            (
                xsd::namespace_iri().to_string(),
                xsd::default_prefix().to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect();
        mappings
    }

    fn write_subject<'a, W: Write>(
        &self,
        writer: &mut EventWriter<W>,
        graph: &Ref<'_, dyn Graph>,
        subject: &SubjectNodeRef,
        flat: bool,
    ) -> Result<()> {
        if let Some(blank) = subject.as_blank() {
            if flat {
                writer.write(
                    XmlEvent::start_element(RDF_DESCRIPTION.as_str())
                        .attr(RDF_NODE_ID.as_str(), blank),
                )?;
            } else {
                writer.write(XmlEvent::start_element(RDF_DESCRIPTION.as_str()))?;
            }
        } else if let Some(subject) = subject.as_iri() {
            writer.write(
                XmlEvent::start_element(RDF_DESCRIPTION.as_str())
                    .attr(RDF_ABOUT.as_str(), &subject.to_string()),
            )?;
        } else {
            return Err(ErrorKind::Msg("RDF* not supported by XML writer".to_string()).into());
        }

        for predicate in graph.predicates_for(subject) {
            let (ns, mut name) = split_uri(predicate);

            for object in graph.objects_for(subject, predicate) {
                let event = if let Some(prefix) = self.mappings.get(&ns) {
                    name = format!("{}:{}", prefix, name);
                    XmlEvent::start_element(name.as_str()).ns(prefix, &ns)
                } else {
                    XmlEvent::start_element(name.as_str()).default_ns(&ns)
                };

                if let Some(iri) = object.as_iri() {
                    let iri = iri.to_string();
                    let event = event.attr(RDF_RESOURCE.as_str(), &iri);
                    writer.write(event)?;
                } else if let Some(blank) = object.as_blank() {
                    if flat {
                        let event = event.attr(RDF_NODE_ID.as_str(), blank);
                        writer.write(event)?;
                    } else {
                        writer.write(event)?;
                        self.write_subject(
                            writer,
                            graph,
                            &SubjectNode::blank_named(blank).into(),
                            flat,
                        )?;
                    }
                } else if let Some(literal) = object.as_literal() {
                    let event = if let Some(language) = literal.language() {
                        event.attr(ATTRIBUTE_XML_LANG, language)
                    } else {
                        event
                    };
                    if let Some(data_type) = literal.data_type() {
                        let dt_iri = data_type.as_iri().to_string();
                        writer.write(event.attr(RDF_DATATYPE.as_str(), &dt_iri))?
                    } else {
                        writer.write(event)?;
                    }
                    writer.write(XmlEvent::Characters(literal.lexical_form()))?;
                } else {
                    return Err(
                        ErrorKind::Msg("RDF* not supported by XML writer".to_string()).into(),
                    );
                }
                writer.write(XmlEvent::end_element().name(name.as_str()))?;
            }
        }

        writer.write(XmlEvent::end_element().name(RDF_DESCRIPTION.as_str()))?;

        Ok(())
    }
}

fn split_uri(iri: &IRIRef) -> (String, String) {
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
