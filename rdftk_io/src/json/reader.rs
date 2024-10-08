use super::NAME;
use crate::json::syntax::{
    BNODE_PREFIX, OBJ_KEY_DATATYPE, OBJ_KEY_LANG, OBJ_KEY_TYPE, OBJ_KEY_VALUE, OBJ_TYPE_BNODE,
    OBJ_TYPE_LITERAL, OBJ_TYPE_URI,
};
use objio::ObjectReader;
use rdftk_core::error::Error;
use rdftk_core::model::graph::Graph;
use rdftk_core::model::literal::{DataType, LanguageTag, Literal};
use rdftk_core::model::statement::{BlankNode, Statement, SubjectNode};
use rdftk_iri::Iri;
use serde_json::{Map, Value};
use std::io::Read;
use std::str::FromStr;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An implementation of the GraphReader trait to read resources in the JSON representation.
///
#[derive(Debug, Default)]
pub struct JsonReader {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectReader<Graph> for JsonReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<Graph, Self::Error>
    where
        R: Read,
    {
        let value: Value = serde_json::from_reader(r).map_err(json_error)?;
        parse_graph(value)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

macro_rules! parse_rule {
    ($rule_fn:literal entry) => {
        const RULE_FN: &'static str = $rule_fn;
        ::tracing::trace!("{}(...)", $rule_fn);
    };
}

fn json_error(err: serde_json::Error) -> Error {
    Error::Tokenizer {
        representation: NAME.into(),
        source: Box::new(err),
    }
}

fn value_variant(value: &Value) -> String {
    match value {
        Value::Null => "Null",
        Value::Bool(_) => "Bool",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    }
    .to_string()
}

fn parse_graph(value: Value) -> Result<Graph, Error> {
    parse_rule!("parse_graph" entry);

    if let Value::Object(json) = value {
        let mut graph = Graph::default();
        for (subject, predicate_objects) in json.iter() {
            parse_statements(subject, predicate_objects, &mut graph)?;
        }
        Ok(graph)
    } else {
        error!("rule {RULE_FN} expecting `Object` variant");
        Err(Error::ParserUnexpected {
            rule_fn: RULE_FN.into(),
            given: value_variant(&value),
            expecting: vec!["Object".into()],
        })
    }
}

fn parse_statements(
    subject: &str,
    predicate_objects: &Value,
    graph: &mut Graph,
) -> Result<(), Error> {
    parse_rule!("parse_statements" entry);

    if let Value::Object(json) = predicate_objects {
        let subject = if let Some(subject) = subject.strip_prefix(BNODE_PREFIX) {
            BlankNode::from_str(subject)?.into()
        } else {
            Iri::from_str(subject)?.into()
        };
        for (predicate, objects) in json.iter() {
            parse_predicates(&subject, predicate, objects, graph)?;
        }
        Ok(())
    } else {
        error!("rule {RULE_FN} expecting `Object` variant");
        Err(Error::ParserUnexpected {
            rule_fn: "parse_statements".into(),
            given: value_variant(predicate_objects),
            expecting: vec!["Object".into()],
        })
    }
}

fn parse_predicates(
    subject: &SubjectNode,
    predicate: &str,
    objects: &Value,
    graph: &mut Graph,
) -> Result<(), Error> {
    parse_rule!("parse_predicates" entry);

    if let Value::Array(json) = objects {
        let predicate = Iri::from_str(predicate)?;
        for object in json {
            parse_object(subject, &predicate, object, graph)?;
        }
        Ok(())
    } else {
        error!("rule {RULE_FN} expecting `Array` variant");
        Err(Error::ParserUnexpected {
            rule_fn: RULE_FN.into(),
            given: value_variant(objects),
            expecting: vec!["Array".into()],
        })
    }
}

fn parse_object(
    subject: &SubjectNode,
    predicate: &Iri,
    object: &Value,
    graph: &mut Graph,
) -> Result<(), Error> {
    parse_rule!("parse_object" entry);

    if let Value::Object(json) = object {
        match json.get(OBJ_KEY_TYPE) {
            Some(Value::String(s)) => {
                if s == OBJ_TYPE_LITERAL {
                    parse_literal_object(subject, predicate, json, graph)
                } else if s == OBJ_TYPE_BNODE {
                    parse_bnode_object(subject, predicate, json, graph)
                } else if s == OBJ_TYPE_URI {
                    parse_uri_object(subject, predicate, json, graph)
                } else {
                    error!("parse_object() - unknown 'type' key value: {}", s);
                    Err(Error::ParserUnexpected {
                        rule_fn: RULE_FN.into(),
                        given: s.into(),
                        expecting: vec![
                            OBJ_TYPE_LITERAL.into(),
                            OBJ_TYPE_BNODE.into(),
                            OBJ_TYPE_URI.into(),
                        ],
                    })
                }
            }
            _ => {
                error!(
                    "rule {RULE_FN} expecting object to have key {}",
                    OBJ_KEY_TYPE
                );
                Err(Error::ParserExpected {
                    rule_fn: RULE_FN.into(),
                    expecting: OBJ_KEY_TYPE.into(),
                })
            }
        }
    } else {
        error!("rule {RULE_FN} expecting `Object` variant");
        Err(Error::ParserUnexpected {
            rule_fn: RULE_FN.into(),
            given: value_variant(object),
            expecting: vec!["Object".into()],
        })
    }
}

fn parse_literal_object(
    subject: &SubjectNode,
    predicate: &Iri,
    object: &Map<String, Value>,
    graph: &mut Graph,
) -> Result<(), Error> {
    parse_rule!("parse_literal_object" entry);
    let value = object.get(OBJ_KEY_VALUE);
    let language = object.get(OBJ_KEY_LANG);
    let data_type = object.get(OBJ_KEY_DATATYPE);

    let object = match (value, language, data_type) {
        (Some(Value::String(v)), None, None) => Literal::plain(v),
        (Some(Value::String(v)), Some(Value::String(l)), None) => {
            Literal::with_language(v, LanguageTag::from_str(l)?)
        }
        (Some(Value::String(v)), None, Some(Value::String(d))) => {
            let data_type = Iri::from_str(d)?;
            Literal::with_data_type(v, DataType::from(data_type))
        }
        _ => {
            error!("parse_literal_object() - bad value/data type/language combination");
            return Err(Error::ParserUnreachable {
                rule_fn: RULE_FN.into(),
                given: "bad value/data type/language combination".into(),
            });
        }
    };
    let st = Statement::new(subject.clone(), predicate.clone(), object);
    graph.insert(st);
    Ok(())
}

fn parse_bnode_object(
    subject: &SubjectNode,
    predicate: &Iri,
    object: &Map<String, Value>,
    graph: &mut Graph,
) -> Result<(), Error> {
    parse_rule!("parse_bnode_object" entry);
    if let Some(Value::String(s)) = object.get(OBJ_KEY_VALUE) {
        let object = BlankNode::from_str(&s[2..])?;
        let st = Statement::new(subject.clone(), predicate.clone(), object);
        graph.insert(st);
        Ok(())
    } else {
        error!(
            "rule {RULE_FN} expecting object to have key {}",
            OBJ_KEY_VALUE
        );
        Err(Error::ParserExpected {
            rule_fn: RULE_FN.into(),
            expecting: OBJ_KEY_VALUE.into(),
        })
    }
}

fn parse_uri_object(
    subject: &SubjectNode,
    predicate: &Iri,
    object: &Map<String, Value>,
    graph: &mut Graph,
) -> Result<(), Error> {
    parse_rule!("parse_uri_object" entry);

    if let Some(Value::String(s)) = object.get(OBJ_KEY_VALUE) {
        let object = Iri::from_str(s)?;
        let st = Statement::new(subject.clone(), predicate.clone(), object);
        graph.insert(st);
        Ok(())
    } else {
        error!(
            "rule {RULE_FN} expecting object to have key {}",
            OBJ_KEY_VALUE
        );
        Err(Error::ParserExpected {
            rule_fn: RULE_FN.into(),
            expecting: OBJ_KEY_VALUE.into(),
        })
    }
}
