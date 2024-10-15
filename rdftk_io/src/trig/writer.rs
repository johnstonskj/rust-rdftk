use crate::turtle::TurtleWriterOptions;
use crate::{DataSetWriter, GraphWriter};
use objio::{impl_has_options, HasOptions, ObjectWriter};
use rdftk_core::error::Error;
use rdftk_core::model::data_set::DataSet;
use rdftk_core::model::graph::Graph;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct TrigWriterOptions {
    turtle: TurtleWriterOptions,
    omit_graph_keyword: bool,
}

#[derive(Debug, Default)]
pub struct TrigWriter {
    options: TrigWriterOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<TurtleWriterOptions> for TrigWriterOptions {
    fn from(value: TurtleWriterOptions) -> Self {
        Self {
            turtle: value,
            ..Default::default()
        }
    }
}

impl AsRef<TurtleWriterOptions> for TrigWriterOptions {
    fn as_ref(&self) -> &TurtleWriterOptions {
        &self.turtle
    }
}

impl TrigWriterOptions {
    pub fn with_omit_graph_keyword(self, omit_graph_keyword: bool) -> Self {
        Self {
            omit_graph_keyword,
            ..self
        }
    }

    pub fn omit_graph_keyword(&self) -> bool {
        self.omit_graph_keyword
    }

    pub fn set_omit_graph_keyword(&mut self, omit_graph_keyword: bool) {
        self.omit_graph_keyword = omit_graph_keyword;
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_options!(TrigWriter, TrigWriterOptions);

impl ObjectWriter<DataSet> for TrigWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, data_set: &DataSet) -> Result<(), Self::Error>
    where
        W: std::io::Write,
    {
        for graph in data_set.graphs() {
            self.write(w, graph)?;
        }
        Ok(())
    }
}

impl DataSetWriter for TrigWriter {}

impl ObjectWriter<Graph> for TrigWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &Graph) -> Result<(), Self::Error>
    where
        W: std::io::prelude::Write,
    {
        if let Some(name) = graph.name() {
            if !self.options().omit_graph_keyword() {
                w.write_all(b"GRAPH ")?;
            }
            w.write_all(name.to_string().as_bytes())?;
            w.write_all(b" ")?;
        }
        w.write_all(b"{\n")?;

        // TODO: call the TurtleWriter

        w.write_all(b"}\n")?;
        Ok(())
    }
}

impl GraphWriter for TrigWriter {}
