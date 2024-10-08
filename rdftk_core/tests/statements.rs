use rdftk_core::model::literal::Literal;
use rdftk_core::model::statement::Statement;
use rdftk_core::model::{literal::DataType, statement::BlankNode};
use rdftk_iri::Iri;
use rdftk_names::{rdf, rdfs};
use std::str::FromStr;

#[test]
fn make_a_statement() {
    let st = Statement::new(
        BlankNode::from_str("B01").unwrap(),
        rdf::a_type().clone(),
        rdfs::class(),
    );
    assert_eq!(st.to_string(), "_:B01 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2000/01/rdf-schema#Class>");
}

#[test]
fn reify_a_statement() {
    let st = Statement::new(
        BlankNode::from_str("B01").unwrap(),
        rdf::a_type().clone(),
        rdfs::class(),
    );
    let (_, sts) = st.reify().unwrap();
    assert_eq!(sts.len(), 4);
}

#[test]
fn reify_nested_statement() {
    let st = Statement::new(
        BlankNode::from_str("B01").unwrap(),
        rdf::a_type().clone(),
        rdfs::class(),
    );

    let st = Statement::new(st, rdf::a_type().clone(), rdf::statement());

    println!("{}", st);
    let (_, sts) = st.reify().unwrap();
    for st in &sts {
        println!("{}", st);
    }

    assert_eq!(sts.len(), 8);
}

#[test]
fn make_literal_statement() {
    let st = Statement::new(
        BlankNode::from_str("B01").unwrap(),
        rdfs::label().clone(),
        Literal::plain("some thing"),
    );
    assert_eq!(
        st.to_string(),
        "_:B01 <http://www.w3.org/2000/01/rdf-schema#label> \"some thing\""
    );
}

#[test]
fn make_typed_literal_statement() {
    let st = Statement::new(
        BlankNode::from_str("B01").unwrap(),
        rdfs::label().clone(),
        Literal::with_data_type("2020", DataType::Int),
    );
    assert_eq!(
        st.to_string(),
        "_:B01 <http://www.w3.org/2000/01/rdf-schema#label> \"2020\"^^<http://www.w3.org/2001/XMLSchema#int>"
    );
}

#[test]
fn make_an_embedded_statement() {
    //  <<...>> <http://example.org/p> <http://example.org/o>
    let about = Statement::new(
        Iri::from_str("http://example.org/s").unwrap(),
        Iri::from_str("http://example.org/p").unwrap(),
        Iri::from_str("http://example.org/o").unwrap(),
    );

    let st = Statement::new(
        BlankNode::from_str("a").unwrap(),
        Iri::from_str("http://example.org/v/occurenceOf").unwrap(),
        about,
    );

    assert_eq!(st.to_string(), "_:a <http://example.org/v/occurenceOf> << <http://example.org/s> <http://example.org/p> <http://example.org/o> >>");
}
