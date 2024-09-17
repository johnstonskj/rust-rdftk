use super::Rule;
use pest::iterators::Pair;
use rdftk_core::error::Error;
use rdftk_core::model::graph::{GraphFactoryRef, GraphRef};
use rdftk_core::model::literal::{DataType, LanguageTag, LiteralFactoryRef, LiteralRef};
use rdftk_core::model::statement::{
    ObjectNodeRef, StatementFactoryRef, StatementRef, SubjectNodeRef,
};
use rdftk_iri::{Iri, IriRef};
use regex::Regex;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_doc(
    input_pair: Pair<'_, Rule>,
    factory: GraphFactoryRef,
) -> Result<GraphRef, Error> {
    parse_rule!("parse_doc" entry input_pair);

    let graph = factory.graph();

    if input_pair.as_rule() == Rule::ntripleDoc {
        for inner_pair in input_pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ntriple => {
                    let mut graph = graph.borrow_mut();
                    let st = triple(
                        inner_pair,
                        &graph.statement_factory(),
                        &graph.literal_factory(),
                    )?;
                    graph.insert(st);
                }
                Rule::EOI => {
                    return Ok(graph);
                }
                _ => {
                    return Err(pest_error!(
                        unexpected
                        RULE_FN,
                        &inner_pair,
                        [Rule::ntriple, Rule::EOI]
                    ));
                }
            }
        }
    } else {
        return Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::ntripleDoc]));
    }

    unreachable!()
}

fn triple(
    input_pair: Pair<'_, Rule>,
    statements: &StatementFactoryRef,
    literals: &LiteralFactoryRef,
) -> Result<StatementRef, Error> {
    parse_rule!("triple" entry input_pair);

    if input_pair.as_rule() == Rule::ntriple {
        let mut inner_pairs = input_pair.into_inner();
        let subject = subject(inner_pairs.next().unwrap(), statements)?;
        let predicate = predicate(inner_pairs.next().unwrap())?;
        let object = object(inner_pairs.next().unwrap(), statements, literals)?;
        statements.statement(subject, predicate, object)
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::ntriple]))
    }
}

pub(crate) fn subject(
    input_pair: Pair<'_, Rule>,
    factory: &StatementFactoryRef,
) -> Result<SubjectNodeRef, Error> {
    parse_rule!("nt_subject" entry input_pair);

    if input_pair.as_rule() == Rule::ntripleSubject {
        let inner_pair = input_pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::IRIREF => Ok(factory.named_subject(iri_ref(inner_pair)?)),
            Rule::blankNode => {
                let node = inner_pair.as_str().to_string();
                // strip the leading '_:'
                let node = &node[2..];
                factory.blank_subject_named(node)
            }
            _ => Err(pest_error!(
                unexpected
                RULE_FN,
                &inner_pair,
                [Rule::IRIREF, Rule::blankNode]
            )),
        }
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::ntripleSubject]))
    }
}

pub(crate) fn predicate(input_pair: Pair<'_, Rule>) -> Result<IriRef, Error> {
    parse_rule!("predicate" entry input_pair);

    if input_pair.as_rule() == Rule::ntriplePredicate {
        let inner_pair = input_pair.into_inner().next().unwrap();
        if inner_pair.as_rule() == Rule::IRIREF {
            Ok(iri_ref(inner_pair)?)
        } else {
            Err(pest_error!(unexpected RULE_FN, &inner_pair, [Rule::IRIREF]))
        }
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::ntriplePredicate]))
    }
}

pub(crate) fn object(
    input_pair: Pair<'_, Rule>,
    factory: &StatementFactoryRef,
    literals: &LiteralFactoryRef,
) -> Result<ObjectNodeRef, Error> {
    parse_rule!("object" entry input_pair);

    if input_pair.as_rule() == Rule::ntripleObject {
        let inner_pair = input_pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::IRIREF => Ok(factory.named_object(iri_ref(inner_pair)?)),
            Rule::blankNode => {
                let node = inner_pair.as_str().to_string();
                // strip the leading '_:'
                let node = &node[2..];
                Ok(factory.blank_object_named(node)?)
            }
            Rule::ntripleLiteral => {
                let literal = literal(inner_pair, literals)?;
                Ok(factory.literal_object(literal))
            }
            _ => Err(pest_error!(
                 unexpected
                RULE_FN,
                &inner_pair,
                [Rule::IRIREF, Rule::blankNode, Rule::ntripleLiteral]
            )),
        }
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::ntripleObject]))
    }
}

fn literal(input_pair: Pair<'_, Rule>, literals: &LiteralFactoryRef) -> Result<LiteralRef, Error> {
    parse_rule!("literal" entry input_pair);

    if input_pair.as_rule() == Rule::ntripleLiteral {
        let inner_pair = input_pair.into_inner().next().unwrap();
        rdf_literal(inner_pair, literals)
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::ntripleObject]))
    }
}

fn rdf_literal(
    input_pair: Pair<'_, Rule>,
    literals: &LiteralFactoryRef,
) -> Result<LiteralRef, Error> {
    parse_rule!("rdf_literal" entry input_pair);

    if input_pair.as_rule() == Rule::ntripleRdfLiteral {
        let mut inner_pair = input_pair.into_inner();
        let lexical_form = string(inner_pair.next().unwrap())?;

        if let Some(other) = inner_pair.next() {
            match other.as_rule() {
                Rule::IRIREF => {
                    let data_type = DataType::Other(iri_ref(other)?);
                    Ok(literals.with_data_type(&lexical_form, data_type))
                }
                Rule::LANGTAG => {
                    let lang_tag = lang_tag(other)?;
                    Ok(literals.with_language(&lexical_form, lang_tag))
                }
                _ => Err(pest_error!(
                    unexpected
                    RULE_FN,
                    &other,
                   [Rule::IRIREF, Rule::LANGTAG]
                )),
            }
        } else {
            Ok(literals.literal(&lexical_form))
        }
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::ntripleRdfLiteral]))
    }
}

fn string(input_pair: Pair<'_, Rule>) -> Result<String, Error> {
    parse_rule!("string" entry input_pair);

    if input_pair.as_rule() == Rule::ntripleString {
        let inner_pair = input_pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::STRING_LITERAL_QUOTE => {
                let inner_pair = inner_pair.into_inner().next().unwrap();
                if inner_pair.as_rule() == Rule::QUOTE_INNER {
                    Ok(inner_pair.as_str().to_string())
                } else {
                    Err(pest_error!(
                        unexpected
                        RULE_FN,
                        &inner_pair,
                        [Rule::QUOTE_INNER]
                    ))
                }
            }
            _ => Err(pest_error!(
                unexpected
                RULE_FN,
                &inner_pair,
                [Rule::STRING_LITERAL_QUOTE]
            )),
        }
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::ntripleString]))
    }
}

fn iri_ref(input_pair: Pair<'_, Rule>) -> Result<IriRef, Error> {
    parse_rule!("iri_ref" entry input_pair);

    if input_pair.as_rule() == Rule::IRIREF {
        let iri = input_pair.as_str().to_string();
        // strip the '<' and '>' characters.
        let iri_str = unescape_iri(&iri[1..iri.len() - 1]);
        Ok(IriRef::new(Iri::from_str(&iri_str)?))
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::IRIREF]))
    }
}

fn lang_tag(input_pair: Pair<'_, Rule>) -> Result<LanguageTag, Error> {
    parse_rule!("lang_tag" entry input_pair);

    if input_pair.as_rule() == Rule::LANGTAG {
        let tag = input_pair.as_str().to_string();
        println!("**{tag}**");
        // strip the leading '@'
        let tag = &tag[1..];
        println!("**{tag}**");
        Ok(LanguageTag::from_str(tag)?)
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::LANGTAG]))
    }
}

// ------------------------------------------------------------------------------------------------

lazy_static::lazy_static! {
    static ref UNICODE_ESC: Regex =
        Regex::new(r"(\\U[[:xdigit:]]{8})|(\\u[[:xdigit:]]{4})").unwrap();
}

fn unescape_iri(iri: &str) -> String {
    let (new_iri, end) =
        UNICODE_ESC
            .captures_iter(iri)
            .fold((String::new(), 0), |(so_far, start), cap| {
                let cap = cap.get(0).unwrap();
                (
                    format!(
                        "{}{}{}",
                        so_far,
                        &iri[start..cap.start()],
                        unescape_uchar(cap.as_str())
                    ),
                    cap.end(),
                )
            });

    format!("{}{}", new_iri, &iri[end..])
}

fn unescape_uchar(uchar: &str) -> char {
    use std::char;
    let uchar = &uchar[2..];
    let uchar_u32 = u32::from_str_radix(uchar, 16).unwrap();
    char::from_u32(uchar_u32).unwrap()
}
