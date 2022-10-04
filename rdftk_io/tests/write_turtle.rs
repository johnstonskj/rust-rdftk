#![cfg(feature = "turtle")]

use rdftk_io::write_graph_to_string;
use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;
use rdftk_io::turtle::writer::TurtleOptions;
use rdftk_io::turtle::writer::TurtleWriter;
use crate::common::TonyBennType;

mod common;

#[test]
fn write_to_turtle_no_types() {
    let graph = common::tony_benn_graph(TonyBennType::NoType);

    let writer = TurtleWriter::with_indent_with(4);

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    // assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    // assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> dc:"));
    // assert!(output.contains("dc:description [\n"));
    // assert!(output.contains("    foaf:name \"Tony Benn\""));
    // assert!(output.contains("    rdf:type foaf:Person"));
}

#[test]
fn write_to_turtle_one_type() {
    let graph = common::tony_benn_graph(TonyBennType::OneType);

    let writer = TurtleWriter::with_indent_with(4);

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle, no base IRI with type on new line\n{}", output);

    // assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    // assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> dc:"));
    // assert!(output.contains("dc:description [\n"));
    // assert!(output.contains("    foaf:name \"Tony Benn\""));
    // assert!(output.contains("    rdf:type foaf:Person"));
}

#[test]
fn write_to_turtle_two_types() {
    let graph = common::tony_benn_graph(TonyBennType::TwoTypes);

    let options = TurtleOptions::new_with_intellij_style_with_type_on_subject_line();
    let writer = TurtleWriter::new(options);

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle, no base IRI, intellij style with type on subject line\n{}", output);

    // assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    // assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> dc:"));
    // assert!(output.contains("dc:description [\n"));
    // assert!(output.contains("    foaf:name \"Tony Benn\""));
    // assert!(output.contains("    rdf:type foaf:Person"));
}

#[test]
fn write_to_turtle_two_types_with_type_on_new_line() {
    let graph = common::tony_benn_graph(TonyBennType::TwoTypes);

    let writer = TurtleWriter::with_indent_with(4);

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    // assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    // assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> dc:"));
    // assert!(output.contains("dc:description [\n"));
    // assert!(output.contains("    foaf:name \"Tony Benn\""));
    // assert!(output.contains("    rdf:type foaf:Person"));
}

#[test]
fn write_to_turtle_with_base() {
    let graph = common::tony_benn_graph(TonyBennType::TwoTypes);

    let writer = TurtleWriter::with_base(
        IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/").unwrap()),
        TurtleOptions::default().with_indent_width(4)
    );

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    // assert!(output.starts_with("@base <http://en.wikipedia.org/wiki/> .\n"));
    // assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    // assert!(output.contains("<Tony_Benn> dc:"));
    // assert!(output.contains("dc:description [\n"));
    // assert!(output.contains("    foaf:name \"Tony Benn\""));
    // assert!(output.contains("    rdf:type foaf:Person"));
}

#[test]
fn write_to_turtle_with_options() {
    let graph = common::tony_benn_graph(TonyBennType::TwoTypes);

    let mut options = TurtleOptions::default();
    options.use_sparql_style = true;
    options.nest_blank_nodes = false;
    let writer = TurtleWriter::with_base(
        IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/").unwrap()),
        options,
    );

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    // assert!(output.starts_with("BASE <http://en.wikipedia.org/wiki/>\n"));
    // assert!(output.contains("PREFIX dc: <http://purl.org/dc/elements/1.1/>\n"));
    // assert!(output.contains("<Tony_Benn> dc:"));
    // assert!(output.contains("dc:description _:B1"));
    // assert!(output.contains("\n_:B1"));
}
