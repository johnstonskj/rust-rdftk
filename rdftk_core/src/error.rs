/*!
Provides the crate's `Error` and `Result` types as well as helper functions.

 */

use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait ErrorSource: std::error::Error {
    fn as_error_source(&self) -> &(dyn std::error::Error + 'static);
}

///
/// The Error type for this crate.
///
#[derive(Debug)]
pub enum Error {
    ///
    /// This signals a parser *tokenizer* error.
    ///
    Tokenizer {
        representation: String,
        source: Box<dyn std::error::Error>,
    },
    ///
    /// This signals a parser error where some expected value was **not** found.
    ///
    ParserExpected {
        rule_fn: String,
        expecting: String,
    },
    ///
    /// This signals a parser error where some unexpected value **was** not found.
    ///
    ParserUnexpected {
        rule_fn: String,
        given: String,
        expecting: Vec<String>,
    },
    ParserUnreachable {
        rule_fn: String,
        given: String,
    },
    ///
    /// The String value provided is not a valid value for it's type.
    ///
    InvalidFromStr {
        value: String,
        name: String,
    },
    ///
    /// Could not coerce from `from_type` to `to_type`.
    ///
    InvalidLiteralTypeCooercion {
        from_type: String,
        to_type: String,
    },
    ///
    /// Could not decode a supposedly hex-encoded string.
    ///
    HexDecoder {
        value: String,
        index: usize,
    },

    ///
    /// The String value provided is not a valid Blank Node name.
    ///
    InvalidBlankNodeName {
        name: String,
    },
    ///
    /// A QName may not have an empty name part.
    ///
    EmptyQName,
    ///
    /// The String value provided is not a valid QName.
    ///
    InvalidQName {
        name: String,
    },
    ///
    /// Values from these different providers cannot be combined.
    ///
    ProviderMismatch {
        lhs: String,
        rhs: String,
    },
    ///
    /// The match combination is not valid.
    ///
    InvalidMatch,
    ///
    /// An Absolute IRI was expected.
    ///
    AbsoluteIriExpected {
        uri: String,
    },

    ///
    ///Some model element was in an invalid state for the requested operation.
    ///
    InvalidState,
    ///
    /// Statements as objects, from RDF*, are not supported by this representation.
    ///
    RdfStarNotSupported {
        representation: String,
    },
    ///
    /// Cited model.formulae, from N3, are not supported by this representation.
    ///
    FormulaeNotSupported {
        representation: String,
    },
    ///
    /// Could not read or write query results in this representation.
    ///
    QueryResultsFormat {
        representation: String,
    },

    ///
    /// An error occurred borrowing from a standard cell type.
    ///
    Borrow(::std::cell::BorrowError),
    ///
    /// An error in the standard I/O library.
    ///
    Io(::std::io::Error),
    ///
    /// An error parsing IRI strings.
    ///
    Iri(::rdftk_iri::Error),
    ///
    /// An eror parsing language-tag strings.
    ///
    LanguageTag(::language_tags::ParseError),
    ///
    /// An error parsing Name strings.
    ///
    Name(::rdftk_iri::NameParseError),
    ///
    /// An error occurred converting to UTF-8 text.
    ///
    Utf8(::std::string::FromUtf8Error),

    ///
    /// An error occurred decoding base-64 data.
    ///
    #[cfg(feature = "binary_types")]
    Base64Decoder(::base64::DecodeError),
    ///
    /// An unknown error occurred.
    ///
    Unknown(String),
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

#[inline(always)]
pub fn unknown_error<E>(source: E) -> Error
where
    E: ::std::error::Error,
{
    Error::Unknown(source.to_string())
}

#[inline(always)]
pub fn unknown_error_from<S>(message: S) -> Error
where
    S: Into<String>,
{
    Error::Unknown(message.into())
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
                Self::Tokenizer { representation, source } => format!("The tokenizer for {representation} generated an error: {source}"),
                Self::ParserExpected { rule_fn, expecting } => format!("Parser was expecting `{expecting}` in function `{rule_fn}`."),
                Self::ParserUnexpected { rule_fn, given, expecting } => format!("Parser was not expecting `{given}` in function `{rule_fn}`; expecting {expecting:?}."),
                Self::ParserUnreachable { rule_fn, given } => format!("Parser should not have reached `{given}` in function `{rule_fn}`."),
                Self::InvalidFromStr { value, name } => format!(
                    "The String value `{value}` is not a valid value for it's type: '{name}'."
                ),
                Self::InvalidBlankNodeName { name } =>
                    format!("The String value `{name}` is not a valid Blank Node name."),
                Self::InvalidLiteralTypeCooercion { from_type, to_type } =>
                    format!("Not possible to coerce a literal from `{from_type}` into `{to_type}`."),
                Self::HexDecoder { value, index } => format!("Could not decode a hex-encoded string, bad value `{value}` at index {index}"),
                Self::EmptyQName => "A QName may not have an empty name part.".to_string(),
                Self::InvalidQName { name } =>
                    format!("The String value `{name}` is not a valid QName."),
                Self::ProviderMismatch { lhs, rhs } => format!(
                    "Values from these different providers cannot be combined ({lhs:?}, {rhs:?})."
                ),
                Self::InvalidMatch => "The match combination is not valid.".to_string(),
                Self::AbsoluteIriExpected { uri } =>
                    format!("An Absolute IRI was expected at, not '{uri}'."),
                Self::InvalidState =>
                    "Some model element was in an invalid state for the requested operation.".to_string(),
                Self::RdfStarNotSupported { representation } => format!("Statements as objects, from RDF*, are not supported by the {representation:?} representation."),
                Self::FormulaeNotSupported { representation } => format!("Cited model.formulae, from N3, are not supported by the {representation:?} representation."),
                Self::QueryResultsFormat { representation } => format!("Could not read or write query results in the {representation:?} representation."),
                Self::Borrow(source) => format!("A cell borrow error occurred; source: {source}"),
                Self::Io(source) => format!("An I/O error occurred; source: {source}"),
                Self::Iri(source) =>format!("An error occurred parsing an IRI; source: {source}"),
                Self::LanguageTag(source) => format!("An error occurred parsing a language tag; source: {source}"),
                Self::Name(source) => format!("An error occurred parsing a name; source: {source}"),
                Self::Utf8(source) => format!("An error occurred parsing a UTF-8 string; source: {source}"),
                #[cfg(feature = "binary_types")]
                Self::Base64Decoder(source) => format!("An error occurred parsing a base64 encoded string; source: {source}"),
                Self::Unknown(source) => format!("Unknown error: {source}"),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            //Self::Tokenizer { source } => Some(source),
            Self::Borrow(source) => Some(source),
            Self::Io(source) => Some(source),
            Self::Iri(source) => Some(source),
            Self::LanguageTag(source) => Some(source),
            Self::Name(source) => Some(source),
            Self::Utf8(source) => Some(source),
            Self::Base64Decoder(source) => Some(source),
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

#[cfg(feature = "binary_types")]
impl From<::base64::DecodeError> for Error {
    fn from(source: ::base64::DecodeError) -> Self {
        Self::Base64Decoder(source)
    }
}

impl From<String> for Error {
    fn from(source: String) -> Self {
        Self::Unknown(source)
    }
}

#[allow(clippy::from_over_into)]
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
