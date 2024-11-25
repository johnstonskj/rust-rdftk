use objio::{impl_has_options, ObjectWriter};
use rdftk_core::error::Error;
use rdftk_core::model::graph::Graph;
use rdftk_core::model::statement::{ObjectNode, SubjectNode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use objio::HasOptions;
use crate::GraphWriter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Used to set configurable components of the generated dot file. A default set of options may be
/// be used by using the `Default` implementation on `DotWriter`, or by passing the `Default`
/// implementation of `DotOptions` to `DotWriter::new`.
///
#[derive(Debug)]
pub struct DotOptions {
    /// The dot shape used to render a blank node. Default is `circle`.
    blank_shape: String,
    /// The color name used to render a blank node. Default is `green`.
    blank_color: String,
    /// Determines whether labels are included in blank node shapes. Default is `false`.
    blank_labels: bool,
    /// The dot shape used to render an IRI node. Default is `ellipse`.
    iri_shape: String,
    /// The color name used to render an IRI node. Default is `blue`.
    iri_color: String,
    /// The dot shape used to render a literal node. Default is `record`.
    literal_shape: String,
    /// The color name used to render a literal node. Default is `black`.
    literal_color: String,
    /// The prefix string used to generate internal node identifiers. Default is `node_`.
    node_prefix: String,
}

///
/// This struct implements the `GraphWriter` trait and will write out a serialized form for the
/// entire graph.
///
#[derive(Debug, Default)]
pub struct DotWriter {
    nodes: RefCell<HashMap<String, Node>>,
    options: DotOptions,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum NodeKind {
    Blank,
    Iri,
    Literal,
}

#[derive(Debug)]
struct Node {
    id: String,
    kind: NodeKind,
    label: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for DotOptions {
    fn default() -> Self {
        Self {
            blank_shape: "circle".to_string(),
            blank_color: "green".to_string(),
            blank_labels: false,
            iri_shape: "ellipse".to_string(),
            iri_color: "blue".to_string(),
            literal_shape: "record".to_string(),
            literal_color: "black".to_string(),
            node_prefix: "node_".to_string(),
        }
    }
}

impl DotOptions {
    pub fn with_blank_shape<S>(self, s: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            blank_shape: s.into(),
            ..self
        }
    }

    pub fn set_blank_shape<S>(&mut self, s: S)
    where
        S: Into<String>,
    {
        self.blank_shape = s.into();
    }

    pub fn blank_shape(&self) -> &String {
        &self.blank_shape
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_blank_color<S>(self, s: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            blank_color: s.into(),
            ..self
        }
    }

    pub fn set_blank_color<S>(&mut self, s: S)
    where
        S: Into<String>,
    {
        self.blank_color = s.into();
    }

    pub fn blank_color(&self) -> &String {
        &self.blank_color
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_blank_labels(self, blank_labels: bool) -> Self {
        Self {
            blank_labels,
            ..self
        }
    }

    pub fn set_blank_labels(&mut self, blank_labels: bool) {
        self.blank_labels = blank_labels;
    }

    pub fn blank_labels(&self) -> bool {
        self.blank_labels
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_iri_shape<S>(self, iri_shape: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            iri_shape: iri_shape.into(),
            ..self
        }
    }

    pub fn set_iri_shape<S>(&mut self, iri_shape: S)
    where
        S: Into<String>,
    {
        self.iri_shape = iri_shape.into();
    }

    pub fn iri_shape(&self) -> &String {
        &self.iri_shape
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_iri_color<S>(self, iri_color: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            iri_color: iri_color.into(),
            ..self
        }
    }

    pub fn set_iri_color<S>(&mut self, iri_color: S)
    where
        S: Into<String>,
    {
        self.iri_color = iri_color.into();
    }

    pub fn iri_color(&self) -> &String {
        &self.iri_color
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_literal_shape<S>(self, literal_shape: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            literal_shape: literal_shape.into(),
            ..self
        }
    }

    pub fn set_literal_shape<S>(&mut self, literal_shape: S)
    where
        S: Into<String>,
    {
        self.literal_shape = literal_shape.into();
    }

    pub fn literal_shape(&self) -> &String {
        &self.literal_shape
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_literal_color<S>(self, literal_color: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            literal_color: literal_color.into(),
            ..self
        }
    }

    pub fn set_literal_color<S>(&mut self, literal_color: S)
    where
        S: Into<String>,
    {
        self.literal_color = literal_color.into();
    }

    pub fn literal_color(&self) -> &String {
        &self.literal_color
    }

    // --------------------------------------------------------------------------------------------

    pub fn with_node_prefix<S>(self, node_prefix: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            node_prefix: node_prefix.into(),
            ..self
        }
    }

    pub fn set_node_prefix<S>(&mut self, node_prefix: S)
    where
        S: Into<String>,
    {
        self.node_prefix = node_prefix.into();
    }

    pub fn node_prefix(&self) -> &String {
        &self.node_prefix
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_options!(DotWriter, DotOptions);

impl DotWriter {
    pub fn with_options(self, options: DotOptions) -> Self {
        let mut self_mut = self;
        self_mut.set_options(options);
        self_mut
    }
}

impl ObjectWriter<Graph> for DotWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &Graph) -> Result<(), Self::Error>
    where
        W: Write,
    {
        writeln!(w, "digraph {{\n    rankdir=BT\n    charset=\"utf-8\";")?;

        // TODO: emit graph name

        writeln!(w)?;

        let mappings = graph.prefix_mappings();
        for statement in graph.statements() {
            writeln!(
                w,
                "    \"{}{}\" -> \"node_{}\" [label=\"{}\"];",
                self.options.node_prefix,
                self.subject_id(statement.subject()),
                self.object_id(statement.object()),
                match mappings.compress(statement.predicate()) {
                    None => statement.predicate().to_string(),
                    Some(qname) => qname.to_string(),
                }
            )?;
        }

        writeln!(w)?;

        for node in self.nodes.borrow().values() {
            match node.kind {
                NodeKind::Blank => {
                    if self.options.blank_labels {
                        writeln!(
                            w,
                            "    \"{}{}\" [label=\"{}{}\",shape={},color={}];",
                            self.options.node_prefix,
                            node.id,
                            self.options.node_prefix,
                            node.id,
                            self.options.blank_shape,
                            self.options.blank_color
                        )?;
                    } else {
                        writeln!(
                            w,
                            "    \"{}{}\" [label=\"\",shape={},color={}];",
                            self.options.node_prefix,
                            node.id,
                            self.options.blank_shape,
                            self.options.blank_color
                        )?;
                    }
                }
                NodeKind::Iri => {
                    writeln!(
                        w,
                        "    \"{}{}\" [URL=\"{}\",label=\"{}\",shape={},color={}];",
                        self.options.node_prefix,
                        node.id,
                        node.label,
                        node.label,
                        self.options.iri_shape,
                        self.options.iri_color
                    )?;
                }
                NodeKind::Literal => {
                    writeln!(
                        w,
                        "    \"{}{}\" [label=\"{}\",shape={},color={}];",
                        self.options.node_prefix,
                        node.id,
                        node.label,
                        self.options.literal_shape,
                        self.options.literal_color
                    )?;
                }
            }
        }
        writeln!(w, "}}")?;
        Ok(())
    }
}

impl GraphWriter for DotWriter {}

impl DotWriter {
    ///
    /// Create a new writer with the provided options, this is used to override the default
    /// options that are used when calling `Default::default`.
    ///
    pub fn new(options: DotOptions) -> Self {
        Self {
            nodes: Default::default(),
            options,
        }
    }

    fn subject_id(&self, node: &SubjectNode) -> String {
        let mut nodes = self.nodes.borrow_mut();
        if let Some(node) = nodes.get(&node.to_string()) {
            node.id.clone()
        } else {
            let id = format!("{}", nodes.len() + 1);
            if node.is_blank() {
                let _ = nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::Blank,
                        label: node.as_blank().unwrap().as_ref().into(),
                    },
                );
            } else if node.is_resource() {
                let _ = nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::Iri,
                        label: node.as_resource().unwrap().to_string(),
                    },
                );
            }
            id
        }
    }

    fn object_id(&self, node: &ObjectNode) -> String {
        let mut nodes = self.nodes.borrow_mut();
        if let Some(node) = nodes.get(&node.to_string()) {
            node.id.clone()
        } else {
            let id = format!("{}", nodes.len() + 1);
            if node.is_blank() {
                let _ = nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::Blank,
                        label: node.as_blank().unwrap().as_ref().into(),
                    },
                );
            } else if node.is_resource() {
                let _ = nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::Iri,
                        label: node.as_resource().unwrap().to_string(),
                    },
                );
            } else if node.is_literal() {
                let _ = nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::Literal,
                        label: node.as_literal().unwrap().lexical_form().clone(),
                    },
                );
            }
            id
        }
    }
}
