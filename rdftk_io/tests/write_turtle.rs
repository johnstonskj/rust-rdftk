#![cfg(feature = "turtle")]

use objio::{HasOptions, ObjectWriter};
use rdftk_io::turtle::{TurtleWriter, TurtleWriterOptions};
use rdftk_iri::Iri;
use std::str::FromStr;

mod common;

#[test]
fn write_to_turtle() {
    let graph = common::tony_benn_graph(Default::default());

    let writer = TurtleWriter::default();

    let result = writer.write_to_string(&graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn>"));
    assert!(output.contains("  dc:description [\n"));
    assert!(output.contains("    a         foaf:Person"));
    assert!(output.contains("    foaf:name \"Tony Benn\""));
    assert!(output.contains("  ] ;\n"));
    assert!(output.contains("  dc:publisher   \"Wikipedia\" .\n"));
}

#[test]
fn write_to_turtle_with_base() {
    let graph = common::tony_benn_graph(Default::default());

    let options = TurtleWriterOptions::default()
        .with_id_base(Iri::from_str("http://en.wikipedia.org/wiki/").unwrap());
    let writer = TurtleWriter::default().with_options(options);

    let result = writer.write_to_string(&graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    assert!(output.starts_with("@base <http://en.wikipedia.org/wiki/> .\n"));
    assert!(output.contains("@prefix dc: <http://purl.org/dc/elements/1.1/> .\n"));
    assert!(output.contains("<Tony_Benn>"));
    assert!(output.contains("  dc:description [\n"));
    assert!(output.contains("    a         foaf:Person"));
    assert!(output.contains("    foaf:name \"Tony Benn\""));
    assert!(output.contains("  ] ;\n"));
    assert!(output.contains("  dc:publisher   \"Wikipedia\" .\n"));
}

#[test]
fn write_to_turtle_with_options() {
    let graph = common::tony_benn_graph(Default::default());

    let options = TurtleWriterOptions::default()
        .with_id_base(Iri::from_str("http://en.wikipedia.org/wiki/").unwrap())
        .with_sparql_style()
        .with_nested_blank_nodes();
    let writer = TurtleWriter::default().with_options(options);

    let result = writer.write_to_string(&graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: turtle\n{}", output);

    assert!(output.starts_with("BASE <http://en.wikipedia.org/wiki/>\n"));
    assert!(output.contains("PREFIX dc: <http://purl.org/dc/elements/1.1/>\n"));
    assert!(output.contains("<Tony_Benn>"));
    assert!(output.contains("  dc:description [\n"));
    assert!(output.contains("    a         foaf:Person"));
    assert!(output.contains("    foaf:name \"Tony Benn\""));
    assert!(output.contains("  ] ;\n"));
    assert!(output.contains("  dc:publisher   \"Wikipedia\" .\n"));
}
