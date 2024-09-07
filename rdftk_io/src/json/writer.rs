/*!
Provides the `JsonWriter` implementation of the `GraphWriter` trait.

```rust
use rdftk_io::json::writer::{JsonWriter};
use rdftk_io::write_graph_to_string;
# use rdftk_core::model::graph::GraphRef;
# fn make_graph() -> GraphRef { rdftk_core::simple::graph::graph_factory().graph() }

let writer = JsonWriter::pretty();

let result = write_graph_to_string(&writer, &make_graph());
```


*/

use crate::json::syntax::{
    OBJ_KEY_DATATYPE, OBJ_KEY_LANG, OBJ_KEY_TYPE, OBJ_KEY_VALUE, OBJ_TYPE_BNODE, OBJ_TYPE_LITERAL,
    OBJ_TYPE_URI,
};
use crate::json::NAME;
use crate::GraphWriter;
use rdftk_core::error::{Error, ErrorKind, Result};
use rdftk_core::model::graph::GraphRef;
use serde_json::{Map, Value};
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------
///
/// This struct implements the `GraphWriter` trait and will write out a serialized form of the
/// entire graph.
///
#[derive(Debug)]
pub struct JsonWriter {
    pretty: bool,
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

impl Default for JsonWriter {
    fn default() -> Self {
        Self { pretty: false }
    }
}

impl GraphWriter for JsonWriter {
    fn write<W>(&self, w: &mut W, graph: &GraphRef) -> Result<()>
    where
        W: Write,
    {
        let graph = graph.borrow();

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
                        return Err(ErrorKind::RdfStarNotSupported(NAME.to_string()).into());
                    }
                    objects.push(Value::Object(object_map));
                }
                let _ = predicate_map.insert(predicate.to_string(), Value::Array(objects));
            }
            let _ = json_graph.insert(subject.to_string(), Value::Object(predicate_map));
        }
        if self.pretty {
            serde_json::to_writer_pretty(w, &Value::Object(json_graph)).map_err(json_error)?;
        } else {
            serde_json::to_writer(w, &Value::Object(json_graph)).map_err(json_error)?;
        }

        Ok(())
    }
}

impl JsonWriter {
    /// Construct a writer that will output a pretty-printed form.
    pub fn pretty() -> Self {
        Self { pretty: true }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn json_error(e: serde_json::Error) -> Error {
    log::error!("Error parsing JSON source: {:?}", e);
    Error::with_chain(e, ErrorKind::ReadWrite(NAME.to_string()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
