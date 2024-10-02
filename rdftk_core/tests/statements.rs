use rdftk_core::model::literal::{DataType, LiteralFactory};
use rdftk_core::model::statement::{Statement, StatementFactory};
use rdftk_core::simple::literal::SimpleLiteralFactory;
use rdftk_core::simple::statement::SimpleStatementFactory;
use rdftk_iri::Iri;
use rdftk_names::{rdf, rdfs};
use std::str::FromStr;

#[test]
fn make_a_statement() {
    let factory = SimpleStatementFactory::default();
    let st = factory
        .statement(
            factory.blank_subject_named("B01").unwrap(),
            rdf::a_type().clone(),
            factory.named_object(rdfs::class().clone()),
        )
        .unwrap();
    assert_eq!(st.to_string(), "_:B01 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2000/01/rdf-schema#Class>");
}

#[test]
fn reify_a_statement() {
    let factory = SimpleStatementFactory::default();
    let st = factory
        .statement(
            factory.blank_subject_named("B01").unwrap(),
            rdf::a_type().clone(),
            factory.named_object(rdfs::class().clone()),
        )
        .unwrap();
    let sts = st.reify(&factory).unwrap();
    assert_eq!(sts.1.len(), 4);
}

#[test]
fn reify_nested_statement() {
    let factory = SimpleStatementFactory::default();

    let st = factory
        .statement(
            factory.blank_subject_named("B01").unwrap(),
            rdf::a_type().clone(),
            factory.named_object(rdfs::class().clone()),
        )
        .unwrap();

    let st = factory
        .statement(
            factory.statement_subject(st.into()),
            rdf::a_type().clone(),
            factory.named_object(rdf::statement().clone()),
        )
        .unwrap();

    println!("{}", st);
    let sts = st.reify(&factory).unwrap();
    for st in &sts.1 {
        println!("{}", st);
    }

    assert_eq!(sts.1.len(), 8);
}

#[test]
fn make_literal_statement() {
    let factory = SimpleStatementFactory::default();
    let literals = SimpleLiteralFactory::default();
    let st = factory
        .statement(
            factory.blank_subject_named("B01").unwrap(),
            rdfs::label().clone(),
            factory.literal_object(literals.literal("some thing")),
        )
        .unwrap();
    assert_eq!(
        st.to_string(),
        "_:B01 <http://www.w3.org/2000/01/rdf-schema#label> \"some thing\""
    );
}

#[test]
fn make_typed_literal_statement() {
    let factory = SimpleStatementFactory::default();
    let literals = SimpleLiteralFactory::default();
    let st = factory
        .statement(
            factory.blank_subject_named("B01").unwrap(),
            rdfs::label().clone(),
            factory.literal_object(literals.with_data_type("2020", DataType::Int)),
        )
        .unwrap();
    assert_eq!(
        st.to_string(),
        "_:B01 <http://www.w3.org/2000/01/rdf-schema#label> \"2020\"^^<http://www.w3.org/2001/XMLSchema#int>"
    );
}

#[test]
fn make_an_embedded_statement() {
    let factory = SimpleStatementFactory::default();

    //  <<...>> <http://example.org/p> <http://example.org/o>
    let about = factory
        .statement(
            factory.named_subject(Iri::from_str("http://example.org/s").unwrap()),
            Iri::from_str("http://example.org/p").unwrap(),
            factory.named_object(Iri::from_str("http://example.org/o").unwrap()),
        )
        .unwrap();

    let st = factory
        .statement(
            factory.blank_subject_named("a").unwrap(),
            Iri::from_str("http://example.org/v/occurenceOf").unwrap(),
            factory.statement_object(about.into()),
        )
        .unwrap();

    assert_eq!(st.to_string(), "_:a <http://example.org/v/occurenceOf> << <http://example.org/s> <http://example.org/p> <http://example.org/o> >>");
}
