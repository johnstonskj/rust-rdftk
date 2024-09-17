/*!
Internal, common, code for reader and writer implementations.

*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "nt")]
pub(crate) mod indenter;

#[cfg(any(
    feature = "n3",
    feature = "nq",
    feature = "nt",
    feature = "trig",
    feature = "turtle"
))]
pub(crate) mod parser;
