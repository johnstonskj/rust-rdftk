/*!
Provides the `DotWriter` implementation of the `GraphWriter` trait. This writer also as certain
options that govern the output generated, these are set using the `DotOptions` structure which
can be passed to `DotWriter::new`.

```rust
use rdftk_io::dot::writer::{DotOptions, DotWriter};
use rdftk_io::write_graph_to_string;
# use rdftk_core::graph::GraphRef;
# fn make_graph() -> GraphRef { rdftk_memgraph::simple::graph_factory().new_graph() }

let mut options = DotOptions::default();
options.blank_labels = true;
let writer = DotWriter::new(options);

let result = write_graph_to_string(&writer, &make_graph());
```
*/

use crate::GraphWriter;
use rdftk_core::graph::GraphRef;
use rdftk_core::{ObjectNode, SubjectNode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;

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
    pub blank_shape: String,
    /// The color name used to render a blank node. Default is `green`.
    pub blank_color: String,
    /// Determines whether labels are included in blank node shapes. Default is `false`.
    pub blank_labels: bool,
    /// The dot shape used to render an IRI node. Default is `ellipse`.
    pub iri_shape: String,
    /// The color name used to render an IRI node. Default is `blue`.
    pub iri_color: String,
    /// The dot shape used to render a literal node. Default is `record`.
    pub literal_shape: String,
    /// The color name used to render a literal node. Default is `black`.
    pub literal_color: String,
    /// The prefix string used to generate internal node identifiers. Default is `node_`.
    pub node_prefix: String,
}

///
/// This struct implements the `GraphWriter` trait and will write out a serialized form for the
/// entire graph.
///
#[derive(Debug)]
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
    IRI,
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

// ------------------------------------------------------------------------------------------------

impl Default for DotWriter {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            options: Default::default(),
        }
    }
}

impl GraphWriter for DotWriter {
    fn write(&self, w: &mut impl Write, graph: &GraphRef) -> crate::error::Result<()> {
        writeln!(w, "digraph {{\n    rankdir=BT\n    charset=\"utf-8\";")?;

        writeln!(w)?;

        let graph = graph.borrow();

        let mappings = graph.prefix_mappings();
        for statement in graph.statements() {
            writeln!(
                w,
                "    \"{}{}\" -> \"node_{}\" [label=\"{}\"];",
                self.options.node_prefix,
                self.subject_id(statement.subject()),
                self.object_id(statement.object()),
                match mappings.borrow().compress(&statement.predicate()) {
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
                NodeKind::IRI => {
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
                        label: node.as_blank().unwrap().clone(),
                    },
                );
            } else if node.is_iri() {
                let _ = nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::IRI,
                        label: node.as_iri().unwrap().to_string(),
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
                        label: node.as_blank().unwrap().clone(),
                    },
                );
            } else if node.is_iri() {
                let _ = nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::IRI,
                        label: node.as_iri().unwrap().to_string(),
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
