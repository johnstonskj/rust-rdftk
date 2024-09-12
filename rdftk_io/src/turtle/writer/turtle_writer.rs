use crate::turtle::writer::cursor::TurtleCursor;
use crate::turtle::writer::options::TurtleOptions;
use objio::{impl_has_options, ObjectWriter};
use rdftk_core::error::Error;
use rdftk_core::model::graph::GraphRef;
use std::cell::RefCell;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug, Default)]
pub struct TurtleWriter {
    options: TurtleOptions,
}

impl_has_options!(TurtleWriter, TurtleOptions);

impl ObjectWriter<GraphRef> for TurtleWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &GraphRef) -> Result<(), Error>
    where
        W: Write,
    {
        let cursor = TurtleCursor::new(
            Rc::new(RefCell::from(w)),
            Rc::from(graph.deref().borrow()),
            self.options.clone(),
        );

        cursor.write()
    }
}
