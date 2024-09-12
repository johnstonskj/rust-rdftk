/*!
Provides the crate's `Error` and `Result` types as well as helper functions.

 */

use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Error type for this crate.
///
#[derive(Debug)]
pub enum Error {
    /// The String value provided is not a valid value for it's type.
    InvalidFromStr {
        value: String,
        name: String,
    },
    /// The String value provided is not a valid Blank Node name.
    InvalidBlankNodeName {
        name: String,
    },
    /// A QName may not have an empty name part.
    EmptyQName,
    /// The String value provided is not a valid QName.
    InvalidQName {
        name: String,
    },
    /// Values from these different providers cannot be combined.
    ProviderMismatch {
        lhs: String,
        rhs: String,
    },
    /// The match combination is not valid.
    InvalidMatch,
    /// An Absolute IRI was expected.
    AbsoluteIriExpected {
        uri: String,
    },
    ///A failure occurred reading or writing a graph.
    ReadWrite {
        representation: String,
        source: Option<Box<dyn std::error::Error>>,
    },
    ///Some model element was in an invalid state for the requested operation.
    InvalidState,
    /// Statements as objects, from RDF*, are not supported by this representation.
    RdfStarNotSupported {
        representation: String,
    },
    /// Cited model.formulae, from N3, are not supported by this representation.
    FormulaeNotSupported {
        representation: String,
    },
    /// Could not read or write query results in this representation.
    QueryResultsFormat {
        representation: String,
    },

    Borrow(::std::cell::BorrowError),
    /// An error in the standard I/O library.
    Io(::std::io::Error),
    /// An error parsing IRI strings.
    Iri(::rdftk_iri::Error),
    /// An eror parsing language-tag strings.
    LanguageTag(::language_tags::ParseError),
    /// An error parsing Name strings.
    Name(::rdftk_iri::NameParseError),
    /// An error occured converting to UTF-8 text.
    Utf8(::std::string::FromUtf8Error),
}

///
/// A Result type that specifically uses this crate's Error.
///
pub type Result<T> = std::result::Result<T, Error>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Create Error object.
///
#[inline(always)]
pub fn invalid_from_str_error<S1, S2>(value: S1, type_name: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    Error::InvalidFromStr {
        value: value.into(),
        name: type_name.into(),
    }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn invalid_blank_node_name_error<S>(name: S) -> Error
where
    S: Into<String>,
{
    Error::InvalidBlankNodeName { name: name.into() }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn empty_qname_error() -> Error {
    Error::EmptyQName
}

///
/// Create Error object.
///
#[inline(always)]
pub fn invalid_qname_error<S>(name: S) -> Error
where
    S: Into<String>,
{
    Error::InvalidQName { name: name.into() }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn provider_mismatch_error<S1, S2>(lhs: S1, rhs: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    Error::ProviderMismatch {
        lhs: lhs.into(),
        rhs: rhs.into(),
    }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn invalid_match_error() -> Error {
    Error::InvalidMatch
}

///
/// Create Error object.
///
#[inline(always)]
pub fn absolute_iri_expected_error<S>(uri: S) -> Error
where
    S: Into<String>,
{
    Error::AbsoluteIriExpected { uri: uri.into() }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn read_write_error<S>(representation: S) -> Error
where
    S: Into<String>,
{
    Error::ReadWrite {
        representation: representation.into(),
        source: None,
    }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn read_write_error_with<S, E>(representation: S, source: E) -> Error
where
    S: Into<String>,
    E: std::error::Error + 'static,
{
    Error::ReadWrite {
        representation: representation.into(),
        source: Some(Box::new(source)),
    }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn invalid_state_error() -> Error {
    Error::InvalidState
}

///
/// Create Error object.
///
#[inline(always)]
pub fn rdf_star_not_supported_error<S>(representation: S) -> Error
where
    S: Into<String>,
{
    Error::RdfStarNotSupported {
        representation: representation.into(),
    }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn formulae_not_supported_error<S>(representation: S) -> Error
where
    S: Into<String>,
{
    Error::FormulaeNotSupported {
        representation: representation.into(),
    }
}

///
/// Create Error object.
///
#[inline(always)]
pub fn query_results_format_error<S>(representation: S) -> Error
where
    S: Into<String>,
{
    Error::QueryResultsFormat {
        representation: representation.into(),
    }
}

// ------------------------------------------------------------------------------------------------

///
/// Display an error trace to stdout.
///
#[inline(always)]
pub fn print_trace(e: &dyn std::error::Error) {
    println!("{}", error_trace(e));
}

///
/// Display an error trace to stderr.
///
#[inline(always)]
pub fn eprint_trace(e: &dyn std::error::Error) {
    eprintln!("{}", error_trace(e));
}

///
/// Convert an error into a trace string.
///
#[inline(always)]
pub fn error_trace(e: &dyn std::error::Error) -> String {
    trace_one(e, 1)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidFromStr { value, name } => format!(
                    "The String value `{value}` is not a valid value for it's type: '{name}'."
                ),
                Error::InvalidBlankNodeName { name } =>
                    format!("The String value `{name}` is not a valid Blank Node name."),
                Error::EmptyQName => "A QName may not have an empty name part.".to_string(),
                Error::InvalidQName { name } =>
                    format!("The String value `{name}` is not a valid QName."),
                Error::ProviderMismatch { lhs, rhs } => format!(
                    "Values from these different providers cannot be combined ({lhs:?}, {rhs:?})."
                ),
                Error::InvalidMatch => "The match combination is not valid.".to_string(),
                Error::AbsoluteIriExpected { uri } =>
                    format!("An Absolute IRI was expected at, not '{uri}'."),
                Error::ReadWrite { representation, source } =>
                    format!("A failure occurred reading or writing a graph, for representation: {representation:?}.{}", match source {
                        Some(source) => format!("Source: {source}"),
    None => String::default(),
                    }),
                Error::InvalidState =>
                    "Some model element was in an invalid state for the requested operation.".to_string(),
                Error::RdfStarNotSupported { representation } => format!("Statements as objects, from RDF*, are not supported by the {representation:?} representation."),
                Error::FormulaeNotSupported { representation } => format!("Cited model.formulae, from N3, are not supported by the {representation:?} representation."),
                Error::QueryResultsFormat { representation } => format!("Could not read or write query results in the {representation:?} representation."),
                Error::Borrow(source) => format!("A cell borrow error occurred; source: {source}"),
                Error::Io(source) => format!("An I/O error occurred; source: {source}"),
                Error::Iri(source) =>format!("An error occurred parsing an IRI; source: {source}"),
                Error::LanguageTag(source) => format!("An error occurred parsing a language tag; source: {source}"),
                Error::Name(source) => format!("An error occurred parsing a name; source: {source}"),
                Error::Utf8(source) => format!("An error occurred parsing a UTF-8 string; source: {source}"),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadWrite {
                representation: _,
                source,
            } => source.as_ref().map(|e| e.as_ref()),
            Self::Borrow(source) => Some(source),
            Self::Io(source) => Some(source),
            Self::Iri(source) => Some(source),
            Self::LanguageTag(source) => Some(source),
            Self::Name(source) => Some(source),
            Self::Utf8(source) => Some(source),
            _ => None,
        }
    }
}

impl From<::std::cell::BorrowError> for Error {
    fn from(source: ::std::cell::BorrowError) -> Self {
        Self::Borrow(source)
    }
}

impl From<::std::io::Error> for Error {
    fn from(source: ::std::io::Error) -> Self {
        Self::Io(source)
    }
}

impl From<::rdftk_iri::Error> for Error {
    fn from(source: ::rdftk_iri::Error) -> Self {
        Self::Iri(source)
    }
}

impl From<::language_tags::ParseError> for Error {
    fn from(source: ::language_tags::ParseError) -> Self {
        Self::LanguageTag(source)
    }
}

impl From<::rdftk_iri::NameParseError> for Error {
    fn from(source: ::rdftk_iri::NameParseError) -> Self {
        Self::Name(source)
    }
}

impl From<::std::string::FromUtf8Error> for Error {
    fn from(source: ::std::string::FromUtf8Error) -> Self {
        Self::Utf8(source)
    }
}

impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn trace_one(e: &dyn std::error::Error, count: i32) -> String {
    use std::fmt::Write;

    let mut trace = String::new();

    writeln!(&mut trace, "{}. {}", count, e).expect("Failed to write message.");

    if let Some(source) = e.source() {
        write!(&mut trace, "{}", trace_one(source, count + 1)).expect("Failed to write source.");
    }

    writeln!(&mut trace).expect("Failed to write line break");
    trace
}
