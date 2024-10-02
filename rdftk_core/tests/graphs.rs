use rdftk_core::model::graph::{Graph, GraphFactory, PrefixMapping};
use rdftk_core::model::literal::LiteralFactory;
use rdftk_core::model::statement::StatementFactory;
use rdftk_core::model::Implementation;
use rdftk_core::simple::graph::SimpleGraph;
use rdftk_core::simple::literal::SimpleLiteral;
use rdftk_core::simple::statement::SimpleStatement;
use rdftk_core::simple::Implementation as SimpleImplementation;
use rdftk_iri::Iri;
use std::str::FromStr;

pub fn tony_benn_graph(
    factory: &impl Implementation<
        Graph = SimpleGraph,
        Statement = SimpleStatement,
        Literal = SimpleLiteral,
    >,
) -> SimpleGraph {
    let mappings = PrefixMapping::default()
        .with_rdf()
        .with_dcterms()
        .with_foaf();

    let mut graph = factory.graph_factory().graph();

    graph.set_prefix_mappings(mappings);

    let st_factory = factory.statement_factory();
    let lit_factory = factory.literal_factory();

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap();

    graph.insert(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri.clone()),
                Iri::from_str("http://purl.org/dc/elements/1.1/title").unwrap(),
                st_factory.literal_object(lit_factory.literal("Tony Benn")),
            )
            .unwrap(),
    );
    graph.insert(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri.clone()),
                Iri::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap(),
                st_factory.literal_object(lit_factory.literal("Wikipedia")),
            )
            .unwrap(),
    );
    graph.insert(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri),
                Iri::from_str("http://purl.org/dc/elements/1.1/description").unwrap(),
                st_factory.blank_object_named("B1").unwrap(),
            )
            .unwrap(),
    );
    graph.insert(
        st_factory
            .statement(
                st_factory.blank_subject_named("B1").unwrap(),
                Iri::from_str("http://xmlns.com/foaf/0.1/name").unwrap(),
                st_factory.literal_object(lit_factory.literal("Tony Benn")),
            )
            .unwrap(),
    );
    graph.insert(
        st_factory
            .statement(
                st_factory.blank_subject_named("B1").unwrap(),
                Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                st_factory.named_object(Iri::from_str("http://xmlns.com/foaf/0.1/Person").unwrap()),
            )
            .unwrap(),
    );
    graph
}

#[test]
fn test_simple_graph_len() {
    let implementation = SimpleImplementation::default();
    let graph = tony_benn_graph(&implementation);

    assert_eq!(graph.len(), 5);
}

#[test]
fn test_simple_graph_contains_individual() {
    let implementation = SimpleImplementation::default();
    let graph = tony_benn_graph(&implementation);

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap();
    assert!(graph.contains_subject(&subject_iri.into()));

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Harold_Wilson").unwrap();
    assert!(!graph.contains_subject(&subject_iri.into()));
}

#[test]
fn test_simple_graph_contains_subject() {
    let implementation = SimpleImplementation::default();
    let graph = tony_benn_graph(&implementation);

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap();
    let subject = implementation
        .statement_factory()
        .named_subject(subject_iri);
    assert!(graph.contains_subject(&subject));

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Harold_Wilson").unwrap();
    let subject = implementation
        .statement_factory()
        .named_subject(subject_iri);
    assert!(!graph.contains_subject(&subject));
}
