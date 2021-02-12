use rdftk_core::graph::{Graph, PrefixMappings};
use rdftk_core::statement::StatementRef;
use rdftk_core::{Literal, ObjectNode, Statement, SubjectNode};
use rdftk_iri::{IRIRef, IRI};
use rdftk_memgraph::{Mappings, MemGraph};
use std::rc::Rc;
use std::str::FromStr;

pub fn tony_benn_graph() -> impl Graph {
    let mut mappings = Mappings::default();
    mappings.include_rdf();
    mappings.insert(
        "dc",
        IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/").unwrap()),
    );
    mappings.insert(
        "foaf",
        IRIRef::from(IRI::from_str("http://xmlns.com/foaf/0.1/").unwrap()),
    );

    let mut statements: Vec<StatementRef> = Default::default();

    let subject_iri =
        IRIRef::from(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap());

    statements.push(
        Statement::new(
            SubjectNode::named(subject_iri.clone()),
            IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap()),
            Literal::new("Tony Benn").into(),
        )
        .into(),
    );
    statements.push(
        Statement::new(
            SubjectNode::named(subject_iri.clone()),
            IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap()),
            Literal::new("Wikipedia").into(),
        )
        .into(),
    );
    statements.push(
        Statement::new(
            SubjectNode::named(subject_iri),
            IRIRef::from(IRI::from_str("http://purl.org/dc/elements/1.1/description").unwrap()),
            ObjectNode::blank_named("B1"),
        )
        .into(),
    );
    statements.push(
        Statement::new(
            SubjectNode::blank_named("B1"),
            IRIRef::from(IRI::from_str("http://xmlns.com/foaf/0.1/name").unwrap()),
            Literal::new("Tony Benn").into(),
        )
        .into(),
    );
    statements.push(
        Statement::new(
            SubjectNode::blank_named("B1"),
            IRIRef::from(IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap()),
            IRIRef::from(IRI::from_str("http://xmlns.com/foaf/0.1/Person").unwrap()).into(),
        )
        .into(),
    );
    MemGraph::default()
        .with(statements)
        .mappings(Rc::new(mappings))
        .to_owned()
}
