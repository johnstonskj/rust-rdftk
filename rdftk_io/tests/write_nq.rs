#![cfg(feature = "nq")]

use rdftk_core::data_set::MutableDataSet;
use rdftk_io::nq::writer::NQuadDataSetWriter;
use rdftk_io::write_data_set_to_string;
use rdftk_iri::IRI;
use rdftk_memgraph::data_set::MemDataSet;
use std::str::FromStr;

mod common;

#[test]
fn write_to_nquads() {
    let graph = common::tony_benn_graph();
    let mut data_set = MemDataSet::default();
    data_set.insert_graph(
        IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn")
            .unwrap()
            .into(),
        graph,
    );

    let writer = NQuadDataSetWriter::default();

    let result = write_data_set_to_string(&writer, &data_set);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: N-Quads\n{}", output);

    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/title> \"Tony Benn\" <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/publisher> \"Wikipedia\" <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
    assert!(output.contains("<http://en.wikipedia.org/wiki/Tony_Benn> <http://purl.org/dc/elements/1.1/description> _:B1 <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
    assert!(output.contains("_:B1 <http://xmlns.com/foaf/0.1/name> \"Tony Benn\" <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
    assert!(output.contains("_:B1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> <http://en.wikipedia.org/wiki/Tony_Benn> .\n"));
}