/*!
Feature identifiers for indexes supported by graphs and data sets.

# Example

```rust
use rdftk_iri::IRIRef;
use rdftk_core::Graph;
use rdftk_core::indices::FEATURE_IDX_SUBJECT_PREDICATE_OBJECT;

fn has_spo_index(graph: &impl Graph) -> bool {
    graph.supports_feature((&FEATURE_IDX_SUBJECT_PREDICATE_OBJECT as &IRIRef))
}
```

*/

#![allow(missing_docs)]

use rdftk_iri::{IRIRef, IRI};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    pub static ref FEATURE_IDX_SUBJECT: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/subject").unwrap());
    pub static ref FEATURE_IDX_PREDICATE: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/predicate").unwrap());
    pub static ref FEATURE_IDX_OBJECT: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/object").unwrap());
    pub static ref FEATURE_IDX_SUBJECT_PREDICATE: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_predicate").unwrap()
    );
    pub static ref FEATURE_IDX_SUBJECT_PREDICATE_OBJECT: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_predicate_object").unwrap()
    );
    pub static ref FEATURE_IDX_SUBJECT_OBJECT: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/subject_object").unwrap());
    pub static ref FEATURE_IDX_PREDICATE_OBJECT: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/predicate_object").unwrap()
    );
    pub static ref FEATURE_IDX_GRAPH: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/graph").unwrap());
    pub static ref FEATURE_IDX_SUBJECT_GRAPH: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/subject_graph").unwrap());
    pub static ref FEATURE_IDX_PREDICATE_GRAPH: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/predicate_graph").unwrap());
    pub static ref FEATURE_IDX_OBJECT_GRAPH: IRIRef =
        IRIRef::from(IRI::from_str("http://rust-rdftk.dev/feature/index/object_graph").unwrap());
    pub static ref FEATURE_IDX_SUBJECT_PREDICATE_GRAPH: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_predicate_graph").unwrap()
    );
    pub static ref FEATURE_IDX_SUBJECT_PREDICATE_OBJECT_GRAPH: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_predicate_object_graph")
            .unwrap()
    );
    pub static ref FEATURE_IDX_SUBJECT_OBJECT_GRAPH: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/subject_object_graph").unwrap()
    );
    pub static ref FEATURE_IDX_PREDICATE_OBJECT_GRAPH: IRIRef = IRIRef::from(
        IRI::from_str("http://rust-rdftk.dev/feature/index/predicate_object_graph").unwrap()
    );
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
