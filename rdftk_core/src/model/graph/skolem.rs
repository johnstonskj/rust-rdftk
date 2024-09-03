/*!
Support for the Skolemization process of a graph. For more details on the process, see
[Skolemization (Informative)](https://www.w3.org/TR/rdf11-mt/#skolemization-informative)
and [Replacing Blank Nodes with IRIs](https://www.w3.org/TR/rdf11-concepts/#section-skolemization).

*/

use crate::error::{Error, ErrorKind};
use crate::model::graph::{Graph, GraphRef};
use crate::model::statement::{BlankNode, StatementRef};
use rdftk_iri::{new_genid, IriRef};
use std::collections::HashMap;
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Replace all blank nodes with new, unique Iris. This creates a new graph and leaves the initial
/// graph unchanged. The base Iri is used to create identifiers, it's path will be replaced
/// entirely by a well-known format.
///
pub fn skolemize(graph: &impl Graph, base: &IriRef) -> Result<GraphRef, Error> {
    let mut mapping: HashMap<BlankNode, IriRef> = Default::default();

    let factory = graph.factory();

    let new_graph = factory.graph();

    for statement in graph.statements() {
        let factory = graph.statement_factory();
        let mut new_statement: StatementRef = statement.clone();
        let mut_statement = match Rc::get_mut(&mut new_statement) {
            None => return Err(ErrorKind::InvalidState.into()),
            Some(st) => st,
        };
        if let Some(blank) = mut_statement.subject().as_blank() {
            if !mapping.contains_key(blank) {
                let _ = mapping.insert(blank.clone(), new_genid(base)?);
            }
            let name = mapping.get(blank).unwrap().clone();
            let subject = factory.named_subject(name);
            mut_statement.set_subject(subject);
        }
        if let Some(blank) = mut_statement.object().as_blank() {
            if !mapping.contains_key(blank) {
                let _ = mapping.insert(blank.clone(), new_genid(base)?);
            }
            let name = mapping.get(blank).unwrap().clone();
            let object = factory.named_object(name);
            mut_statement.set_object(object);
        }
        let mut mut_graph = new_graph.borrow_mut();
        mut_graph.insert(new_statement);
    }

    Ok(new_graph)
}
