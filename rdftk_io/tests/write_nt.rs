#![cfg(feature = "nt")]

use rdftk_io::nt::NTripleWriter;
use objio::ObjectWriter;

mod common;

#[test]
fn write_to_ntriples() {
    let graph = common::tony_benn_graph(common::TonyBennType::OneType);

    let writer = NTripleWriter::default();

    let result = writer.write_to_string(&graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: N-Triples\n{}", output);

    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/title> \"Tony Benn\" .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/publisher> \"Wikipedia\" .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/description> _:B1 .\n"));
    assert!(output.contains("_:B1 <http://xmlns.com/foaf/0.1/name> \"Tony Benn\" .\n"));
    assert!(output.contains("_:B1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> .\n"));
}
