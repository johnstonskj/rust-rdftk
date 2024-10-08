use crate::common::parser::parse_nquad_doc;
use objio::ObjectReader;
use rdftk_core::{error::Error, model::data_set::DataSet};
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct NQuadReader {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectReader<DataSet> for NQuadReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<DataSet, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        parse_nquad_doc(buffer)
    }
}
