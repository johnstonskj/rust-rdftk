use rdftk_io::write_graph_to_string;
use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;

pub mod common;

#[test]
fn write_to_dot() {
    use rdftk_io::dot::DotWriter;

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
    use rdftk_io::dot::{DotOptions, DotWriter};

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

#[test]
fn write_to_ntriples() {
    use rdftk_io::nt::NTripleWriter;

    let graph = common::tony_benn_graph();

    let writer = NTripleWriter::default();

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: N-Triples\n{}", output);

    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/title> \"Tony Benn\" .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/publisher> \"Wikipedia\" .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/description> _:B1 .\n"));
    assert!(output.contains("_:B1 <http://xmlns.com/foaf/0.1/name> \"Tony Benn\" .\n"));
    assert!(output.contains("_:B1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> .\n"));
}

#[test]
fn write_to_turtle() {
    use rdftk_io::turtle::TurtleWriter;

    let graph = common::tony_benn_graph();

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
    use rdftk_io::turtle::{TurtleOptions, TurtleWriter};

    let graph = common::tony_benn_graph();

    let writer = TurtleWriter::with_base(
        IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/").unwrap()),
        TurtleOptions::default(),
    );

    let result = write_graph_to_string(&writer, &graph);
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
    use rdftk_io::turtle::{TurtleOptions, TurtleWriter};

    let graph = common::tony_benn_graph();

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

    assert!(output.starts_with("BASE <http://en.wikipedia.org/wiki/>\n"));
    assert!(output.contains("PREFIX dc: <http://purl.org/dc/elements/1.1/>\n"));
    assert!(output.contains("<Tony_Benn> dc:"));
    assert!(output.contains("dc:description _:B1"));
    assert!(output.contains("\n_:B1"));
}
