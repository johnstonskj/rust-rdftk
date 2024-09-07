use crate::turtle::writer::cursor::TurtleCursor;
use crate::turtle::writer::options::TurtleOptions;
use crate::GraphWriterWithOptions;
use rdftk_core::model::graph::GraphRef;
use std::cell::RefCell;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub struct TurtleWriter {}

impl GraphWriterWithOptions for TurtleWriter {
    type Options = TurtleOptions;

    fn write_with_options<W>(
        &self,
        w: &mut W,
        graph: &GraphRef,
        options: &Self::Options,
    ) -> rdftk_core::error::Result<()>
    where
        W: Write,
    {
        let cursor = TurtleCursor::new(
            Rc::new(RefCell::from(w)),
            Rc::from(graph.deref().borrow()),
            options.clone(),
        );

        cursor.write()
    }
}
