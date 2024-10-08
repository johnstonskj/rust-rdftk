use rdftk_core::model::{
    graph::{Graph, GraphName, PrefixMapping},
    literal::Literal,
    statement::{BlankNode, Statement},
};
use rdftk_iri::{Iri, Name};
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
pub fn tony_benn_named_graph(graph_type: TonyBennType) -> Graph {
    let name = GraphName::from(Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());
    let (mappings, statements) = some_tony_benn_graph(graph_type);
    Graph::named(name)
        .with_mappings(mappings)
        .with_statements(statements)
}

#[allow(dead_code)]
pub fn tony_benn_graph(graph_type: TonyBennType) -> Graph {
    let (mappings, statements) = some_tony_benn_graph(graph_type);
    Graph::default()
        .with_mappings(mappings)
        .with_statements(statements)
}

fn some_tony_benn_graph(graph_type: TonyBennType) -> (PrefixMapping, Vec<Statement>) {
    let mut mappings = PrefixMapping::default()
        .with_rdf()
        .with_dc_elements()
        .with_foaf();
    if graph_type == TonyBennType::TwoTypes {
        mappings.insert(
            Name::new_unchecked("fibo-fnd-aap-ppl"),
            Iri::from_str("https://spec.edmcouncil.org/fibo/ontology/FND/AgentsAndPeople/People/")
                .unwrap(),
        );
    }

    let mut statements = Vec::default();

    let subject_iri = Iri::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap();

    statements.push(Statement::new(
        subject_iri.clone(),
        Iri::from_str("http://purl.org/dc/elements/1.1/title").unwrap(),
        Literal::plain("Tony Benn"),
    ));
    statements.push(Statement::new(
        subject_iri.clone(),
        Iri::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap(),
        Literal::plain("Wikipedia"),
    ));
    statements.push(Statement::new(
        subject_iri,
        Iri::from_str("http://purl.org/dc/elements/1.1/description").unwrap(),
        BlankNode::from_str("B1").unwrap(),
    ));
    statements.push(Statement::new(
        BlankNode::from_str("B1").unwrap(),
        Iri::from_str("http://xmlns.com/foaf/0.1/name").unwrap(),
        Literal::plain("Tony Benn"),
    ));
    statements.push(Statement::new(
        BlankNode::from_str("B1").unwrap(),
        Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
        Iri::from_str("http://xmlns.com/foaf/0.1/Person").unwrap(),
    ));
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
pub fn use_cases_graph() -> Graph {
    let mappings = PrefixMapping::default()
        .with_rdf()
        .with(
            Name::new_unchecked("use-case"),
            Iri::from_str("https://ekgf.org/ontology/use-case/").unwrap(),
        )
        .with(
            Name::new_unchecked("test"),
            Iri::from_str("https://whatever.org/ontology/test/").unwrap(),
        );

    let mut statements = Vec::default();

    let subject_iri = Iri::from_str("https://placeholder.kg/id/use-case-currencies").unwrap();

    for concept_iri in [
        "functional-currency",
        "currency-search-text",
        "currency-label",
        "currency-tag",
        "capital-raise-currency",
        "functional-currency-label",
        "share-issue-denomination-currency",
    ]
    .map(|c| Iri::from_str(format!("https://placeholder.kg/id/concept-{c}").as_str()).unwrap())
    {
        statements.push(Statement::new(
            subject_iri.clone(),
            Iri::from_str("https://ekgf.org/ontology/use-case/usesConcept").unwrap(),
            concept_iri,
        ));
    }

    statements.push(Statement::new(
        subject_iri,
        Iri::from_str("https://whatever.org/ontology/test/predicate").unwrap(),
        Iri::from_str("https://whatever.org/ontology/test/whatever").unwrap(),
    ));

    Graph::default()
        .with_mappings(mappings)
        .with_statements(statements)
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
pub fn many_blank_nodes_graph() -> Graph {
    let mappings = PrefixMapping::default()
        .with_rdf()
        .with_rdfs()
        .with(
            Name::new_unchecked("use-case"),
            Iri::from_str("https://ekgf.org/ontology/use-case/").unwrap(),
        )
        .with(
            Name::new_unchecked("concept"),
            Iri::from_str("https://ekgf.org/ontology/concept/").unwrap(),
        )
        .with(
            Name::new_unchecked("graph"),
            Iri::from_str("https://yourcompany.com/graph/").unwrap(),
        );

    let mut statements = Vec::default();

    let subject_iri = Iri::from_str("https://yourcompany.com/id/use-case-currencies").unwrap();

    let concept_type_iri = Iri::from_str("https://ekgf.org/ontology/concept/Concept").unwrap();

    for concept_label in [
        "Capital Raise Currency",
        "Currency Label",
        "Currency Search Text",
        "Currency Tag",
    ] {
        let bn = BlankNode::generate();
        statements.push(Statement::new(
            subject_iri.clone(),
            Iri::from_str("https://ekgf.org/ontology/use-case/usesConcept").unwrap(),
            bn.clone(),
        ));
        statements.push(Statement::new(
            bn.clone(),
            Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
            concept_type_iri.clone(),
        ));
        statements.push(Statement::new(
            bn,
            Iri::from_str("http://www.w3.org/2000/01/rdf-schema#label").unwrap(),
            Literal::plain(concept_label),
        ));
    }

    Graph::default()
        .with_mappings(mappings)
        .with_statements(statements)
}
