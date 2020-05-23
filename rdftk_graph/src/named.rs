/*!
An extension to the core `Graph` to support named graphs.

# Example

TBD

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
