/*!
This module contains the traits and types used to describe an abstract DataSet, Graph, and Statement
RDF model.
*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A trait implemented by types that are constructed by providers. This allows for providers to
/// ensure that values belong to them.
///
/// Note, by convention the fully qualified crate/module name is used as a provider name.
///
pub trait Provided {
    ///
    /// Return the identifier for the provider associated with this instance.
    ///
    fn provider_id(&self) -> &'static str;
}

///
/// Denotes equivalence between Self and some other type. Equivalence is a very specific,
/// non-symmetric, non-transitive, directed type to type equality.
///
pub trait Equiv<T>
where
    T: Sized,
{
    /// Returns `true` if `other` is equivalent to `self`, else `false`.
    fn eqv(&self, other: &T) -> bool;

    /// Returns `true` if `other` is **not** equivalent to `self`, else `false`.
    fn not_eqv(&self, other: &T) -> bool {
        !self.eqv(other)
    }
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

pub mod data_set;

pub mod features;

pub mod graph;

pub mod literal;

pub mod qname;

pub mod statement;
