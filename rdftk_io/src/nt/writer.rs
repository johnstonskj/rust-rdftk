use crate::GraphWriter;
use objio::{HasOptions, ObjectWriter};
use rdftk_core::error::Error;
use rdftk_core::model::graph::Graph;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NTripleWriterOptions {
    force_string_literals: bool,
}

///
/// This struct implements the `ObjectWriter` trait for graphs and will write out a serialized
/// form of the entire graph.
///
#[derive(Debug, Default)]
pub struct NTripleWriter {
    options: NTripleWriterOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectWriter<Graph> for NTripleWriter {
    type Error = Error;

    fn write<W>(&self, w: &mut W, graph: &Graph) -> Result<(), Self::Error>
    where
        W: Write,
    {
        let simple_graph = graph.simplify()?;
        for subject in simple_graph.subjects() {
            for predicate in simple_graph.predicates_for(subject) {
                for object in simple_graph.objects_for(subject, predicate) {
                    if self.options().force_string_literals {
                        writeln!(w, "{subject} <{predicate}> {object:#} .")?;
                    } else {
                        writeln!(w, "{subject} <{predicate}> {object} .")?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl GraphWriter for NTripleWriter {}

impl HasOptions<NTripleWriterOptions> for NTripleWriter {
    fn set_options(&mut self, options: NTripleWriterOptions) {
        self.options = options;
    }

    fn options(&self) -> &NTripleWriterOptions {
        &self.options
    }
}

// ------------------------------------------------------------------------------------------------

impl NTripleWriterOptions {
    pub fn force_string_literals(self, flag: bool) -> Self {
        let mut self_mut = self;
        self_mut.force_string_literals = flag;
        self_mut
    }
}
