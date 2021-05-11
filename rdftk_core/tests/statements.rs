use rdftk_core::literal::DataType;
use rdftk_core::statement::{Statement, StatementRef, SubjectNode};
use rdftk_core::ObjectNode;
use rdftk_iri::IRI;
use rdftk_names::{rdf, rdfs};
use std::str::FromStr;

#[test]
fn make_a_statement() {
    let st = Statement::new(
        SubjectNode::blank_named("01").into(),
        rdf::a_type().clone(),
        ObjectNode::named(rdfs::class().clone()).into(),
    );
    assert_eq!(st.to_string(), "_:01 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2000/01/rdf-schema#Class> .");
}

#[test]
fn reify_a_statement() {
    let st = Statement::new(
        SubjectNode::blank_named("01").into(),
        rdf::a_type().clone(),
        ObjectNode::named(rdfs::class().clone()).into(),
    );
    let sts = st.reify();
    assert_eq!(sts.len(), 4);
}

#[test]
fn reify_nested_statement() {
    let st = StatementRef::from(Statement::new(
        SubjectNode::blank_named("01").into(),
        rdf::a_type().clone(),
        ObjectNode::named(rdfs::class().clone()).into(),
    ));
    let st = Statement::new(
        SubjectNode::about(st).into(),
        rdf::a_type().clone(),
        ObjectNode::named(rdf::statement().clone()).into(),
    );
    println!("{}", st);
    let sts = st.reify();
    for st in &sts {
        println!("{}", st);
    }
    assert_eq!(sts.len(), 8);
}

#[test]
fn make_literal_statement() {
    let st = Statement::new(
        SubjectNode::blank_named("01").into(),
        rdfs::label().clone(),
        ObjectNode::literal_str("some thing").into(),
    );
    assert_eq!(
        st.to_string(),
        "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"some thing\" ."
    );
}

#[test]
fn make_typed_literal_statement() {
    let st = Statement::new(
        SubjectNode::blank_named("01").into(),
        rdfs::label().clone(),
        ObjectNode::literal_with_type("2020", DataType::Int).into(),
    );
    assert_eq!(
        st.to_string(),
        "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"2020\"^^<http://www.w3.org/2001/XMLSchema#int> ."
    );
}

#[test]
fn make_an_embedded_statement() {
    //  <> <http://example.org/p> <http://example.org/o>
    let about = StatementRef::from(Statement::new(
        SubjectNode::from(IRI::from_str("http://example.org/s").unwrap()).into(),
        IRI::from_str("http://example.org/p").unwrap().into(),
        ObjectNode::from(IRI::from_str("http://example.org/o").unwrap()).into(),
    ));
    let st = Statement::new(
        SubjectNode::blank_named("a").into(),
        IRI::from_str("http://example.org/v/occurenceOf")
            .unwrap()
            .into(),
        ObjectNode::about(about).into(),
    );
    assert_eq!(st.to_string(), "_:a <http://example.org/v/occurenceOf> << <http://example.org/s> <http://example.org/p> <http://example.org/o> >> .");
}
