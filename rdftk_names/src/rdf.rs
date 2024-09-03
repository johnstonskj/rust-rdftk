/*!
Functions that create IRIs for the [RDF](https://www.w3.org/TR/rdf11-concepts/) namespace.
 */

use rdftk_iri::{Iri, IriRef};

namespace! {
    "rdf",
    "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    {
        html, "HTML",
        lang_string, "langString",
        plain_literal, "PlainLiteral",
        a_type, "type",
        property, "Property",
        statement, "Statement",
        subject, "subject",
        predicate, "predicate",
        object, "object",
        bag, "Bag",
        seq, "Seq",
        alt, "Alt",
        value, "value",
        list, "List",
        nil, "nil",
        first, "first",
        rest, "rest",
        li, "li",
        xml_literal, "XMLLiteral",
        json, "JSON",
        compound_literal, "CompoundLiteral",
        language, "language",
        direction, "direction"
    }
}

/// Create a numbered member in a container
#[inline]
pub fn member(index: usize) -> IriRef {
    Iri::from_str(&format!("{}_{}", NAMESPACE, index))
        .unwrap()
        .into()
}

/// Create a numbered member in a container
#[inline]
pub fn member_qname(index: usize) -> String {
    format!("{}:_{}", PREFIX, index)
}
