use std::cell::RefCell;
use std::fmt::{Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct Indenter {
    width: usize,
    depth: RefCell<usize>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Indenter {
    fn default() -> Self {
        Self {
            width: 2,
            depth: RefCell::new(0),
        }
    }
}

impl Display for Indenter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:width$}",
            "",
            width = self.width * (*self.depth.borrow())
        )
    }
}

impl Indenter {
    pub(crate) fn with_default_indent_width(self, width: usize) -> Self {
        let mut self_mut = self;
        self_mut.width = width;
        self_mut
    }

    #[allow(dead_code)]
    pub(crate) fn with_initial_depth(self, depth: usize) -> Self {
        let _ = self.depth.replace(depth);
        self
    }

    pub(crate) fn depth(&self) -> usize {
        *self.depth.borrow()
    }

    #[allow(dead_code)]
    pub(crate) fn default_width(&self) -> usize {
        self.width
    }

    pub(crate) fn reset_depth(&self) {
        self.depth.replace(0);
    }

    pub(crate) fn indent(&self) {
        self.indent_by(self.width);
    }

    pub(crate) fn indent_by(&self, by: usize) {
        self.depth.replace(self.depth() + by);
    }

    #[allow(dead_code)]
    pub(crate) fn indent_for<T: Into<usize>>(&self, for_: T) {
        self.indent_by(for_.into())
    }

    pub(crate) fn outdent(&self) {
        self.indent_by(self.width);
    }

    #[allow(dead_code)]
    pub(crate) fn outdent_by(&self, by: usize) {
        self.depth.replace(self.depth() - by);
    }

    #[allow(dead_code)]
    pub(crate) fn outdent_for<T: Into<usize>>(&self, for_: T) {
        self.outdent_by(for_.into())
    }
}
