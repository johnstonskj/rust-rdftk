use crate::common::parser::parse_nquad_doc;
use crate::make_factory_options;
use objio::{impl_has_options, HasOptions, ObjectReader};
use rdftk_core::simple::data_set_factory;
use rdftk_core::{
    error::Error,
    model::data_set::{DataSetFactoryRef, DataSetRef},
};
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

make_factory_options!(NQuadReaderOptions, DataSetFactoryRef, data_set_factory);

#[derive(Debug, Default)]
pub struct NQuadReader {
    options: NQuadReaderOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_options!(NQuadReader, NQuadReaderOptions);

impl ObjectReader<DataSetRef> for NQuadReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<DataSetRef, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        let factory = self.options().factory().clone();
        parse_nquad_doc(buffer, factory)
    }
}
