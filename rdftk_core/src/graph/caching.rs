/*!
A trait for implementations that can cache values and save space.
*/

use crate::{Literal, SubjectNode};
use rdftk_iri::IRIRef;
use std::time::Duration;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ValueFactory {
    fn blank_subject_named(&self, name: &str) -> &SubjectNode;
    fn subject(&self, iri: &IRIRef) -> &SubjectNode;

    fn string(&self, v: &str) -> &Literal;
    fn qname(&self, v: &str) -> &Literal;
    fn uri(&self, v: &IRIRef) -> &Literal;
    fn boolean(&self, v: &bool) -> &Literal;
    fn float(&self, v: &f32) -> &Literal;
    fn double(&self, v: &f64) -> &Literal;
    fn long(&self, v: i64) -> &Literal;
    fn int(&self, v: i32) -> &Literal;
    fn short(&self, v: i16) -> &Literal;
    fn byte(&self, v: i8) -> &Literal;
    fn unsigned_long(&self, v: u64) -> &Literal;
    fn unsigned_int(&self, v: u32) -> &Literal;
    fn unsigned_short(&self, v: u16) -> &Literal;
    fn unsigned_byte(&self, v: u8) -> &Literal;
    fn duration(&self, v: &Duration) -> &Literal;
}
