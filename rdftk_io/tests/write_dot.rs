#![cfg(feature = "dot")]

use objio::{HasOptions, ObjectWriter};
use rdftk_io::dot::{DotOptions, DotWriter};

mod common;

#[test]
fn write_to_dot() {
    let graph = common::tony_benn_graph(common::TonyBennType::OneType);

    let writer = DotWriter::default();

    let result = writer.write_to_string(&graph);
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
    let graph = common::tony_benn_graph(common::TonyBennType::OneType);

    let options = DotOptions::default()
        .with_blank_color("red")
        .with_literal_color("gold")
        .with_literal_shape("square")
        .with_iri_color("black");

    let mut writer = DotWriter::default();
    writer.set_options(options);

    let result = writer.write_to_string(&graph);
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
