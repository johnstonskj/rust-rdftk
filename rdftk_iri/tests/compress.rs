use rdftk_iri::{Iri, IriPrefixMap};
use std::str::FromStr;

#[test]
fn test_owlapi_example_section_3p7() {
    let mut map = IriPrefixMap::common();
    map.set_default_namespace(Iri::from_str("http://www.example.com/ontology1#").unwrap());

    let name = map.compress(&Iri::from_str("http://www.example.com/ontology1#Child").unwrap());
    assert!(name.is_some());
    let name = name.unwrap();

    assert_eq!(":Child".to_string(), name.to_string())
}
