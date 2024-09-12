#![cfg(feature = "xml")]

use rdftk_io::xml::{XmlOptions, XmlWriter};
use objio::{HasOptions, ObjectWriter};

mod common;

#[test]
fn write_to_flat_xml() {
    let graph = common::tony_benn_graph(Default::default());

    let options = XmlOptions::default().flat().pretty().with_encoding("utf-8");
    let writer = XmlWriter::default().with_options(options);

    let result = writer.write_to_string(&graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: XML\n{}", output);
}

#[test]
fn write_to_striped_xml() {
    let graph = common::tony_benn_graph(Default::default());

    let options = XmlOptions::default()
        .striped()
        .pretty()
        .with_encoding("utf-8");
    let writer = XmlWriter::default().with_options(options);

    let result = writer.write_to_string(&graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: XML\n{}", output);
}
