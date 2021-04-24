#![cfg(feature = "dot")]

use rdftk_io::dot::writer::{DotOptions, DotWriter};
use rdftk_io::write_graph_to_string;

mod common;

#[test]
fn write_to_dot() {
    let graph = common::tony_benn_graph();

    let writer = DotWriter::default();

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: dot\n{}", output);
    assert!(output.starts_with("digraph {"));
    assert!(output.contains("\"node_1\" -> \"node_2\" [label=\"dc:title\"];"));
    assert!(output.contains("\"node_2\" [label=\"Tony Benn\",shape=record,color=black];"));
    assert!(output.contains("\"node_4\" [label=\"\",shape=circle,color=green];"));
    assert!(output.contains("\"node_5\" [URL=\"http://xmlns.com/foaf/0.1/Person\",label=\"http://xmlns.com/foaf/0.1/Person\",shape=ellipse,color=blue];"));
    assert!(output.ends_with("}\n"));
}

#[test]
fn write_to_dot_with_options() {
    let graph = common::tony_benn_graph();

    let mut options = DotOptions::default();
    options.literal_color = "gold".to_string();
    options.literal_shape = "square".to_string();
    options.blank_color = "red".to_string();
    options.iri_color = "black".to_string();
    let writer = DotWriter::new(options);

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: dot\n{}", output);

    assert!(output.starts_with("digraph {"));
    assert!(output.contains("\"node_1\" -> \"node_2\" [label=\"dc:title\"];"));
    assert!(output.contains("\"node_2\" [label=\"Tony Benn\",shape=square,color=gold];"));
    assert!(output.contains("\"node_4\" [label=\"\",shape=circle,color=red];"));
    assert!(output.contains("\"node_5\" [URL=\"http://xmlns.com/foaf/0.1/Person\",label=\"http://xmlns.com/foaf/0.1/Person\",shape=ellipse,color=black];"));
    assert!(output.ends_with("}\n"));
}
