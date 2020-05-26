/*!
Provides for writing a graph in the [GraphViz](https://graphviz.gitlab.io/) dot file format.
*/

use crate::GraphWriter;
use rdftk_core::{ObjectNode, Statement, SubjectNode};
use rdftk_graph::Graph;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct DotWriter {
    nodes: RefCell<HashMap<String, Node>>,
    blank_shape: String,
    blank_color: String,
    blank_labels: bool,
    iri_shape: String,
    iri_color: String,
    literal_shape: String,
    literal_color: String,
    node_prefix: String,
}

pub const NAME: &str = "GraphViz";

pub const FILE_EXTENSION: &str = "dot";

pub const MIME_TYPE: &str = "text/vnd.graphviz";

// ------------------------------------------------------------------------------------------------
// Implementations
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

impl Default for DotWriter {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
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

impl<W: Write, G: Graph> GraphWriter<W, G> for DotWriter {
    fn write(&self, w: &mut W, graph: &G) -> std::io::Result<()> {
        self.write(w, &graph.statements())
    }
}

impl DotWriter {
    fn subject_id(&self, node: &SubjectNode) -> String {
        let mut nodes = self.nodes.borrow_mut();
        if let Some(node) = nodes.get(&node.to_string()) {
            node.id.clone()
        } else {
            let id = format!("{}", nodes.len() + 1);
            if node.is_blank() {
                nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::Blank,
                        label: node.as_blank().unwrap().clone(),
                    },
                );
            } else if node.is_uri() {
                nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::IRI,
                        label: node.as_uri().unwrap().to_string(),
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
                nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::Blank,
                        label: node.as_blank().unwrap().clone(),
                    },
                );
            } else if node.is_uri() {
                nodes.insert(
                    node.to_string(),
                    Node {
                        id: id.clone(),
                        kind: NodeKind::IRI,
                        label: node.as_uri().unwrap().to_string(),
                    },
                );
            } else if node.is_literal() {
                nodes.insert(
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
    fn write<W: Write>(&self, w: &mut W, statements: &[Rc<Statement>]) -> std::io::Result<()> {
        writeln!(w, "digraph {{    \nrankdir=BT\n    charset=\"utf-8\";")?;

        writeln!(w)?;

        for statement in statements {
            writeln!(
                w,
                "    \"{}{}\" -> \"node_{}\" [label=\"{}\"];",
                self.node_prefix,
                self.subject_id(statement.subject()),
                self.object_id(statement.object()),
                statement.predicate(),
            )?;
        }

        writeln!(w)?;

        for node in self.nodes.borrow().values() {
            match node.kind {
                NodeKind::Blank => {
                    if self.blank_labels {
                        writeln!(
                            w,
                            "    \"{}{}\" [label=\"{}{}\",shape={},color={}];",
                            self.node_prefix,
                            node.id,
                            self.node_prefix,
                            node.id,
                            self.blank_shape,
                            self.blank_color
                        )?;
                    } else {
                        writeln!(
                            w,
                            "    \"{}{}\" [label=\"\",shape={},color={}];",
                            self.node_prefix, node.id, self.blank_shape, self.blank_color
                        )?;
                    }
                }
                NodeKind::IRI => {
                    writeln!(
                        w,
                        "    \"{}{}\" [URL=\"{}\",label=\"{}\",shape={},color={}];",
                        self.node_prefix,
                        node.id,
                        node.label,
                        node.label,
                        self.iri_shape,
                        self.iri_color
                    )?;
                }
                NodeKind::Literal => {
                    writeln!(
                        w,
                        "    \"{}{}\" [label=\"{}\",shape={},color={}];",
                        self.node_prefix,
                        node.id,
                        node.label,
                        self.literal_shape,
                        self.literal_color
                    )?;
                }
            }
        }
        writeln!(w, "}}")
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rdftk_core::{Literal, Statement};
    use rdftk_iri::IRI;
    use std::str::FromStr;

    #[test]
    fn test_dot_writer() {
        let mut statements: Vec<Rc<Statement>> = Default::default();

        statements.push(Rc::new(Statement::new(
            SubjectNode::named(&IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()),
            IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap(),
            Literal::new("Tony Benn").into(),
        )));
        statements.push(Rc::new(Statement::new(
            SubjectNode::named(&IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()),
            IRI::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap(),
            Literal::new("Wikipedia").into(),
        )));
        statements.push(Rc::new(Statement::new(
            SubjectNode::named(&IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()),
            IRI::from_str("http://purl.org/dc/elements/1.1/description").unwrap(),
            ObjectNode::blank_named("B1"),
        )));
        statements.push(Rc::new(Statement::new(
            SubjectNode::blank_named("B1"),
            IRI::from_str("http://xmlns.com/foaf/0.1/name").unwrap(),
            Literal::new("Tony Benn").into(),
        )));
        statements.push(Rc::new(Statement::new(
            SubjectNode::blank_named("B1"),
            IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
            IRI::from_str("http://xmlns.com/foaf/0.1/Person")
                .unwrap()
                .into(),
        )));

        let writer = DotWriter::default();
        let mut out = std::io::stdout();
        assert!(writer.write(&mut out, &statements).is_ok());
    }
}
