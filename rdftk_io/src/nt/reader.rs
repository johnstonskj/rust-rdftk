use crate::common::parser::parse_ntriple_doc;
use objio::ObjectReader;
use rdftk_core::error::Error;
use rdftk_core::model::graph::Graph;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct NTripleReader {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectReader<Graph> for NTripleReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<Graph, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        parse_ntriple_doc(buffer)
    }
}
