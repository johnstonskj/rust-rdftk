/*!
One-line description.

More detailed description, with

# Example

*/

#![allow(clippy::upper_case_acronyms)] // << generated by pest.

use crate::common::parser_error::ParserErrorFactory;
use pest::iterators::Pair;
use pest::Parser;
use rdftk_core::error::Error;
use rdftk_core::model::graph::GraphRef;
use rdftk_core::simple::graph::graph_factory;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "turtle/turtle.pest"]
struct TurtleParser;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[allow(dead_code)]
const ERROR: ParserErrorFactory = ParserErrorFactory { repr: super::NAME };

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[allow(dead_code)]
pub(super) fn parse_text(input: &str) -> Result<GraphRef, Error> {
    let mut parsed =
        TurtleParser::parse(Rule::turtleStarDoc, input).map_err(|e| ERROR.parser(e))?;
    let top_node = parsed.next().unwrap();
    turtle_star_doc(top_node)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[allow(dead_code)]
fn turtle_star_doc(input_pair: Pair<'_, Rule>) -> Result<GraphRef, Error> {
    let graph: GraphRef = graph_factory().graph();

    trace!("turtle_star_doc({:?})", &input_pair.as_rule());

    match input_pair.as_rule() {
        Rule::turtleStarDoc => {}
        _ => unexpected!("parse_idl", input_pair),
    }

    Ok(graph)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let result: Result<GraphRef, Error> = parse_text(
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
}
