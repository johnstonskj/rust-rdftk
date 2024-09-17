use crate::common::parser::parse_trig_doc;
use crate::make_factory_options;
use objio::{impl_has_options, HasOptions, ObjectReader};
use rdftk_core::error::Error;
use rdftk_core::model::data_set::{DataSetFactoryRef, DataSetRef};
use rdftk_core::simple::data_set_factory;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

make_factory_options!(TrigReaderOptions, DataSetFactoryRef, data_set_factory);

#[derive(Debug, Default)]
pub struct TrigReader {
    options: TrigReaderOptions,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_has_options!(TrigReader, TrigReaderOptions);

impl ObjectReader<DataSetRef> for TrigReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<DataSetRef, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        let factory = self.options().factory().clone();
        parse_trig_doc(buffer, factory)
    }
}
