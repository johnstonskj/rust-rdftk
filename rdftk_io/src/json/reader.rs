/*!
Provides the `JsonReader` implementation of the `GraphReader` trait.

# Example

*/

use crate::common::ReaderOptions;
use crate::json::syntax::{
    BNODE_PREFIX, OBJ_KEY_DATATYPE, OBJ_KEY_LANG, OBJ_KEY_TYPE, OBJ_KEY_VALUE, OBJ_TYPE_BNODE,
    OBJ_TYPE_LITERAL, OBJ_TYPE_URI,
};
use objio::{impl_has_options, HasOptions, ObjectReader};
use rdftk_core::error::{read_write_error, read_write_error_with, Error};
use rdftk_core::model::graph::{GraphFactoryRef, GraphRef};
use rdftk_core::model::literal::{DataType, LanguageTag};
use rdftk_core::model::statement::SubjectNodeRef;
use rdftk_core::simple::statement::statement_factory;
use rdftk_iri::{Iri, IriRef};
use serde_json::{Map, Value};
use std::io::Read;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An implementation of the GraphReader trait to read resources in the JSON representation.
///
#[derive(Debug, Default)]
pub struct JsonReader {
    options: ReaderOptions,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_options!(JsonReader, ReaderOptions);

impl ObjectReader<GraphRef> for JsonReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<GraphRef, Self::Error>
    where
        R: Read,
    {
        let value: Value =
            serde_json::from_reader(r).map_err(|e| read_write_error_with(super::NAME, e))?;
        parse_graph(value, self.options().factory().clone())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_graph(value: Value, factory: GraphFactoryRef) -> Result<GraphRef, Error> {
    if let Value::Object(json) = value {
        let graph = factory.graph();
        for (subject, predicate_objects) in json.iter() {
            parse_statements(subject, predicate_objects, &graph)?;
        }
        Ok(graph)
    } else {
        log::error!("parse_graph() - expecting Value::Object");
        read_write_error(super::NAME).into()
    }
}

fn parse_statements(
    subject: &str,
    predicate_objects: &Value,
    graph: &GraphRef,
) -> Result<(), Error> {
    if let Value::Object(json) = predicate_objects {
        let subject = if subject.starts_with(BNODE_PREFIX) {
            graph
                .borrow()
                .statement_factory()
                .blank_subject_named(&subject[2..])?
        } else {
            graph
                .borrow()
                .statement_factory()
                .named_subject(IriRef::new(Iri::from_str(subject)?))
        };
        for (predicate, objects) in json.iter() {
            parse_predicates(&subject, predicate, objects, graph)?;
        }
        Ok(())
    } else {
        log::error!("parse_statements() - expecting Value::Object");
        read_write_error(super::NAME).into()
    }
}

fn parse_predicates(
    subject: &SubjectNodeRef,
    predicate: &str,
    objects: &Value,
    graph: &GraphRef,
) -> Result<(), Error> {
    if let Value::Array(json) = objects {
        let predicate = IriRef::new(Iri::from_str(predicate)?);
        for object in json {
            parse_object(subject, &predicate, object, graph)?;
        }
        Ok(())
    } else {
        log::error!("parse_predicates() - expecting Value::Array");
        read_write_error(super::NAME).into()
    }
}

fn parse_object(
    subject: &SubjectNodeRef,
    predicate: &IriRef,
    object: &Value,
    graph: &GraphRef,
) -> Result<(), Error> {
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
                    log::error!("parse_object() - unknown 'type' key value: {}", s);
                    read_write_error(super::NAME).into()
                }
            }
            _ => {
                log::error!("parse_object() - no 'type' key in object");
                read_write_error(super::NAME).into()
            }
        }
    } else {
        log::error!("parse_object() - expecting Value::Object");
        read_write_error(super::NAME).into()
    }
}

fn parse_literal_object(
    subject: &SubjectNodeRef,
    predicate: &IriRef,
    object: &Map<String, Value>,
    graph: &GraphRef,
) -> Result<(), Error> {
    let mut graph = graph.borrow_mut();
    let value = object.get(OBJ_KEY_VALUE);
    let language = object.get(OBJ_KEY_LANG);
    let data_type = object.get(OBJ_KEY_DATATYPE);

    let object = statement_factory().literal_object(match (value, language, data_type) {
        (Some(Value::String(v)), None, None) => graph.literal_factory().literal(v),
        (Some(Value::String(v)), Some(Value::String(l)), None) => graph
            .literal_factory()
            .with_language(v, LanguageTag::from_str(l)?),
        (Some(Value::String(v)), None, Some(Value::String(d))) => {
            let data_type = IriRef::new(Iri::from_str(d)?);
            graph
                .literal_factory()
                .with_data_type(v, DataType::from(data_type))
        }
        _ => {
            log::error!("parse_literal_object() - bad value/data type/language combination");
            return read_write_error(super::NAME).into();
        }
    });
    let st = graph
        .statement_factory()
        .statement(subject.clone(), predicate.clone(), object)?;
    graph.insert(st);
    Ok(())
}

fn parse_bnode_object(
    subject: &SubjectNodeRef,
    predicate: &IriRef,
    object: &Map<String, Value>,
    graph: &GraphRef,
) -> Result<(), Error> {
    let mut graph = graph.borrow_mut();
    if let Some(Value::String(s)) = object.get(OBJ_KEY_VALUE) {
        let object = graph.statement_factory().blank_object_named(&s[2..])?;
        let st = graph
            .statement_factory()
            .statement(subject.clone(), predicate.clone(), object)?;
        graph.insert(st);
        Ok(())
    } else {
        log::error!("parse_bnode_object() - expecting Value::String");
        read_write_error(super::NAME).into()
    }
}

fn parse_uri_object(
    subject: &SubjectNodeRef,
    predicate: &IriRef,
    object: &Map<String, Value>,
    graph: &GraphRef,
) -> Result<(), Error> {
    let mut graph = graph.borrow_mut();
    if let Some(Value::String(s)) = object.get(OBJ_KEY_VALUE) {
        let uri = IriRef::new(Iri::from_str(s)?);
        let object = graph.statement_factory().named_object(uri);
        let st = graph
            .statement_factory()
            .statement(subject.clone(), predicate.clone(), object)?;
        graph.insert(st);
        Ok(())
    } else {
        log::error!("parse_uri_object() - expecting Value::String");
        read_write_error(super::NAME).into()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
