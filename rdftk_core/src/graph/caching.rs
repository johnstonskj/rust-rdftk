/*!
Graphs may have mechanisms to cache commonly used values, or those with significant storage
overhead. In such cases they provide a value factory that should be used to construct new values
for use in the associated graph. It is possible that all graphs provided by some graph store share
a common value factory by store rather than by graph.
*/

use crate::statement::SubjectNodeRef;
use crate::Literal;
use rdftk_iri::IRIRef;
use std::time::Duration;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A value factory can be used to provide previously cached values rather than creating duplicates
/// within a graph.
///
pub trait ValueFactory {
    /// Returns a cached subject node with the provided blank node name.
    fn blank_subject_named(&self, name: &str) -> &SubjectNodeRef;
    /// Returns a cached subject node with the provided IRI reference.
    fn subject(&self, iri: &IRIRef) -> &SubjectNodeRef;

    /// Returns a cached literal value with the provided string.
    fn string(&self, v: &str) -> &Literal;
    /// Returns a cached literal value with the provided QName.
    fn qname(&self, v: &str) -> &Literal;
    /// Returns a cached literal value with the provided IRI.
    fn uri(&self, v: &IRIRef) -> &Literal;
    /// Returns a cached literal value with the provided boolean.
    fn boolean(&self, v: &bool) -> &Literal;
    /// Returns a cached literal value with the provided float.
    fn float(&self, v: &f32) -> &Literal;
    /// Returns a cached literal value with the provided double.
    fn double(&self, v: &f64) -> &Literal;
    /// Returns a cached literal value with the provided long.
    fn long(&self, v: i64) -> &Literal;
    /// Returns a cached literal value with the provided int.
    fn int(&self, v: i32) -> &Literal;
    /// Returns a cached literal value with the provided short.
    fn short(&self, v: i16) -> &Literal;
    /// Returns a cached literal value with the provided byte.
    fn byte(&self, v: i8) -> &Literal;
    /// Returns a cached literal value with the provided unsigned long.
    fn unsigned_long(&self, v: u64) -> &Literal;
    /// Returns a cached literal value with the provided unsigned int.
    fn unsigned_int(&self, v: u32) -> &Literal;
    /// Returns a cached literal value with the provided unsigned short.
    fn unsigned_short(&self, v: u16) -> &Literal;
    /// Returns a cached literal value with the provided unigned byte.
    fn unsigned_byte(&self, v: u8) -> &Literal;
    /// Returns a cached literal value with the provided duration.
    fn duration(&self, v: &Duration) -> &Literal;
}
