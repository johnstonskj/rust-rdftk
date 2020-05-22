/*!
One-line description.

More detailed description, with

# Example

*/

#![allow(clippy::module_name_repetitions)]

use crate::Graph;
use rdftk_core::SubjectNode;
use rdftk_iri::IRI;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// TODO: this needs more
pub trait CachingGraph: Graph {
    fn blank_subject(&self) -> &SubjectNode;
    fn blank_subject_named(&self, name: &str) -> &SubjectNode;
    fn subject(&self, iri: &IRI) -> &SubjectNode;
}
