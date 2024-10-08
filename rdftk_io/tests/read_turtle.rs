#![cfg(feature = "turtle")]

use objio::ObjectReader;
use rdftk_core::{error::Error, model::graph::Graph};
use rdftk_io::turtle::TurtleReader;

#[test]
#[ignore]
fn parse_simple_turtle() {
    let reader = TurtleReader::default();
    let result: Result<Graph, Error> = reader.read_from_string(
        r###"@base <http://example.org/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix rel: <http://www.perceive.net/schemas/relationship/> .

<#green-goblin>
    rel:enemyOf <#spiderman> ;
    a foaf:Person ;    # in the context of the Marvel universe
    foaf:name "Green Goblin" .

<#spiderman>
    rel:enemyOf <#green-goblin> ;
    a foaf:Person ;
    foaf:name "Spiderman", "Человек-паук"@ru ."###,
    );
    assert!(result.is_ok());
}
