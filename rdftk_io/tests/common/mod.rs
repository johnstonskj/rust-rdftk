use rdftk_core::model::graph::{named::GraphName, GraphRef};
use rdftk_core::model::graph::{NamedGraphRef, PrefixMappingRef};
use rdftk_core::model::statement::StatementList;
use rdftk_core::simple::graph::graph_factory;
use rdftk_core::simple::literal::literal_factory;
use rdftk_core::simple::mapping::default_mappings;
use rdftk_core::simple::statement::statement_factory;
use rdftk_iri::{Iri, IriRef, Name};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TonyBennType {
    #[allow(dead_code)]
    NoType,
    #[default]
    OneType,
    TwoTypes,
}

#[allow(dead_code)]
pub fn tony_benn_named_graph(graph_type: TonyBennType) -> NamedGraphRef {
    let name = GraphName::from(Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());
    let (mappings, statements) = some_tony_benn_graph(graph_type);
    graph_factory().named_graph_from(Some(name.into()), &statements, Some(mappings))
}

#[allow(dead_code)]
pub fn tony_benn_graph(graph_type: TonyBennType) -> GraphRef {
    let (mappings, statements) = some_tony_benn_graph(graph_type);
    graph_factory().graph_from(&statements, Some(mappings))
}

fn some_tony_benn_graph(graph_type: TonyBennType) -> (PrefixMappingRef, StatementList) {
    let mappings = default_mappings();
    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.insert_rdf();
        mut_mappings.insert(
            Name::new_unchecked("dc"),
            IriRef::from(Iri::from_str("http://purl.org/dc/elements/1.1/").unwrap()),
        );
        mut_mappings.insert(
            Name::new_unchecked("foaf"),
            IriRef::from(Iri::from_str("http://xmlns.com/foaf/0.1/").unwrap()),
        );
        if graph_type == TonyBennType::TwoTypes {
            mut_mappings.insert(
                Name::new_unchecked("fibo-fnd-aap-ppl"),
                IriRef::from(
                    Iri::from_str(
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
        IriRef::from(Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());

    statements.push(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri.clone()),
                IriRef::from(Iri::from_str("http://purl.org/dc/elements/1.1/title").unwrap()),
                st_factory.literal_object(lit_factory.literal("Tony Benn")),
            )
            .unwrap(),
    );
    statements.push(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri.clone()),
                IriRef::from(Iri::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap()),
                st_factory.literal_object(lit_factory.literal("Wikipedia")),
            )
            .unwrap(),
    );
    statements.push(
        st_factory
            .statement(
                st_factory.named_subject(subject_iri),
                IriRef::from(Iri::from_str("http://purl.org/dc/elements/1.1/description").unwrap()),
                st_factory.blank_object_named("B1").unwrap(),
            )
            .unwrap(),
    );
    statements.push(
        st_factory
            .statement(
                st_factory.blank_subject_named("B1").unwrap(),
                IriRef::from(Iri::from_str("http://xmlns.com/foaf/0.1/name").unwrap()),
                st_factory.literal_object(lit_factory.literal("Tony Benn")),
            )
            .unwrap(),
    );
    statements.push(
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
    (mappings, statements)
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
    let mappings = default_mappings();
    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.insert_rdf();
        mut_mappings.insert(
            Name::new_unchecked("use-case"),
            IriRef::from(Iri::from_str("https://ekgf.org/ontology/use-case/").unwrap()),
        );
        mut_mappings.insert(
            Name::new_unchecked("test"),
            IriRef::from(Iri::from_str("https://whatever.org/ontology/test/").unwrap()),
        );
    }
    let st_factory = statement_factory();

    let mut statements: StatementList = Default::default();

    let subject_iri =
        IriRef::from(Iri::from_str("https://placeholder.kg/id/use-case-currencies").unwrap());

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
        IriRef::from(
            Iri::from_str(format!("https://placeholder.kg/id/concept-{c}").as_str()).unwrap(),
        )
    }) {
        statements.push(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri.clone()),
                    IriRef::from(
                        Iri::from_str("https://ekgf.org/ontology/use-case/usesConcept").unwrap(),
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
                IriRef::from(
                    Iri::from_str("https://whatever.org/ontology/test/predicate").unwrap(),
                ),
                st_factory.named_object(IriRef::from(
                    Iri::from_str("https://whatever.org/ontology/test/whatever").unwrap(),
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
    let mappings = default_mappings();
    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.insert_rdf();
        mut_mappings.insert_rdfs();
        mut_mappings.insert(
            Name::new_unchecked("use-case"),
            IriRef::from(Iri::from_str("https://ekgf.org/ontology/use-case/").unwrap()),
        );
        mut_mappings.insert(
            Name::new_unchecked("concept"),
            IriRef::from(Iri::from_str("https://ekgf.org/ontology/concept/").unwrap()),
        );
        mut_mappings.insert(
            Name::new_unchecked("graph"),
            IriRef::from(Iri::from_str("https://yourcompany.com/graph/").unwrap()),
        );
    }
    let st_factory = statement_factory();
    let li_factory = literal_factory();

    let mut statements: StatementList = Default::default();

    let subject_iri =
        IriRef::from(Iri::from_str("https://yourcompany.com/id/use-case-currencies").unwrap());

    let concept_type_iri =
        IriRef::from(Iri::from_str("https://ekgf.org/ontology/concept/Concept").unwrap());

    for concept_label in [
        "Capital Raise Currency",
        "Currency Label",
        "Currency Search Text",
        "Currency Tag",
    ] {
        let bn = st_factory.blank_object_new();
        statements.push(
            st_factory
                .statement(
                    st_factory.named_subject(subject_iri.clone()),
                    IriRef::from(
                        Iri::from_str("https://ekgf.org/ontology/use-case/usesConcept").unwrap(),
                    ),
                    bn.clone(),
                )
                .unwrap(),
        );
        statements.push(
            st_factory
                .statement(
                    st_factory.object_as_subject(bn.clone()).unwrap(),
                    IriRef::from(
                        Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
                    ),
                    st_factory.named_object(concept_type_iri.clone()),
                )
                .unwrap(),
        );
        statements.push(
            st_factory
                .statement(
                    st_factory.object_as_subject(bn.clone()).unwrap(),
                    IriRef::from(
                        Iri::from_str("http://www.w3.org/2000/01/rdf-schema#label").unwrap(),
                    ),
                    st_factory.literal_object(li_factory.literal(concept_label)),
                )
                .unwrap(),
        );
    }

    graph_factory().graph_from(&statements, Some(mappings))
}
