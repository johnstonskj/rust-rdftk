/*!
One-line description.

More detailed description, with

# Example

 */

use crate::results::Results;
use rdftk_core::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn from_file<P>(path: P) -> Result<Results, Error>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    from_reader(reader)
}

pub fn from_reader<R>(reader: R) -> Result<Results, Error>
where
    R: Read,
{
    todo!()
}

pub fn from_str<S>(s: S) -> Result<Results, Error>
where
    S: AsRef<str>,
{
    from_reader(s.as_ref().as_bytes())
}

pub fn from_slice(bytes: &[u8]) -> Result<Results, Error> {
    let reader = BufReader::new(bytes);
    from_reader(reader)
}

// ------------------------------------------------------------------------------------------------

pub fn to_string<T>(results: &T) -> Result<String, Error>
where
    T: Into<Results>,
{
    let mut writer = BufWriter::new(Vec::new());
    to_writer(results, &mut writer)?;
    let bytes = writer.into_inner().unwrap(); // TODO: error wrapper
    Ok(String::from_utf8(bytes)?)
}

pub fn to_writer<T, W>(results: &T, writer: &mut W) -> Result<(), Error>
where
    T: Into<Results>,
    W: Write,
{
    todo!()
}

pub fn to_file<T, P>(results: &T, path: P) -> Result<(), Error>
where
    T: Into<Results>,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut writer = BufWriter::new(file);
    to_writer(results, &mut writer)
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

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
