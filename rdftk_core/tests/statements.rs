use rdftk_core::model::literal::DataType;
use rdftk_core::model::statement::reify_statement;
use rdftk_core::simple::literal::literal_factory;
use rdftk_core::simple::statement::statement_factory;
use rdftk_iri::IRI;
use rdftk_names::{rdf, rdfs};
use std::str::FromStr;

#[test]
fn make_a_statement() {
    let factory = statement_factory();
    let st = factory
        .statement(
            factory.blank_subject_named("01").unwrap(),
            rdf::a_type().clone(),
            factory.named_object(rdfs::class().clone()),
        )
        .unwrap();
    assert_eq!(st.to_string(), "_:01 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2000/01/rdf-schema#Class>");
}

#[test]
fn reify_a_statement() {
    let factory = statement_factory();
    let st = factory
        .statement(
            factory.blank_subject_named("01").unwrap(),
            rdf::a_type().clone(),
            factory.named_object(rdfs::class().clone()),
        )
        .unwrap();
    let sts = reify_statement(&st, &factory).unwrap();
    assert_eq!(sts.1.len(), 4);
}

#[test]
fn reify_nested_statement() {
    let factory = statement_factory();

    let st = factory
        .statement(
            factory.blank_subject_named("01").unwrap(),
            rdf::a_type().clone(),
            factory.named_object(rdfs::class().clone()),
        )
        .unwrap();

    let st = factory
        .statement(
            factory.statement_subject(st),
            rdf::a_type().clone(),
            factory.named_object(rdf::statement().clone()),
        )
        .unwrap();

    println!("{}", st);
    let sts = reify_statement(&st, &factory).unwrap();
    for st in &sts.1 {
        println!("{}", st);
    }

    assert_eq!(sts.1.len(), 8);
}

#[test]
fn make_literal_statement() {
    let factory = statement_factory();
    let literals = literal_factory();
    let st = factory
        .statement(
            factory.blank_subject_named("01").unwrap(),
            rdfs::label().clone(),
            factory.literal_object(literals.literal("some thing")),
        )
        .unwrap();
    assert_eq!(
        st.to_string(),
        "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"some thing\""
    );
}

#[test]
fn make_typed_literal_statement() {
    let factory = statement_factory();
    let literals = literal_factory();
    let st = factory
        .statement(
            factory.blank_subject_named("01").unwrap(),
            rdfs::label().clone(),
            factory.literal_object(literals.with_data_type("2020", DataType::Int)),
        )
        .unwrap();
    assert_eq!(
        st.to_string(),
        "_:01 <http://www.w3.org/2000/01/rdf-schema#label> \"2020\"^^<http://www.w3.org/2001/XMLSchema#int>"
    );
}

#[test]
fn make_an_embedded_statement() {
    let factory = statement_factory();

    //  <<...>> <http://example.org/p> <http://example.org/o>
    let about = factory
        .statement(
            factory.named_subject(IRI::from_str("http://example.org/s").unwrap().into()),
            IRI::from_str("http://example.org/p").unwrap().into(),
            factory.named_object(IRI::from_str("http://example.org/o").unwrap().into()),
        )
        .unwrap();

    let st = factory
        .statement(
            factory.blank_subject_named("a").unwrap(),
            IRI::from_str("http://example.org/v/occurenceOf")
                .unwrap()
                .into(),
            factory.statement_object(about),
        )
        .unwrap();

    assert_eq!(st.to_string(), "_:a <http://example.org/v/occurenceOf> << <http://example.org/s> <http://example.org/p> <http://example.org/o> >>");
}
