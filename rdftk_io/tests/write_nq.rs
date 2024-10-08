#![cfg(feature = "nq")]

use objio::ObjectWriter;
use rdftk_core::model::data_set::DataSet;
use rdftk_io::nq::NQuadWriter;

mod common;

#[test]
fn write_to_nquads() {
    let graph = common::tony_benn_named_graph(common::TonyBennType::OneType);
    let data_set = DataSet::from(graph);

    let writer = NQuadWriter::default();

    let result = writer.write_to_string(&data_set);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: N-Quads\n{}", output);

    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/title> \"Tony Benn\" <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/publisher> \"Wikipedia\" <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/description> _:B1 <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
    assert!(output.contains("_:B1 <http://xmlns.com/foaf/0.1/name> \"Tony Benn\" <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
    assert!(output.contains("_:B1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
}
