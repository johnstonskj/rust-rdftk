use crate::common::parser::parse_n3_doc;
use crate::make_factory_options;
use objio::{impl_has_options, HasOptions, ObjectReader};
use rdftk_core::error::Error;
use rdftk_core::model::graph::{GraphFactoryRef, GraphRef};
use rdftk_core::simple::graph_factory;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

make_factory_options!(N3ReaderOptions, GraphFactoryRef, graph_factory);

#[derive(Debug, Default)]
pub struct N3Reader {
    options: N3ReaderOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_options!(N3Reader, N3ReaderOptions);

impl ObjectReader<GraphRef> for N3Reader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<GraphRef, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        let factory = self.options().factory().clone();
        parse_n3_doc(buffer, factory)
    }
}
