#![cfg(feature = "turtle")]

use rdftk_io::turtle::writer::{TurtleOptions, TurtleWriter};
use rdftk_io::{write_graph_to_string, write_graph_to_string_with_options};
use rdftk_iri::{Iri, IriRef};
use std::str::FromStr;

mod common;

#[test]
fn write_to_turtle() {
    let graph = common::tony_benn_graph(Default::default());

    let writer = TurtleWriter::default();

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> dc:"));
    assert!(output.contains("dc:description [\n"));
    assert!(output.contains("    foaf:name \"Tony Benn\""));
    assert!(output.contains("    rdf:type foaf:Person"));
}

#[test]
fn write_to_turtle_with_base() {
    let graph = common::tony_benn_graph(Default::default());

    let options = TurtleOptions::default().with_id_base(IriRef::from(
        Iri::from_str("http://en.wikipedia.org/wiki/").unwrap(),
    ));
    let writer = TurtleWriter::default();

    let result = write_graph_to_string_with_options(&writer, &graph, &options);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    assert!(output.starts_with("@base <http://en.wikipedia.org/wiki/> .\n"));
    assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    assert!(output.contains("<Tony_Benn> dc:"));
    assert!(output.contains("dc:description [\n"));
    assert!(output.contains("    foaf:name \"Tony Benn\""));
    assert!(output.contains("    rdf:type foaf:Person"));
}

#[test]
fn write_to_turtle_with_options() {
    let graph = common::tony_benn_graph(Default::default());

    let options = TurtleOptions::default()
        .with_id_base(IriRef::from(
            Iri::from_str("http://en.wikipedia.org/wiki/").unwrap(),
        ))
        .with_sparql_style()
        .with_nested_blank_nodes();
    let writer = TurtleWriter::default();

    let result = write_graph_to_string_with_options(&writer, &graph, &options);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    assert!(output.starts_with("BASE <http://en.wikipedia.org/wiki/>\n"));
    assert!(output.contains("PREFIX dc: <http://purl.org/dc/elements/1.1/>\n"));
    assert!(output.contains("<Tony_Benn> dc:"));
    assert!(output.contains("dc:description _:B1"));
    assert!(output.contains("\n_:B1"));
}
