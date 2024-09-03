/*!
One-line description.

See [SPARQL 1.1 Query Results CSV and TSV Formats](https://www.w3.org/TR/sparql11-results-csv-tsv/).

# Example

 */

use crate::results::Results;
use csv::{ReaderBuilder, WriterBuilder};
use rdftk_core::error::{Error, ErrorKind};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const REPRESENTATION_LABEL: &str = "CSV/TSV";

#[derive(Clone, Debug, Default)]
pub struct Options {
    tab_separated: bool,
    skip_headers: bool,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn from_file<P>(path: P, opt: Options) -> Result<Results, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    from_reader(reader, opt)
}

pub fn from_reader<R>(reader: R, opt: Options) -> Result<Results, Error>
where
    R: Read,
{
    let mut reader = ReaderBuilder::new()
        .delimiter(if opt.tab_separated { b'\t' } else { b',' })
        .has_headers(!opt.skip_headers || opt.tab_separated)
        .from_reader(reader);

    if !opt.skip_headers || opt.tab_separated {
        let headers = reader.headers();
    }

    for row in reader.records() {}

    todo!()
}

pub fn from_str<S>(s: S, opt: Options) -> Result<Results, Error>
where
    S: AsRef<str>,
{
    from_reader(s.as_ref().as_bytes(), opt)
}

pub fn from_slice(bytes: &[u8], opt: Options) -> Result<Results, Error> {
    let reader = BufReader::new(bytes);
    from_reader(reader, opt)
}

// ------------------------------------------------------------------------------------------------

pub fn to_string(results: &Results, opt: Options) -> Result<String, Error> {
    let mut writer = Vec::new();
    to_writer(results, &mut writer, opt)?;
    Ok(String::from_utf8(writer)?)
}

pub fn to_writer<W>(results: &Results, writer: &mut W, opt: Options) -> Result<(), Error>
where
    W: Write,
{
    let mut writer = WriterBuilder::new()
        .delimiter(if opt.tab_separated { b'\t' } else { b',' })
        .from_writer(writer);

    if !opt.skip_headers || opt.tab_separated {
        writer
            .write_record(results.columns())
            .map_err(map_csv_error)?;
    }

    for row in results.rows() {
        writer
            .write_record(row.as_ref().iter().map(|v| v.to_string()))
            .map_err(map_csv_error)?;
    }

    Ok(())
}

pub fn to_file<P>(results: &Results, path: P, opt: Options) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    to_writer(results, &mut file, opt)
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn map_csv_error(e: csv::Error) -> Error {
    Error::with_chain(
        e,
        ErrorKind::QueryResultsFormat(REPRESENTATION_LABEL.to_string()),
    )
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
