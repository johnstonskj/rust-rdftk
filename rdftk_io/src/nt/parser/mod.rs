/*!
One-line description.

More detailed description, with

# Example

*/

#![allow(clippy::upper_case_acronyms)]

use crate::common::parser_error::ParserErrorFactory;
use pest::iterators::Pair;
use pest::Parser;
use rdftk_core::error::Result;
use rdftk_core::graph::{GraphFactoryRef, GraphRef};
use rdftk_core::statement::{ObjectNodeRef, StatementRef, SubjectNodeRef};
use rdftk_core::{DataType, Literal, ObjectNode, Statement, SubjectNode};
use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "nt/nt.pest"]
struct NTripleParser;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------
#[allow(dead_code)]
const ERROR: ParserErrorFactory = ParserErrorFactory { repr: super::NAME };

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_graph(input: &str, factory: GraphFactoryRef) -> Result<GraphRef> {
    let mut parsed = NTripleParser::parse(Rule::ntriplesDoc, input).map_err(|e| ERROR.parser(e))?;
    let top_node = parsed.next().unwrap();
    ntriples_doc(top_node, factory)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn ntriples_doc(input_pair: Pair<'_, Rule>, factory: GraphFactoryRef) -> Result<GraphRef> {
    trace!("ntriples_doc({:?})", &input_pair.as_rule());

    let graph = factory.new_graph();

    if input_pair.as_rule() == Rule::ntriplesDoc {
        for inner_pair in input_pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::triple => {
                    let st = triple(inner_pair)?;
                    println!("{}", &st);
                    graph.borrow_mut().insert(st);
                }
                Rule::EOI => {
                    trace!("Done.")
                }
                _ => {
                    unexpected!("ntriples_doc", inner_pair)
                }
            }
        }
    } else {
        unexpected!("ntriples_doc", input_pair);
    }

    Ok(graph)
}

fn triple(input_pair: Pair<'_, Rule>) -> Result<StatementRef> {
    trace!("triple({:?})", &input_pair.as_rule());

    if input_pair.as_rule() == Rule::triple {
        let mut inner_pairs = input_pair.into_inner();
        let subject = subject(inner_pairs.next().unwrap())?;
        let predicate = predicate(inner_pairs.next().unwrap())?;
        let object = object(inner_pairs.next().unwrap())?;
        Ok(StatementRef::new(Statement::new(
            subject, predicate, object,
        )))
    } else {
        unexpected!("triple", input_pair);
    }
}

fn subject(input_pair: Pair<'_, Rule>) -> Result<SubjectNodeRef> {
    trace!("subject({:?})", &input_pair.as_rule());

    if input_pair.as_rule() == Rule::subject {
        let inner_pair = input_pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::IRIREF => {
                let iri = inner_pair.as_str().to_string();
                let iri = &iri[1..iri.len() - 1];
                let iri = IRIRef::new(IRI::from_str(iri).unwrap());
                Ok(SubjectNodeRef::new(SubjectNode::named(iri)))
            }
            Rule::BlankNode => {
                let node = inner_pair.as_str().to_string();
                let node = &node[2..];
                Ok(SubjectNodeRef::new(SubjectNode::blank_named(node)))
            }
            _ => {
                unexpected!("subject", inner_pair)
            }
        }
    } else {
        unexpected!("subject", input_pair);
    }
}

fn predicate(input_pair: Pair<'_, Rule>) -> Result<IRIRef> {
    trace!("predicate({:?})", &input_pair.as_rule());

    if input_pair.as_rule() == Rule::predicate {
        let inner_pair = input_pair.into_inner().next().unwrap();
        if inner_pair.as_rule() == Rule::IRIREF {
            let iri = inner_pair.as_str().to_string();
            let iri = &iri[1..iri.len() - 1];
            Ok(IRIRef::new(IRI::from_str(iri).unwrap()))
        } else {
            unexpected!("subject", inner_pair);
        }
    } else {
        unexpected!("subject", input_pair);
    }
}

fn object(input_pair: Pair<'_, Rule>) -> Result<ObjectNodeRef> {
    trace!("object({:?})", &input_pair.as_rule());

    if input_pair.as_rule() == Rule::object {
        let inner_pair = input_pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::IRIREF => Ok(ObjectNodeRef::new(ObjectNode::named(iri_ref(inner_pair)?))),
            Rule::BlankNode => {
                let node = inner_pair.as_str().to_string();
                let node = &node[2..];
                Ok(ObjectNodeRef::new(ObjectNode::blank_named(node)))
            }
            Rule::literal => {
                let literal = literal(inner_pair)?;
                Ok(ObjectNodeRef::new(ObjectNode::literal(literal)))
            }
            _ => {
                unexpected!("object", inner_pair)
            }
        }
    } else {
        unexpected!("object", input_pair);
    }
}

fn literal(input_pair: Pair<'_, Rule>) -> Result<Literal> {
    trace!("literal({:?})", &input_pair.as_rule());

    if input_pair.as_rule() == Rule::literal {
        let inner_pair = input_pair.into_inner().next().unwrap();
        rdf_literal(inner_pair)
    } else {
        unexpected!("literal", input_pair);
    }
}

fn rdf_literal(input_pair: Pair<'_, Rule>) -> Result<Literal> {
    trace!("literal({:?})", &input_pair.as_rule());

    if input_pair.as_rule() == Rule::rdfLiteral {
        let mut inner_pair = input_pair.into_inner();
        let lexical_form = string(inner_pair.next().unwrap())?;

        if let Some(other) = inner_pair.next() {
            match other.as_rule() {
                Rule::iri => {
                    let data_type = DataType::Other(iri(other)?);
                    Ok(Literal::with_type(&lexical_form, data_type))
                }
                Rule::LANGTAG => {
                    let lang_tag = lang_tag(other)?;
                    Ok(Literal::with_language(&lexical_form, &lang_tag))
                }
                _ => {
                    unexpected!("literal", other);
                }
            }
        } else {
            Ok(Literal::new(&lexical_form))
        }
    } else {
        unexpected!("literal", input_pair);
    }
}

fn string(input_pair: Pair<'_, Rule>) -> Result<String> {
    trace!("string({:?})", &input_pair.as_rule());

    if input_pair.as_rule() == Rule::String {
        let inner_pair = input_pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::STRING_LITERAL_QUOTE => {
                let inner_pair = inner_pair.into_inner().next().unwrap();
                if inner_pair.as_rule() == Rule::QUOTE_INNER {
                    Ok(inner_pair.as_str().to_string())
                } else {
                    unexpected!("string", inner_pair);
                }
            }
            _ => {
                unexpected!("string", inner_pair)
            }
        }
    } else {
        unexpected!("string", input_pair);
    }
}

fn iri(input_pair: Pair<'_, Rule>) -> Result<IRIRef> {
    trace!("iri({:?})", &input_pair.as_rule());

    if input_pair.as_rule() == Rule::iri {
        let inner_pair = input_pair.into_inner().next().unwrap();
        if inner_pair.as_rule() == Rule::IRIREF {
            iri_ref(inner_pair)
        } else {
            unexpected!("iri", inner_pair);
        }
    } else {
        unexpected!("iri", input_pair);
    }
}

fn iri_ref(input_pair: Pair<'_, Rule>) -> Result<IRIRef> {
    trace!("iri_ref({:?})", &input_pair.as_rule());
    if input_pair.as_rule() == Rule::IRIREF {
        let iri = input_pair.as_str().to_string();
        let iri = &iri[1..iri.len() - 1];
        Ok(IRIRef::new(IRI::from_str(iri).unwrap()))
    } else {
        unexpected!("iri_ref", input_pair);
    }
}

fn lang_tag(input_pair: Pair<'_, Rule>) -> Result<String> {
    trace!("lang_tag({:?})", &input_pair.as_rule());
    if input_pair.as_rule() == Rule::LANGTAG {
        let tag = input_pair.as_str().to_string();
        let tag = &tag[1..];
        Ok(tag.to_string())
    } else {
        unexpected!("lang_tag", input_pair);
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nt::writer::NTripleWriter;
    use crate::GraphWriter;
    use rdftk_memgraph::simple::graph_factory;

    fn write_graph(graph: &GraphRef) {
        let writer = NTripleWriter::default();
        let _ = writer.write(&mut std::io::stdout(), graph);
    }

    #[test]
    fn parse_simple() {
        let result: Result<GraphRef> = parse_graph(
            r###"
<http://example.org/show/218> <http://www.w3.org/2000/01/rdf-schema#label> "That Seventies Show"^^<http://www.w3.org/2001/XMLSchema#string> . # literal with XML Schema string datatype
<http://example.org/show/218> <http://www.w3.org/2000/01/rdf-schema#label> "That Seventies Show" . # same as above
<http://example.org/show/218> <http://example.org/show/localName> "That Seventies Show"@en . # literal with a language tag
<http://example.org/show/218> <http://example.org/show/localName> "Cette Série des Années Septante"@fr-be .  # literal outside of ASCII range with a region subtag
<http://example.org/#spiderman> <http://example.org/text> "This is a multi-line\nliteral with many quotes (\"\"\"\"\")\nand two apostrophes ('')." .
<http://en.wikipedia.org/wiki/Helium> <http://example.org/elements/atomicNumber> "2"^^<http://www.w3.org/2001/XMLSchema#integer> . # xsd:integer
<http://en.wikipedia.org/wiki/Helium> <http://example.org/elements/specificGravity> "1.663E-4"^^<http://www.w3.org/2001/XMLSchema#double> .     # xsd:double
"###,
            graph_factory(),
        );
        match result {
            Ok(g) => {
                println!("ok");
                write_graph(&g);
            }
            Err(e) => {
                println!("{:?}", e);
                panic!();
            }
        }
    }

    #[test]
    fn parse_simple_with_blanks() {
        let result: Result<GraphRef> = parse_graph(
            r###"
<http://one.example/subject1> <http://one.example/predicate1> <http://one.example/object1> . # comments here
# or on a line by themselves
_:subject1 <http://an.example/predicate1> "object1" .
_:subject2 <http://an.example/predicate2> "object2" .
"###,
            graph_factory(),
        );
        match result {
            Ok(g) => {
                println!("ok");
                write_graph(&g);
            }
            Err(e) => {
                println!("{:?}", e);
                panic!();
            }
        }
    }
}
