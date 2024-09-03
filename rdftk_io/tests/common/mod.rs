use rdftk_core::model::graph::GraphRef;
use rdftk_core::model::statement::StatementList;
use rdftk_core::simple::empty_mappings;
use rdftk_core::simple::graph::graph_factory;
use rdftk_core::simple::literal::literal_factory;
use rdftk_core::simple::statement::statement_factory;
use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;

//#[derive(Eq, PartialEq)]
//#[allow(dead_code)]
//pub enum TonyBennType {
//    NoType,
//    OneType,
//    TwoTypes,
//}

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
        if graph_type == TonyBennType::TwoTypes {
            mut_mappings.insert(
                "fibo-fnd-aap-ppl",
                IRIRef::from(
                    IRI::from_str(
                        "https://spec.edmcouncil.org/fibo/ontology/FND/AgentsAndPeople/People/",
                    )
                    .unwrap(),
                ),
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

///
/// Create a test graph for this content:
///
/// ```
/// @base <https://placeholder.kg/id/> .
/// @prefix test: <https://whatever.org/ontology/test/> .
/// @prefix use-case: <https://ekgf.org/ontology/use-case/> .
///
/// <use-case-currencies>
///     test:predicate       test:whatever;
///     use-case:usesConcept <concept-functional-currency>,
///                          <concept-currency-search-text>,
///                          <concept-currency-label>,
///                          <concept-currency-tag>,
///                          <concept-capital-raise-currency>,
///                          <concept-functional-currency-label>,
///                          <concept-share-issue-denomination-currency> .
/// ```
#[allow(dead_code)]
pub fn use_cases_graph() -> GraphRef {
    let mappings = graph_factory().mapping_factory().empty();
    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.include_rdf();
        mut_mappings.insert(
            "use-case",
            IRIRef::from(IRI::from_str("https://ekgf.org/ontology/use-case/").unwrap()),
        );
        mut_mappings.insert(
            "test",
            IRIRef::from(IRI::from_str("https://whatever.org/ontology/test/").unwrap()),
        );
    }
    let st_factory = statement_factory();

    let mut statements: StatementList = Default::default();

    let subject_iri =
        IRIRef::from(IRI::from_str("https://placeholder.kg/id/use-case-currencies").unwrap());

    for concept_iri in [
        "functional-currency",
        "currency-search-text",
        "currency-label",
        "currency-tag",
        "capital-raise-currency",
        "functional-currency-label",
        "share-issue-denomination-currency",
    ]
    .map(|c| {
        IRIRef::from(
            IRI::from_str(format!("https://placeholder.kg/id/concept-{c}").as_str()).unwrap(),
        )
    }) {
        statements.push(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri.clone()),
                    IRIRef::from(
                        IRI::from_str("https://ekgf.org/ontology/use-case/usesConcept").unwrap(),
                    ),
                    st_factory.named_object(concept_iri),
                )
                .unwrap(),
        );
    }

    statements.push(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri.clone()),
                IRIRef::from(
                    IRI::from_str("https://whatever.org/ontology/test/predicate").unwrap(),
                ),
                st_factory.named_object(IRIRef::from(
                    IRI::from_str("https://whatever.org/ontology/test/whatever").unwrap(),
                )),
            )
            .unwrap(),
    );

    graph_factory().graph_from(&statements, Some(mappings))
}

///
/// Create a test graph for this content:
///
/// ```
/// @base <https://placeholder.kg/id/> .
///
/// @prefix concept: <https://ekgf.org/ontology/concept/> .
/// @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
/// @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
/// @prefix use-case: <https://ekgf.org/ontology/use-case/> .
///
/// <use-case-currencies>
///   use-case:usesConcept [
///     a          concept:Concept ;
///     rdfs:label "Capital Raise Currency"
///   ],[
///     a          concept:Concept ;
///     rdfs:label "Currency Label"
///   ],[
///     a          concept:Concept ;
///     rdfs:label "Currency Search Text"
///   ],[
///     a          concept:Concept ;
///     rdfs:label "Currency Tag"
///   ] .
/// ```
#[allow(dead_code)]
pub fn many_blank_nodes_graph() -> GraphRef {
    let mappings = graph_factory().mapping_factory().empty();
    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.include_rdf();
        mut_mappings.include_rdfs();
        mut_mappings.insert(
            "use-case",
            IRIRef::from(IRI::from_str("https://ekgf.org/ontology/use-case/").unwrap()),
        );
        mut_mappings.insert(
            "concept",
            IRIRef::from(IRI::from_str("https://ekgf.org/ontology/concept/").unwrap()),
        );
        mut_mappings.insert(
            "graph",
            IRIRef::from(IRI::from_str("https://yourcompany.com/graph/").unwrap()),
        );
    }
    let st_factory = statement_factory();
    let li_factory = literal_factory();

    let mut statements: StatementList = Default::default();

    let subject_iri =
        IRIRef::from(IRI::from_str("https://yourcompany.com/id/use-case-currencies").unwrap());

    let concept_type_iri =
        IRIRef::from(IRI::from_str("https://ekgf.org/ontology/concept/Concept").unwrap());

    for concept_label in [
        "Capital Raise Currency",
        "Currency Label",
        "Currency Search Text",
        "Currency Tag",
    ] {
        let bn = st_factory.blank_object();
        statements.push(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri.clone()),
                    IRIRef::from(
                        IRI::from_str("https://ekgf.org/ontology/use-case/usesConcept").unwrap(),
                    ),
                    bn.clone(),
                )
                .unwrap(),
        );
        statements.push(
            st_factory
                .statement(
                    st_factory.object_as_subject(bn.clone()).unwrap(),
                    IRIRef::from(
                        IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                    ),
                    st_factory.named_object(concept_type_iri.clone()),
                )
                .unwrap(),
        );
        statements.push(
            st_factory
                .statement(
                    st_factory.object_as_subject(bn.clone()).unwrap(),
                    IRIRef::from(
                        IRI::from_str("http://www.w3.org/2000/01/rdf-schema#label").unwrap(),
                    ),
                    st_factory.literal_object(li_factory.literal(concept_label)),
                )
                .unwrap(),
        );
    }

    graph_factory().graph_from(&statements, Some(mappings))
}
