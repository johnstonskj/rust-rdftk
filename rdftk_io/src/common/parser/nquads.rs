use super::ntriples::{object as nt_object, predicate as nt_predicate, subject as nt_subject};
use super::Rule;
use pest::iterators::Pair;
use rdftk_core::error::Error;
use rdftk_core::model::data_set::DataSet;
use rdftk_core::model::graph::{Graph, GraphName};
use rdftk_core::model::statement::{ObjectNode, Statement, SubjectNode};
use rdftk_iri::Iri;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_doc(input_pair: Pair<'_, Rule>) -> Result<DataSet, Error> {
    parse_rule!("nquadDoc" entry input_pair);

    let mut data_set = DataSet::default();

    if input_pair.as_rule() == Rule::nquadDoc {
        for inner_pair in input_pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::nquad => {
                    nquad(inner_pair, &mut data_set)?;
                }
                Rule::EOI => {}
                _ => {
                    return Err(pest_error!(
                        unexpected
                        RULE_FN,
                        &inner_pair,
                        [Rule::nquad, Rule::EOI]
                    ));
                }
            }
        }
        Ok(data_set)
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::nquadDoc]))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn subject_to_graph_name(subject: SubjectNode) -> GraphName {
    subject.into()
}

fn nquad(input_pair: Pair<'_, Rule>, data_set: &mut DataSet) -> Result<(), Error> {
    parse_rule!("nquad" entry input_pair);

    if input_pair.as_rule() == Rule::nquad {
        let mut inner_pairs = input_pair.into_inner();
        let subject: SubjectNode = nt_subject(inner_pairs.next().unwrap())?;
        let predicate: Iri = nt_predicate(inner_pairs.next().unwrap())?;
        let object: ObjectNode = nt_object(inner_pairs.next().unwrap())?;
        let statement: Statement = Statement::new(subject, predicate, object);
        let graph: &mut Graph = if let Some(new_inner_pair) = inner_pairs.next() {
            let graph_name = subject_to_graph_name(nt_subject(new_inner_pair)?);
            if let Some(graph) = data_set.graph_mut(&Some(graph_name.clone())) {
                graph
            } else {
                data_set.insert(Graph::named(graph_name.clone()));
                data_set.graph_mut(&Some(graph_name)).unwrap()
            }
        } else if let Some(graph) = data_set.graph_mut(&None) {
            graph
        } else {
            data_set.insert(Graph::default());
            data_set.graph_mut(&None).unwrap()
        };
        graph.insert(statement);
        Ok(())
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::nquad]))
    }
}
