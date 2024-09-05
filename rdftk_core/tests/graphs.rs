use parameterized::parameterized;
use rdftk_core::model::graph::{GraphFactoryRef, GraphRef};
use rdftk_core::simple::graph::graph_factory as simple_graph_factory;
use rdftk_core::simple::indexed::graph_factory as indexed_graph_factory;
use rdftk_core::simple::mapping::empty_mappings;
use rdftk_core::simple::PROVIDER_ID;
use rdftk_iri::{Iri, IriRef};
use std::str::FromStr;

pub fn tony_benn_graph(graph_factory: GraphFactoryRef) -> GraphRef {
    let mappings = empty_mappings();

    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.include_rdf();
        mut_mappings.insert(
            "dc",
            IriRef::from(Iri::from_str("http://purl.org/dc/elements/1.1/").unwrap()),
        );
        mut_mappings.insert(
            "foaf",
            IriRef::from(Iri::from_str("http://xmlns.com/foaf/0.1/").unwrap()),
        );
    }

    let graph = graph_factory.with_mappings(mappings);

    {
        let mut ref_graph = graph.borrow_mut();

        let st_factory = ref_graph.statement_factory();
        let lit_factory = ref_graph.literal_factory();

        let subject_iri =
            IriRef::from(Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());

        ref_graph.insert(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri.clone()),
                    IriRef::from(Iri::from_str("http://purl.org/dc/elements/1.1/title").unwrap()),
                    st_factory.literal_object(lit_factory.literal("Tony Benn")),
                )
                .unwrap(),
        );
        ref_graph.insert(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri.clone()),
                    IriRef::from(
                        Iri::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap(),
                    ),
                    st_factory.literal_object(lit_factory.literal("Wikipedia")),
                )
                .unwrap(),
        );
        ref_graph.insert(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri),
                    IriRef::from(
                        Iri::from_str("http://purl.org/dc/elements/1.1/description").unwrap(),
                    ),
                    st_factory.blank_object_named("B1").unwrap(),
                )
                .unwrap(),
        );
        ref_graph.insert(
            st_factory
                .statement(
                    st_factory.blank_subject_named("B1").unwrap(),
                    IriRef::from(Iri::from_str("http://xmlns.com/foaf/0.1/name").unwrap()),
                    st_factory.literal_object(lit_factory.literal("Tony Benn")),
                )
                .unwrap(),
        );
        ref_graph.insert(
            st_factory
                .statement(
                    st_factory.blank_subject_named("B1").unwrap(),
                    IriRef::from(
                        Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                    ),
                    st_factory.named_object(
                        Iri::from_str("http://xmlns.com/foaf/0.1/Person")
                            .unwrap()
                            .into(),
                    ),
                )
                .unwrap(),
        );
    }
    graph
}

#[parameterized(graph_factory = { simple_graph_factory(), indexed_graph_factory()})]
fn graph_len(graph_factory: GraphFactoryRef) {
    let graph = tony_benn_graph(graph_factory);
    let graph = graph.borrow();

    assert_eq!(graph.len(), 5);
}

#[parameterized(graph_factory = { simple_graph_factory(), indexed_graph_factory()})]
fn graph_provider(graph_factory: GraphFactoryRef) {
    let graph = tony_benn_graph(graph_factory);
    let graph = graph.borrow();

    assert_eq!(graph.factory().provider_id(), PROVIDER_ID);
}

#[parameterized(graph_factory = { simple_graph_factory(), indexed_graph_factory()})]
fn graph_contains_individual(graph_factory: GraphFactoryRef) {
    let graph = tony_benn_graph(graph_factory);
    let graph = graph.borrow();

    {
        let subject_iri =
            IriRef::from(Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());

        assert!(graph.contains_individual(&subject_iri));
    }

    {
        let subject_iri =
            IriRef::from(Iri::from_str("http://en.wikipedia.org/wiki/Harold_Wilson").unwrap());

        assert!(!graph.contains_individual(&subject_iri));
    }
}

#[parameterized(graph_factory = { simple_graph_factory(), indexed_graph_factory()})]
fn graph_contains_subject(graph_factory: GraphFactoryRef) {
    let graph = tony_benn_graph(graph_factory);
    let graph = graph.borrow();

    {
        let subject_iri =
            IriRef::from(Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());
        let subject = graph.statement_factory().named_subject(subject_iri);

        assert!(graph.contains_subject(&subject));
    }

    {
        let subject_iri =
            IriRef::from(Iri::from_str("http://en.wikipedia.org/wiki/Harold_Wilson").unwrap());
        let subject = graph.statement_factory().named_subject(subject_iri);

        assert!(!graph.contains_subject(&subject));
    }
}
