use crate::common::parser::parse_trig_doc;
use objio::ObjectReader;
use rdftk_core::error::Error;
use rdftk_core::model::data_set::DataSet;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct TrigReader {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectReader<DataSet> for TrigReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<DataSet, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        parse_trig_doc(buffer)
    }
}
