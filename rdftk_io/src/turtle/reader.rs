use crate::common::parser::parse_turtle_doc;
use objio::ObjectReader;
use rdftk_core::error::Error;
use rdftk_core::model::graph::Graph;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct TurtleReader {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ObjectReader<Graph> for TurtleReader {
    type Error = Error;

    fn read<R>(&self, r: &mut R) -> Result<Graph, Self::Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        r.read_to_string(&mut buffer)?;
        parse_turtle_doc(buffer)
    }
}
