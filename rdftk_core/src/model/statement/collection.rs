use crate::{
    error::Error,
    model::{
        literal::Literal,
        statement::{object::ObjectNode, BlankNode, Statement},
    },
};
use rdftk_iri::{Iri, Name};
use rdftk_names::rdf;
use std::{fmt::Display, sync::Arc};

use super::SubjectNode;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An immutable collection type from
/// [RDF11-MT D.3](https://www.w3.org/TR/rdf11-mt/#rdf-collections)
///
/// > RDF provides a vocabulary for describing collections, i.e.'list structures', in terms of
/// > head-tail links. Collections differ from containers in allowing branching structure and in
/// > having an explicit terminator, allowing applications to determine the exact set of items in
/// > the collection.
/// >
/// > As with containers, no special semantic conditions are imposed on this vocabulary other than
/// > the type of `rdf:nil` being `rdf:List`.
///
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Collection(Vec<ObjectNode>);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<ObjectNode> for Collection {
    fn from(v: ObjectNode) -> Self {
        Self::from(vec![v])
    }
}

impl From<&ObjectNode> for Collection {
    fn from(v: &ObjectNode) -> Self {
        Self::from(vec![v.clone()])
    }
}

impl From<Vec<ObjectNode>> for Collection {
    fn from(vs: Vec<ObjectNode>) -> Self {
        Self(vs)
    }
}

impl From<BlankNode> for Collection {
    fn from(v: BlankNode) -> Self {
        Self::from(vec![ObjectNode::Blank(v)])
    }
}

impl From<&BlankNode> for Collection {
    fn from(v: &BlankNode) -> Self {
        Self::from(vec![ObjectNode::Blank(v.clone())])
    }
}

impl From<Name> for Collection {
    fn from(v: Name) -> Self {
        Self::from(vec![ObjectNode::Blank(v.into())])
    }
}

impl From<&Name> for Collection {
    fn from(v: &Name) -> Self {
        Self::from(vec![ObjectNode::Blank(v.clone().into())])
    }
}

impl From<Iri> for Collection {
    fn from(v: Iri) -> Self {
        Self::from(vec![ObjectNode::Resource(v)])
    }
}

impl From<&Iri> for Collection {
    fn from(v: &Iri) -> Self {
        Self::from(vec![ObjectNode::Resource(v.clone())])
    }
}

impl From<Literal> for Collection {
    fn from(v: Literal) -> Self {
        Self::from(vec![ObjectNode::Literal(v)])
    }
}

impl From<&Literal> for Collection {
    fn from(v: &Literal) -> Self {
        Self::from(vec![ObjectNode::Literal(v.clone())])
    }
}

impl From<Statement> for Collection {
    fn from(v: Statement) -> Self {
        Self::from(vec![ObjectNode::Statement(Arc::new(v))])
    }
}

impl From<&Statement> for Collection {
    fn from(v: &Statement) -> Self {
        Self::from(vec![ObjectNode::Statement(Arc::new(v.clone()))])
    }
}

impl From<Arc<Statement>> for Collection {
    fn from(v: Arc<Statement>) -> Self {
        Self::from(vec![ObjectNode::Statement(v)])
    }
}

impl From<&Arc<Statement>> for Collection {
    fn from(v: &Arc<Statement>) -> Self {
        Self::from(vec![ObjectNode::Statement(v.clone())])
    }
}

// ------------------------------------------------------------------------------------------------

impl FromIterator<ObjectNode> for Collection {
    fn from_iter<T: IntoIterator<Item = ObjectNode>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl From<Collection> for Vec<ObjectNode> {
    fn from(value: Collection) -> Self {
        value.0
    }
}

impl Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "( {} )",
            self.iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Collection {
    pub fn nil() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ObjectNode> {
        self.0.iter()
    }

    pub fn reify(&self) -> Result<(SubjectNode, Vec<Statement>), Error> {
        if self.is_empty() {
            Ok((SubjectNode::from(rdf::nil().clone()), Default::default()))
        } else {
            let mut statements: Vec<Statement> = Default::default();
            let first_subject: SubjectNode = BlankNode::generate().into();
            let mut subject: SubjectNode = first_subject.clone();

            for (idx, object) in self.0.iter().enumerate() {
                statements.push(Statement::new(
                    subject.clone(),
                    rdf::a_type().clone(),
                    rdf::list().clone(),
                ));
                // TODO: deal with recursive ObjectNode::Collection
                statements.push(Statement::new(
                    subject.clone(),
                    rdf::first().clone(),
                    object,
                ));
                let old_subject = subject.clone();
                let next_or_nil: ObjectNode = if idx == self.0.len() - 1 {
                    rdf::nil().clone().into()
                } else {
                    subject = BlankNode::generate().into();
                    subject.to_object()
                };
                statements.push(Statement::new(
                    old_subject,
                    rdf::rest().clone(),
                    next_or_nil,
                ));
            }

            Ok((first_subject, statements))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_reify_list() {
        let list: Vec<ObjectNode> = vec![
            BlankNode::from_str("aa").unwrap().into(),
            BlankNode::from_str("bb").unwrap().into(),
            BlankNode::from_str("cc").unwrap().into(),
        ];
        let collection: Collection = list.into();

        println!("{:#?}", collection.reify());
    }
}
