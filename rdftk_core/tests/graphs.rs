use rdftk_core::model::graph::GraphRef;
use rdftk_core::model::statement::StatementList;
use rdftk_core::simple::graph::graph_factory;
use rdftk_core::simple::literal::literal_factory;
use rdftk_core::simple::mapping::empty_mappings;
use rdftk_core::simple::statement::statement_factory;
use rdftk_core::simple::PROVIDER_ID;
use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;

pub fn tony_benn_graph() -> GraphRef {
    let mappings = empty_mappings();

    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.include_rdf();
        mut_mappings.insert(
            "dc",
            IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/").unwrap()),
        );
        mut_mappings.insert(
            "foaf",
            IRIRef::from(IRI::from_str("http://xmlns.com/foaf/0.1/").unwrap()),
        );
    }

    let st_factory = statement_factory();
    let lit_factory = literal_factory();

    let mut statements: StatementList = Default::default();

    let subject_iri =
        IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());

    statements.push(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri.clone()),
                IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap()),
                st_factory.literal_object(lit_factory.literal("Tony Benn")),
            )
            .unwrap(),
    );
    statements.push(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri.clone()),
                IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap()),
                st_factory.literal_object(lit_factory.literal("Wikipedia")),
            )
            .unwrap(),
    );
    statements.push(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri),
                IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/description").unwrap()),
                st_factory.blank_object_named("B1").unwrap(),
            )
            .unwrap(),
    );
    statements.push(
        st_factory
            .statement(
                st_factory.blank_subject_named("B1").unwrap(),
                IRIRef::from(IRI::from_str("http://xmlns.com/foaf/0.1/name").unwrap()),
                st_factory.literal_object(lit_factory.literal("Tony Benn")),
            )
            .unwrap(),
    );
    statements.push(
        st_factory
            .statement(
                st_factory.blank_subject_named("B1").unwrap(),
                IRIRef::from(
                    IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                ),
                st_factory.named_object(
                    IRI::from_str("http://xmlns.com/foaf/0.1/Person")
                        .unwrap()
                        .into(),
                ),
            )
            .unwrap(),
    );

    graph_factory().graph_from(&statements, Some(mappings))
}

#[test]
fn graph_len() {
    let graph = tony_benn_graph();
    let graph = graph.borrow();

    assert_eq!(graph.len(), 5);
}

#[test]
fn graph_provider() {
    let graph = tony_benn_graph();
    let graph = graph.borrow();

    assert_eq!(graph.factory().provider_id(), PROVIDER_ID);
}

#[test]
fn graph_contains_individual() {
    let graph = tony_benn_graph();
    let graph = graph.borrow();

    {
        let subject_iri =
            IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());

        assert!(graph.contains_individual(&subject_iri));
    }

    {
        let subject_iri =
            IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/Harold_Wilson").unwrap());

        assert!(!graph.contains_individual(&subject_iri));
    }
}

#[test]
fn graph_contains_subject() {
    let graph = tony_benn_graph();
    let graph = graph.borrow();

    {
        let subject_iri =
            IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());
        let subject = graph.statement_factory().named_subject(subject_iri);

        assert!(graph.contains_subject(&subject));
    }

    {
        let subject_iri =
            IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/Harold_Wilson").unwrap());
        let subject = graph.statement_factory().named_subject(subject_iri);

        assert!(!graph.contains_subject(&subject));
    }
}
