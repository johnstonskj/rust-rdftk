use crate::common::parser::parse_ntriple_doc;
use crate::make_factory_options;
use objio::{impl_has_options, HasOptions, ObjectReader};
use rdftk_core::error::Error;
use rdftk_core::model::graph::{GraphFactoryRef, GraphRef};
use rdftk_core::simple::graph_factory;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

make_factory_options!(NTripleReaderOptions, GraphFactoryRef, graph_factory);

#[derive(Debug, Default)]
pub struct NTripleReader {
    options: NTripleReaderOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_options!(NTripleReader, NTripleReaderOptions);

impl ObjectReader<GraphRef> for NTripleReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<GraphRef, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        let factory = self.options().factory().clone();
        parse_ntriple_doc(buffer, factory)
    }
}
