/*!
One-line description.

More detailed description, with

# Example

*/

use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Not all the features expressed in the Graph APIs are required to be implemented by a
/// particular type. This trait allows a client to determine which features are supported.
///
pub trait Featured {
    ///
    /// Return true if this instance, or factory, supports the feature identified by the IRI.
    ///
    fn supports_feature(&self, feature: &IRIRef) -> bool;
}

lazy_static! {

    // --------------------------------------------------------------------------------------------
    // DataSet features
    // --------------------------------------------------------------------------------------------

    ///
    /// If true, a data set's default model.graph is a combination of all named graphs. This implies
    /// that `set_default_graph` and `unset_default_graph` have no effect.
    ///
    pub static ref FEATURE_COMBINED_DEFAULT: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/model.data_set/combined_default").unwrap()
    );

    // --------------------------------------------------------------------------------------------
    // Graph/Statement features
    // --------------------------------------------------------------------------------------------

    ///
    /// Denotes that this graph accepts duplicate statements.
    ///
    pub static ref FEATURE_GRAPH_DUPLICATES: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/graph/duplicates").unwrap());

    ///
    /// This model.graph, or corresponding statement, supports
    /// [RDF-star](https://w3c.github.io/rdf-star/cg-spec/editors_draft.html).
    ///
    pub static ref FEATURE_RDF_STAR: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/graph/rdf_star").unwrap());

    ///
    /// This model.graph, or corresponding statement, supports
    /// [N3 Formula](https://www.w3.org/TeamSubmission/n3/#Quoting)
    ///
    pub static ref FEATURE_N3_FORMULAE: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/graph/n3_formulae").unwrap());

    // --------------------------------------------------------------------------------------------
    // Index features
    // --------------------------------------------------------------------------------------------

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_SUBJECT: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/subject").unwrap());

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_PREDICATE: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/predicate").unwrap());

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_OBJECT: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/object").unwrap());

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_SUBJECT_PREDICATE: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_predicate").unwrap()
    );

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_SUBJECT_PREDICATE_OBJECT: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_predicate_object").unwrap()
    );

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_SUBJECT_OBJECT: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/subject_object").unwrap());

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_PREDICATE_OBJECT: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/predicate_object").unwrap()
    );

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_GRAPH: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/model.graph").unwrap());

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_SUBJECT_GRAPH: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/subject_graph").unwrap());

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_PREDICATE_GRAPH: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/predicate_graph").unwrap());

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_OBJECT_GRAPH: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/object_graph").unwrap());

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_SUBJECT_PREDICATE_GRAPH: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_predicate_graph").unwrap()
    );

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_SUBJECT_PREDICATE_OBJECT_GRAPH: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_predicate_object_graph")
            .unwrap()
    );

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_SUBJECT_OBJECT_GRAPH: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_object_graph").unwrap()
    );

    /// Used to determine whether a specific index combination is supported.
    pub static ref FEATURE_IDX_PREDICATE_OBJECT_GRAPH: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/predicate_object_graph").unwrap()
    );
}
