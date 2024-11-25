use std::cell::RefCell;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct Indenter {
    offset_in_chars: usize,
    indent_width: usize,
    current_indentation: RefCell<usize>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Indenter {
    fn default() -> Self {
        Self {
            offset_in_chars: 0,
            indent_width: 2,
            current_indentation: RefCell::new(0),
        }
    }
}

impl Display for Indenter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:width$}", "", width = self.current_indentation())
    }
}

impl Indenter {
    /// Set the default width, in characters, of each indent/outdent action.
    pub(crate) fn with_default_indent_width(self, width_in_chars: usize) -> Self {
        let mut self_mut = self;
        self_mut.indent_width = width_in_chars;
        self_mut
    }

    /// Set an offset, in characters, for indentation to start at.
    #[allow(dead_code)]
    pub(crate) fn with_initial_offset(self, offset_in_chars: usize) -> Self {
        let _ = self.current_indentation.replace(offset_in_chars);
        let mut self_mut = self;
        self_mut.offset_in_chars = offset_in_chars;
        self_mut
    }

    /// Return the current number of characters of indentation.
    pub(crate) fn current_indentation(&self) -> usize {
        *self.current_indentation.borrow()
    }

    /// Return the number of characters of initial offset.
    #[allow(dead_code)]
    pub(crate) fn initial_offset(&self) -> usize {
        self.offset_in_chars
    }

    pub(crate) fn is_not_indented(&self) -> bool {
        *self.current_indentation.borrow() == self.offset_in_chars
    }

    /// Return the width, in chars, for each indent/outdent action.
    #[allow(dead_code)]
    pub(crate) fn default_width(&self) -> usize {
        self.indent_width
    }

    /// Reset the current indentation to the value of the initial offset (0 by default).
    pub(crate) fn reset_depth(&self) {
        self.current_indentation.replace(self.offset_in_chars);
    }

    /// Indent, adding `indent_width` in chars to the `current_indentation` value.
    pub(crate) fn indent(&self) {
        self.indent_by(self.indent_width);
    }

    /// Indent, adding `by_chars` to the `current_indentation` value.
    pub(crate) fn indent_by(&self, by_chars: usize) {
        self.current_indentation
            .replace(self.current_indentation() + by_chars);
    }

    #[allow(dead_code)]
    pub(crate) fn indent_for<T: Into<usize>>(&self, for_: T) {
        self.indent_by(for_.into())
    }

    /// Outdent, subtracting `indent_width` in chars from the `current_indentation` value.
    pub(crate) fn outdent(&self) {
        self.outdent_by(self.indent_width);
    }

    /// Outdent, subtracting `by_chars` from the `current_indentation` value.
    #[allow(dead_code)]
    pub(crate) fn outdent_by(&self, by_chars: usize) {
        self.current_indentation
            .replace(self.current_indentation() - by_chars);
    }

    #[allow(dead_code)]
    pub(crate) fn outdent_for<T: Into<usize>>(&self, for_: T) {
        self.outdent_by(for_.into())
    }
}
