/*!
One-line description.

More detailed description, with

# Example

 */

use std::fmt::Display;

use rdftk_core::model::{literal::LiteralRef, statement::BlankNode};
use rdftk_iri::IriRef;

use crate::sparql::Variable;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Results {
    columns: Vec<Variable>,
    rows: Vec<Row>,
    offset: usize,
    more: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct Row(Vec<Value>);

#[derive(Clone, Debug)]
pub enum Value {
    BNode(BlankNode),
    Iri(IriRef),
    Literal(LiteralRef),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------

impl Results {
    pub fn columns(&self) -> impl Iterator<Item = &Variable> {
        self.columns.iter()
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn rows(&self) -> impl Iterator<Item = &Row> {
        self.rows.iter()
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn has_more(&self) -> Option<bool> {
        self.more
    }
}

// ------------------------------------------------------------------------------------------------

impl AsRef<Vec<Value>> for Row {
    fn as_ref(&self) -> &Vec<Value> {
        &self.0
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BNode(v) => v.to_string(),
                Self::Iri(v) => v.to_string(),
                Self::Literal(v) => v.to_string(),
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
