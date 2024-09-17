#![cfg(feature = "json")]

use objio::ObjectReader;
use rdftk_io::json::JsonReader;

#[test]
fn read_example_01() {
    let mut json = r##"{
  "http://example.org/about" : {
      "http://purl.org/dc/terms/title" : [ { "value" : "Anna's Homepage", 
                                             "type" : "literal", 
                                             "lang" : "en" } ] 
  }
}"##
    .as_bytes();

    let reader = JsonReader::default();
    let result = reader.read(&mut json);
    assert!(result.is_ok());
    let graph = result.unwrap();
    println!("{:?}", graph);
    assert_eq!(graph.borrow().len(), 1);
}

#[test]
fn read_example_02() {
    let mut json = r##"{
  "http://example.org/about" : {
      "http://purl.org/dc/terms/title" : [ { "value" : "Anna's Homepage", 
                                             "type" : "literal", 
                                             "lang" : "en" },
                                           { "value" : "Annas hjemmeside", 
                                             "type" : "literal", 
                                             "lang" : "da" } ] 
  }
}"##
    .as_bytes();

    let reader = JsonReader::default();
    let result = reader.read(&mut json);
    assert!(result.is_ok());
    let graph = result.unwrap();
    println!("{:?}", graph);
    assert_eq!(graph.borrow().len(), 2);
}

#[test]
fn read_example_03() {
    let mut json = r##"{
  "_:anna" : {
      "http://xmlns.com/foaf/0.1/name" : [ { "value" : "Anna", 
                                             "type" : "literal" } ],
      "http://xmlns.com/foaf/0.1/homepage" : [ { "value" : "http://example.org/anna", 
                                                 "type" : "uri" } ] 
  }
}                                                        "##
        .as_bytes();

    let reader = JsonReader::default();
    let result = reader.read(&mut json);
    assert!(result.is_ok());
    let graph = result.unwrap();
    println!("{:?}", graph);
    assert_eq!(graph.borrow().len(), 2);
}
