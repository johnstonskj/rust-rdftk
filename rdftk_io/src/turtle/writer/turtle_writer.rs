#[allow(unused_imports)]
use std::borrow::BorrowMut as DummyABC;
use std::cell::RefCell;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

use rdftk_core::model::graph::GraphRef;
use rdftk_iri::IRIRef;

use crate::GraphWriter;
use crate::turtle::writer::cursor::TurtleCursor;
use crate::turtle::writer::options::TurtleOptions;

#[derive(Debug, Default)]
pub struct TurtleWriter {
    pub options: TurtleOptions,
}

impl GraphWriter for TurtleWriter {
    fn write(&self, w: &mut impl Write, graph: &GraphRef) -> rdftk_core::error::Result<()> {
        let cursor = TurtleCursor::new(
            Rc::new(RefCell::from(w)),
            Rc::from(graph.deref().borrow()),
            self.options.clone(),
        );

        cursor.write()
    }
}

impl TurtleWriter {
    ///
    /// Create a new writer with the provided options, this is used to override the default
    /// options that are used when calling `Default::default`.
    ///
    pub fn new(options: TurtleOptions) -> Self {
        Self {
            options,
        }
    }

    /// Return a new instance of the given `TurtleWriter` where the `id_base` (in its `TurtleOptions`)
    /// is set to the given IRI which will instruct the `TurtleWriter` to generate a `@base <id_base>`
    /// or `BASE <id_base>` statement at the top of the file.
    pub fn with_id_base(id_base: &IRIRef, options: TurtleOptions) -> Self {
        Self::new(options.with_id_base(Some(id_base)))
    }

    pub fn with_indent_with(width: u16) -> Self {
        Self::new(TurtleOptions::default().with_indent_width(width))
    }
}
