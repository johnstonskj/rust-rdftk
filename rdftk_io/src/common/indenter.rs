/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub(crate) struct Indenter {
    width: usize,
    depth: usize,
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

impl Default for Indenter {
    fn default() -> Self {
        Self { width: 2, depth: 0 }
    }
}

impl Display for Indenter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:width$}", "", width = self.width * self.depth)
    }
}

impl Indenter {
    #[allow(dead_code)]
    pub(crate) fn with_width(width: usize) -> Self {
        Self { width, depth: 0 }
    }

    pub(crate) fn depth(&self) -> usize {
        self.depth
    }

    pub(crate) fn indent(&self) -> Self {
        Self {
            width: self.width,
            depth: self.depth + 1,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn indent_by(&self, by: usize) -> Self {
        Self {
            width: self.width,
            depth: self.depth + by,
        }
    }

    pub(crate) fn outdent(&self) -> Self {
        Self {
            width: self.width,
            depth: self.depth - 1,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn outdent_by(&self, by: usize) -> Self {
        Self {
            width: self.width,
            depth: self.depth - by,
        }
    }

    pub(crate) fn one(&self) -> String {
        format!("{:width$}", "", width = self.width)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
