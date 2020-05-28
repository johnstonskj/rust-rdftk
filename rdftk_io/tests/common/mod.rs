use rdftk_core::{Literal, ObjectNode, Statement, SubjectNode};
use rdftk_graph::{Graph, PrefixMappings};
use rdftk_iri::IRI;
use rdftk_memgraph::{Mappings, MemGraph};
use std::rc::Rc;
use std::str::FromStr;

pub fn tony_benn_graph() -> impl Graph {
    let mut mappings = Mappings::default();
    mappings.include_rdf();
    mappings.insert(
        "dc",
        IRI::from_str("http://purl.org/dc/elements/1.1/").unwrap(),
    );
    mappings.insert("foaf", IRI::from_str("http://xmlns.com/foaf/0.1/").unwrap());

    let mut statements: Vec<Rc<Statement>> = Default::default();

    statements.push(Rc::new(Statement::new(
        SubjectNode::named(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()),
        IRI::from_str("http://purl.org/dc/elements/1.1/title").unwrap(),
        Literal::new("Tony Benn").into(),
    )));
    statements.push(Rc::new(Statement::new(
        SubjectNode::named(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()),
        IRI::from_str("http://purl.org/dc/elements/1.1/publisher").unwrap(),
        Literal::new("Wikipedia").into(),
    )));
    statements.push(Rc::new(Statement::new(
        SubjectNode::named(IRI::from_str("http://en.wikipedia.org/wiki/Tony_Benn").unwrap()),
        IRI::from_str("http://purl.org/dc/elements/1.1/description").unwrap(),
        ObjectNode::blank_named("B1"),
    )));
    statements.push(Rc::new(Statement::new(
        SubjectNode::blank_named("B1"),
        IRI::from_str("http://xmlns.com/foaf/0.1/name").unwrap(),
        Literal::new("Tony Benn").into(),
    )));
    statements.push(Rc::new(Statement::new(
        SubjectNode::blank_named("B1"),
        IRI::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap(),
        IRI::from_str("http://xmlns.com/foaf/0.1/Person")
            .unwrap()
            .into(),
    )));
    MemGraph::default()
        .with(statements)
        .mappings(Rc::new(mappings))
        .to_owned()
}
