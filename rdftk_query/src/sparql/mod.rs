/*!
One-line description.

More detailed description, with

# Example

 */

// use ...

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub enum Prologue {
    Base(String),
    Prefix(Option<String>, String),
}

pub struct SelectQuery {
    prolog: Vec<Prologue>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variable(String);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl AsRef<str> for Variable {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<[u8]> for Variable {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Variable {
    type Err = rdftk_core::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            unimplemented!()
        }
    }
}

impl From<Variable> for String {
    fn from(v: Variable) -> Self {
        v.0
    }
}

impl Variable {
    pub fn new_unchecked<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    ///
    /// Returns `true` if the string is a valid variable name, else
    /// `false`.
    ///
    pub fn is_valid(s: &str) -> bool {
        // is_xml_name(if let Some(s) = s.strip_prefix(BLANK_NODE_PREFIX) {
        //     s
        // } else {
        //     s
        // }
        false
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod parser;

pub mod results;
