use rdftk_core::model::graph::GraphRef;
use rdftk_core::model::statement::StatementList;
use rdftk_core::simple::graph::graph_factory;
use rdftk_core::simple::literal::literal_factory;
use rdftk_core::simple::statement::statement_factory;
use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;

#[derive(Eq, PartialEq)]
pub enum TonyBennType {
    NoType,
    OneType,
    TwoTypes,
}

pub fn tony_benn_graph(graph_type: TonyBennType) -> GraphRef {
    let mappings = graph_factory().mapping_factory().empty();
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
        if graph_type == TonyBennType::TwoTypes {
            mut_mappings.insert(
                "fibo-fnd-aap-ppl",
                IRIRef::from(IRI::from_str("https://spec.edmcouncil.org/fibo/ontology/FND/AgentsAndPeople/People/").unwrap()),
            );
        }
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
                st_factory.named_subject(subject_iri.clone()),
                IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/description").unwrap()),
                st_factory.blank_object_named("B1").unwrap(),
            )
            .unwrap(),
    );
    if graph_type == TonyBennType::OneType || graph_type == TonyBennType::TwoTypes {
        statements.push(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri.clone()),
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
    }
    if graph_type == TonyBennType::TwoTypes {
        statements.push(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri.clone()),
                    IRIRef::from(
                        IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                    ),
                    st_factory.named_object(
                        IRI::from_str("https://spec.edmcouncil.org/fibo/ontology/FND/AgentsAndPeople/People/Person")
                            .unwrap()
                            .into(),
                    ),
                )
                .unwrap(),
        );
    }
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
