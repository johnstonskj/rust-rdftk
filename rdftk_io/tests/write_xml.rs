#![cfg(feature = "xml")]

use rdftk_io::write_graph_to_string;
use rdftk_io::xml::writer::{XmlOptions, XmlStyle, XmlWriter};

mod common;

#[test]
fn write_to_flat_xml() {
    let graph = common::tony_benn_graph(Default::default());

    let options = XmlOptions {
        style: XmlStyle::Flat,
        pretty: true,
        encoding: "utf-8".to_string(),
    };
    let writer = XmlWriter::new(options);

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: XML\n{}", output);
}

#[test]
fn write_to_striped_xml() {
    let graph = common::tony_benn_graph(Default::default());

    let options = XmlOptions {
        style: XmlStyle::Striped,
        pretty: true,
        encoding: "utf-8".to_string(),
    };
    let writer = XmlWriter::new(options);

    let result = write_graph_to_string(&writer, &graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: XML\n{}", output);
}
