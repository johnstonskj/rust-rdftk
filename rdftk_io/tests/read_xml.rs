use objio::ObjectReader;
use rdftk_io::xml::XmlReader;

// https://www.w3.org/RDF/Validator/rdfval

pub mod logging;

#[test]
fn read_example_empty_graph() {
    let mut xml = r##"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
            xmlns:dc="http://purl.org/dc/elements/1.1/"
            xmlns:ex="http://example.org/stuff/1.0/">
</rdf:RDF>"##
        .as_bytes();

    logging::try_init();

    let reader = XmlReader::default();
    let result = reader.read(&mut xml);
    println!("{:#?}", result);
    assert!(result.is_ok());
    let graph = result.unwrap();
    println!("{:?}", graph);
    assert_eq!(graph.borrow().len(), 0);
}

#[test]
#[ignore]
fn read_example_01() {
    let mut xml = r##"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
            xmlns:dc="http://purl.org/dc/elements/1.1/"
            xmlns:ex="http://example.org/stuff/1.0/">

  <rdf:Description rdf:about="http://www.w3.org/TR/rdf-syntax-grammar"
             dc:title="RDF1.1 XML Syntax">
    <ex:kind>language grammar</ex:kind>
    <ex:comment rdf:parseType="Literal">hello <em>cruel</em>world</ex:comment>
    <ex:editor>
      <rdf:Description ex:fullName="Dave Beckett">
        <ex:homePage rdf:resource="http://purl.org/net/dajobe/" />
      </rdf:Description>
    </ex:editor>
  </rdf:Description>

</rdf:RDF>"##
        .as_bytes();

    logging::try_init();

    let reader = XmlReader::default();
    let result = reader.read(&mut xml);
    assert!(result.is_ok());
    let graph = result.unwrap();
    for st in graph.borrow().statements() {
        println!("{}", st);
    }
    assert_eq!(graph.borrow().len(), 6);
}
