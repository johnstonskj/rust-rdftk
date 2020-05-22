/*!
One-line description.

More detailed description, with

# Example

*/

#![allow(clippy::module_name_repetitions)]

use crate::Graph;
use rdftk_iri::IRI;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait NamedGraph: Graph {
    fn name(&self) -> &Option<IRI>;

    fn set_name(&mut self, name: IRI) -> Option<IRI>;

    fn unset_name(&mut self) -> Option<IRI>;
}
