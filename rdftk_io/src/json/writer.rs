use crate::json::syntax::{
    OBJ_KEY_DATATYPE, OBJ_KEY_LANG, OBJ_KEY_TYPE, OBJ_KEY_VALUE, OBJ_TYPE_BNODE, OBJ_TYPE_LITERAL,
    OBJ_TYPE_URI,
};
use crate::json::NAME;
use objio::{impl_has_options, ObjectWriter};
use rdftk_core::error::{rdf_star_not_supported_error, Error};
use rdftk_core::model::graph::GraphRef;
use serde_json::{Map, Value};
use std::io::Write;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------
///
/// This struct implements the `GraphWriter` trait and will write out a serialized form of the
/// entire graph.
///
#[derive(Debug, Default)]
pub struct JsonWriter {
    options: JsonOptions,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct JsonOptions {
    pretty_print: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl JsonOptions {
    pub fn with_pretty_print(self, pretty_print: bool) -> Self {
        Self { pretty_print }
    }

    pub fn set_pretty_print(&mut self, pretty_print: bool) {
        self.pretty_print = pretty_print;
    }

    pub fn pretty_print(&self) -> bool {
        self.pretty_print
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_options!(JsonWriter, JsonOptions);

impl ObjectWriter<GraphRef> for JsonWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, object: &GraphRef) -> Result<(), Self::Error>
    where
        W: Write,
    {
        let graph = object.borrow();

        let mut json_graph = Map::new();
        for subject in graph.subjects() {
            let mut predicate_map = Map::new();
            for predicate in graph.predicates_for(subject) {
                let mut objects = Vec::new();
                for object in graph.objects_for(subject, predicate) {
                    let mut object_map = Map::new();
                    if object.is_blank() {
                        let _ = object_map.insert(
                            OBJ_KEY_TYPE.to_string(),
                            Value::String(OBJ_TYPE_BNODE.to_string()),
                        );
                        let _ = object_map.insert(
                            OBJ_KEY_VALUE.to_string(),
                            Value::String(object.as_blank().unwrap().to_string()),
                        );
                    } else if object.is_iri() {
                        let _ = object_map.insert(
                            OBJ_KEY_TYPE.to_string(),
                            Value::String(OBJ_TYPE_URI.to_string()),
                        );
                        let _ = object_map.insert(
                            OBJ_KEY_VALUE.to_string(),
                            Value::String(object.as_iri().unwrap().to_string()),
                        );
                    } else if object.is_literal() {
                        let literal = object.as_literal().unwrap();
                        let _ = object_map.insert(
                            OBJ_KEY_TYPE.to_string(),
                            Value::String(OBJ_TYPE_LITERAL.to_string()),
                        );
                        let _ = object_map.insert(
                            OBJ_KEY_VALUE.to_string(),
                            Value::String(literal.lexical_form().to_string()),
                        );
                        if let Some(language) = literal.language() {
                            let _ = object_map.insert(
                                OBJ_KEY_LANG.to_string(),
                                Value::String(language.to_string()),
                            );
                        }
                        if let Some(data_type) = literal.data_type() {
                            let _ = object_map.insert(
                                OBJ_KEY_DATATYPE.to_string(),
                                Value::String(data_type.as_iri().to_string()),
                            );
                        }
                    } else {
                        return rdf_star_not_supported_error(NAME).into();
                    }
                    objects.push(Value::Object(object_map));
                }
                let _ = predicate_map.insert(predicate.to_string(), Value::Array(objects));
            }
            let _ = json_graph.insert(subject.to_string(), Value::Object(predicate_map));
        }
        if self.options.pretty_print() {
            serde_json::to_writer_pretty(w, &Value::Object(json_graph)).map_err(json_error)?;
        } else {
            serde_json::to_writer(w, &Value::Object(json_graph)).map_err(json_error)?;
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn json_error(e: serde_json::Error) -> Error {
    error!("Error generating JSON source: {:?}", e);
    Error::Tokenizer {
        representation: super::NAME.into(),
        source: Box::new(e),
    }
}
