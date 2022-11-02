/*!
One-line description.

More detailed description, with

# Example

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use std::cmp::max;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Indenter {
    width: u16,
    pub(crate) depth: u8,
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
        Self::with_width(2)
    }
}

impl Display for Indenter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:width$}",
            "",
            width = (self.width * self.depth as u16) as usize
        )
    }
}

impl Indenter {
    pub(crate) fn with_width(width: u16) -> Self {
        Self { width, depth: 0 }
    }

    pub(crate) fn depth(&self) -> u8 {
        self.depth
    }

    pub(crate) fn indent(&self) -> Self {
        self.indent_by(1)
    }

    pub(crate) fn indent_by(&self, by: u8) -> Self {
        Self {
            width: self.width,
            depth: self.depth + by,
        }
    }

    pub(crate) fn outdent(&self) -> Self {
        self.outdent_by(1)
    }

    pub(crate) fn outdent_by(&self, by: u8) -> Self {
        Self {
            width: self.width,
            depth: max(0, self.depth - by),
        }
    }

    #[allow(unused)]
    pub(crate) fn one(&self) -> String {
        format!("{:width$}", "", width = self.width as usize)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
