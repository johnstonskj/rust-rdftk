use super::ntriples::{object as nt_object, predicate as nt_predicate, subject as nt_subject};
use super::Rule;
use pest::iterators::Pair;
use rdftk_core::model::data_set::DataSet;
use rdftk_core::model::graph::named::{GraphName, GraphNameRef};
use rdftk_core::model::graph::NamedGraphRef;
use rdftk_core::model::statement::{ObjectNodeRef, StatementRef, SubjectNodeRef};
use rdftk_core::{
    error::Error,
    model::data_set::{DataSetFactoryRef, DataSetRef},
};
use rdftk_iri::IriRef;
use std::cell::RefMut;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn parse_doc(
    input_pair: Pair<'_, Rule>,
    factory: DataSetFactoryRef,
) -> Result<DataSetRef, Error> {
    parse_rule!("nquadDoc" entry input_pair);

    let data_set = factory.data_set();

    if input_pair.as_rule() == Rule::nquadDoc {
        for inner_pair in input_pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::nquad => {
                    nquad(inner_pair, data_set.borrow_mut())?;
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

fn subject_to_name(subject: SubjectNodeRef) -> GraphNameRef {
    let name: GraphName = subject.into();
    name.into()
}

fn nquad(input_pair: Pair<'_, Rule>, data_set: RefMut<'_, dyn DataSet>) -> Result<(), Error> {
    parse_rule!("nquad" entry input_pair);

    let mut data_set = data_set;

    let graphs = data_set.graph_factory();
    let statements = graphs.statement_factory();
    let literals = statements.literal_factory();

    if input_pair.as_rule() == Rule::nquad {
        let mut inner_pairs = input_pair.into_inner();
        let subject: SubjectNodeRef = nt_subject(inner_pairs.next().unwrap(), &statements)?;
        let predicate: IriRef = nt_predicate(inner_pairs.next().unwrap())?;
        let object: ObjectNodeRef = nt_object(inner_pairs.next().unwrap(), &statements, &literals)?;
        let statement: StatementRef = statements.statement(subject, predicate, object)?;
        let graph: &mut NamedGraphRef = if let Some(new_inner_pair) = inner_pairs.next() {
            let graph_name = subject_to_name(nt_subject(new_inner_pair, &statements)?);
            if let Some(graph) = data_set.graph_mut(&Some(graph_name.clone())) {
                graph
            } else {
                data_set.insert(graphs.named_graph(Some(graph_name.clone())));
                data_set.graph_mut(&Some(graph_name)).unwrap()
            }
        } else if let Some(graph) = data_set.graph_mut(&None) {
            graph
        } else {
            data_set.insert(graphs.named_graph(None));
            data_set.graph_mut(&None).unwrap()
        };
        let mut graph_mut = graph.borrow_mut();
        graph_mut.insert(statement);
        Ok(())
    } else {
        Err(pest_error!(unexpected RULE_FN, &input_pair, [Rule::nquad]))
    }
}
