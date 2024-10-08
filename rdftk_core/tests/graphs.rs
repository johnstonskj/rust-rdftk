use rdftk_core::model::{
    graph::{Graph, PrefixMapping},
    literal::Literal,
    statement::{BlankNode, Statement, SubjectNode},
};
use rdftk_iri::Iri;
use std::str::FromStr;

pub fn tony_benn_graph() -> Graph {
    let mappings = PrefixMapping::default()
        .with_rdf()
        .with_dc_terms()
        .with_foaf();

    let mut graph = Graph::default();

    graph.set_prefix_mappings(mappings);

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap();

    graph.insert(Statement::new(
        &subject_iri,
        Iri::from_str("http://purl.org/dc/elements/1.1/title").unwrap(),
        Literal::plain("Tony Benn"),
    ));
    graph.insert(Statement::new(
        &subject_iri,
        Iri::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap(),
        Literal::plain("Wikipedia"),
    ));
    graph.insert(Statement::new(
        &subject_iri,
        Iri::from_str("http://purl.org/dc/elements/1.1/description").unwrap(),
        BlankNode::from_str("B1").unwrap(),
    ));
    graph.insert(Statement::new(
        BlankNode::from_str("B1").unwrap(),
        Iri::from_str("http://xmlns.com/foaf/0.1/name").unwrap(),
        Literal::plain("Tony Benn"),
    ));
    graph.insert(Statement::new(
        BlankNode::from_str("B1").unwrap(),
        Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
        Iri::from_str("http://xmlns.com/foaf/0.1/Person").unwrap(),
    ));
    graph
}

#[test]
fn test_simple_graph_len() {
    let graph = tony_benn_graph();

    assert_eq!(graph.len(), 5);
}

#[test]
fn test_simple_graph_contains_individual() {
    let graph = tony_benn_graph();

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap();
    assert!(graph.contains_subject(&subject_iri.into()));

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Harold_Wilson").unwrap();
    assert!(!graph.contains_subject(&subject_iri.into()));
}

#[test]
fn test_simple_graph_contains_subject() {
    let graph = tony_benn_graph();

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap();
    let subject = SubjectNode::from(subject_iri);
    assert!(graph.contains_subject(&subject));

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Harold_Wilson").unwrap();
    let subject = SubjectNode::from(subject_iri);
    assert!(!graph.contains_subject(&subject));
}
