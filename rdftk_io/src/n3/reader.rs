use crate::common::parser::parse_n3_doc;
use objio::ObjectReader;
use rdftk_core::error::Error;
use rdftk_core::model::graph::Graph;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct N3Reader {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectReader<Graph> for N3Reader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<Graph, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        parse_n3_doc(buffer)
    }
}
