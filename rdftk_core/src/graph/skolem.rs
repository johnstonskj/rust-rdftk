/*!
Support for the Skolemization process of a graph. For more details on the process, see
[Skolemization (Informative)](https://www.w3.org/TR/rdf11-mt/#skolemization-informative)
and [Replacing Blank Nodes with IRIs](https://www.w3.org/TR/rdf11-concepts/#section-skolemization).

*/

use crate::error::Error;
use crate::graph::GraphRef;
use crate::statement::{ObjectNodeRef, StatementRef, SubjectNodeRef};
use crate::{Graph, ObjectNode, SubjectNode};
use rdftk_iri::{IRIRef, Path};
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Replace all blank nodes with new, unique IRIs. This creates a new graph and leaves the initial
/// graph unchanged. The base IRI is used to create identifiers, it's path will be replaced
/// entirely by a well-known format.
///
pub fn skolemize(graph: &impl Graph, base: &IRIRef) -> Result<GraphRef, Error> {
    let mut mapping: HashMap<String, IRIRef> = Default::default();

    let factory = graph.factory();

    let new_graph = factory.new_graph();

    for statement in graph.statements() {
        let mut new_statement: StatementRef = statement.clone();
        if let Some(blank) = new_statement.subject().as_blank() {
            if !mapping.contains_key(blank) {
                let _ = mapping.insert(blank.clone(), make_unique_iri(base));
            }
            let name = mapping.get(blank).unwrap().clone();
            Rc::get_mut(&mut new_statement)
                .unwrap()
                .set_subject(SubjectNodeRef::new(SubjectNode::named(name)));
        }
        if let Some(blank) = new_statement.object().as_blank() {
            if !mapping.contains_key(blank) {
                let _ = mapping.insert(blank.clone(), make_unique_iri(base));
            }
            let name = mapping.get(blank).unwrap().clone();
            Rc::get_mut(&mut new_statement)
                .unwrap()
                .set_object(ObjectNodeRef::new(ObjectNode::named(name)));
        }
        let mut mut_graph = new_graph.borrow_mut();
        mut_graph.insert(new_statement);
    }

    Ok(new_graph)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make_uuid() -> String {
    let new_uuid = Uuid::new_v4();
    new_uuid
        .to_simple()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_string()
}

fn make_unique_iri(base: &IRIRef) -> IRIRef {
    let iri_path = format!("/.well-known/genid/{}", make_uuid());
    let mut iri = base.as_ref().clone();
    iri.set_path(Path::from_str(&iri_path).unwrap());
    IRIRef::new(iri)
}
