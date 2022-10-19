use itertools::Itertools;
use rdftk_iri::IRIRef;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum TurtleTripleType {
    /// rdf:type triples come first and are written as 'a'
    Type,
    /// rdfs:label and/or skos:prefLabel triples come second
    Label,
    /// rdfs:comment and/or dct:description triples come third
    Comment,
    /// Everything else comes last
    Other,
}

impl TurtleTripleType {
    pub(crate) fn group_predicates<'a>(predicates: &[&'a IRIRef]) -> Vec<(TurtleTripleType, Vec<&'a IRIRef>)> {
        let mut result = predicates
            .iter()
            .group_by(Self::group_predicate)
            .into_iter()
            .map(|(triple_type, group)| (triple_type, group.cloned().collect()))
            .collect::<Vec<(TurtleTripleType, Vec<&IRIRef>)>>();
        result.sort_by_key(|a| a.0);
        result
    }

    fn group_predicate(predicate: &&&IRIRef) -> TurtleTripleType {
        match predicate.to_string().as_str() {
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" => TurtleTripleType::Type,
            "http://www.w3.org/2000/01/rdf-schema#label" => TurtleTripleType::Label,
            "http://xmlns.com/foaf/0.1/name" => TurtleTripleType::Label,
            "http://purl.org/dc/elements/1.1/title" => TurtleTripleType::Label,
            "http://www.w3.org/2000/01/rdf-schema#comment" => TurtleTripleType::Comment,
            "http://purl.org/dc/elements/1.1/description" => TurtleTripleType::Comment,
            _ => TurtleTripleType::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TurtleTripleType;
    use super::TurtleTripleType::*;

    #[test]
    fn test_order() {
        let mut v:Vec<TurtleTripleType> = vec![Comment, Label, Type, Other];
        v.sort();
        let sorted = format!("{:?}",v );
        assert_eq!(sorted, "[Type, Label, Comment, Other]");
    }
}