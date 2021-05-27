/*!
One-line description.

More detailed description, with

# Example

*/

#![allow(clippy::upper_case_acronyms)]

use pest::Parser;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "nq/nq.pest"]
struct NQuadParser;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_text(input: &str) {
    let result = NQuadParser::parse(Rule::nquadsDoc, input);
    match result {
        Ok(parsed) => println!("{:#?}", parsed),
        Err(err) => {
            println!("{}", err);
            panic!("test failed");
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_1() {
        parse_text(
            r###"
<http://one.example/subject1> <http://one.example/predicate1> <http://one.example/object1> <http://example.org/graph3> . # comments here
# or on a line by themselves
_:subject1 <http://an.example/predicate1> "object1" <http://example.org/graph1> .
_:subject2 <http://an.example/predicate2> "object2" <http://example.org/graph5> .
"###,
        )
    }

    #[test]
    fn parse_simple_2() {
        parse_text(
            r###"
_:alice <http://xmlns.com/foaf/0.1/knows> _:bob <http://example.org/graphs/john> .
_:bob <http://xmlns.com/foaf/0.1/knows> _:alice <http://example.org/graphs/james> .
"###,
        )
    }
}