use rdftk_iri::{Normalize, IRI};
use std::str::FromStr;

pub fn parse_success(iri: &str) {
    println!("> parse_success({:?})", iri);
    let result = IRI::from_str(iri);
    assert!(result.is_ok());
    let iri = result.unwrap();
    assert_eq!(iri.to_string(), iri.to_string());
}

pub fn parse_failure(iri: &str) {
    println!("> test_parse_failure({:?})", iri);
    let result = IRI::from_str(iri);
    assert!(result.is_err());
}

pub fn parse_and_compare(iri: &str, compare_to: &str) {
    println!("> parse_and_compare({:?} == {:?})", iri, compare_to);
    let result = IRI::from_str(iri);
    assert!(result.is_ok());
    let iri = result.unwrap();
    assert_eq!(iri.to_string(), compare_to.to_string());
}

pub fn normalize_and_compare(iri: &str, compare_to: &str) {
    println!("> normalize_and_compare({:?} == {:?})", iri, compare_to);
    let result = IRI::from_str(iri);
    assert!(result.is_ok());
    let iri = result.unwrap();
    let result = iri.normalize();
    assert!(result.is_ok());
    let iri = result.unwrap();
    assert_eq!(iri.to_string(), compare_to.to_string());
}
