/*!
Provides the `JsonWriter` implementation of the `GraphWriter` trait.

```rust
use rdftk_io::json::writer::{JsonWriter};
use rdftk_io::write_graph_to_string;
# use rdftk_memgraph::MemGraph;
# fn make_graph() -> MemGraph { MemGraph::default() }

let writer = JsonWriter::pretty();

let result = write_graph_to_string(&writer, &make_graph());
```


*/

use crate::error::{ErrorKind, Result};
use crate::json::NAME;
use crate::GraphWriter;
use rdftk_core::graph::Graph;
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

const OBJ_KEY_VALUE: &str = "value";
const OBJ_KEY_TYPE: &str = "type";
const OBJ_KEY_LANG: &str = "lang";
const OBJ_KEY_DATATYPE: &str = "datatype";

const OBJ_TYPE_BNODE: &str = "bnode";
const OBJ_TYPE_URI: &str = "uri";
const OBJ_TYPE_LITERAL: &str = "literal";

impl Default for JsonWriter {
    fn default() -> Self {
        Self { pretty: false }
    }
}

impl GraphWriter for JsonWriter {
    fn write(&self, w: &mut impl Write, graph: &impl Graph) -> Result<()> {
        let mut json_graph = Map::new();
        for subject in graph.subjects() {
            let mut predicate_map = Map::new();
            for predicate in graph.predicates_for(subject) {
                let mut objects = Vec::new();
                for object in graph.objects_for(subject, predicate) {
                    let mut object_map = Map::new();
                    if object.is_blank() {
                        object_map.insert(
                            OBJ_KEY_TYPE.to_string(),
                            Value::String(OBJ_TYPE_BNODE.to_string()),
                        );
                        object_map.insert(
                            OBJ_KEY_VALUE.to_string(),
                            Value::String(object.as_blank().unwrap().to_string()),
                        );
                    } else if object.is_iri() {
                        object_map.insert(
                            OBJ_KEY_TYPE.to_string(),
                            Value::String(OBJ_TYPE_URI.to_string()),
                        );
                        object_map.insert(
                            OBJ_KEY_VALUE.to_string(),
                            Value::String(object.as_iri().unwrap().to_string()),
                        );
                    } else if object.is_literal() {
                        let literal = object.as_literal().unwrap();
                        object_map.insert(
                            OBJ_KEY_TYPE.to_string(),
                            Value::String(OBJ_TYPE_LITERAL.to_string()),
                        );
                        object_map.insert(
                            OBJ_KEY_VALUE.to_string(),
                            Value::String(literal.lexical_form().to_string()),
                        );
                        if let Some(language) = literal.language() {
                            object_map.insert(
                                OBJ_KEY_LANG.to_string(),
                                Value::String(language.to_string()),
                            );
                        }
                        if let Some(data_type) = literal.data_type() {
                            object_map.insert(
                                OBJ_KEY_DATATYPE.to_string(),
                                Value::String(data_type.to_string()),
                            );
                        }
                    } else {
                        return Err(ErrorKind::RdfStarNotSupported(NAME.to_string()).into());
                    }
                    objects.push(Value::Object(object_map));
                }
                predicate_map.insert(predicate.to_string(), Value::Array(objects));
            }
            json_graph.insert(subject.to_string(), Value::Object(predicate_map));
        }
        if self.pretty {
            serde_json::to_writer_pretty(w, &Value::Object(json_graph))?;
        } else {
            serde_json::to_writer(w, &Value::Object(json_graph))?;
        }

        Ok(())
    }
}

impl JsonWriter {
    pub fn pretty() -> Self {
        Self { pretty: true }
    }
}
// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
