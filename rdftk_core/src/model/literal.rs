/*!
 * The `Literal` type used in the object component of a statement. Literal values are always strings,
 * although an optional data type can be provided to allow consumers to convert from string
 * lexical forms.
 *
 * Note that duration values can be provided using `std::time::Duration`, however the
 * [chrono](https://crates.io/crates/chrono) crate's `chrono::Duration` may also be used. This
 * additional dependency also allows for correct formatting of duration lexical forms by converting
 * all standard duration values to chrono durations which support the correct `to_string` form.
 *
 * # Example
 *
 * ```rust
 * use rdftk_core::model::literal::{
 *     LanguageTag, Literal, DataType,
 * };
 * use std::time::Duration;
 *
 * let string_literal = Literal::plain("string value");
 * assert_eq!(string_literal.lexical_form(), "string value");
 * assert_eq!(string_literal.data_type(), None);
 *
 * let string_literal = Literal::with_language(
 *     "string value",
 *     LanguageTag::parse("en-US").unwrap(),
 * );
 * assert_eq!(
 *     string_literal.language().unwrap().to_string(),
 *     "en-US".to_string()
 * );
 * assert_eq!(string_literal.data_type(), None);
 *
 * let typed_string_literal = Literal::from("string value");
 * assert_eq!(typed_string_literal.data_type(), Some(&DataType::String));
 *
 * let long_literal = Literal::with_data_type("212", DataType::Long);
 * assert_eq!(long_literal.data_type(), Some(&DataType::Long));
 *
 * let long_literal = Literal::from(212_u64);
 * assert_eq!(long_literal.lexical_form(), "212");
 *
 * let duration_literal = Literal::from(Duration::from_secs(63542));
 * assert_eq!(duration_literal.lexical_form(), "PT63542S");
 * assert_eq!(duration_literal.data_type(), Some(&DataType::Duration));
 * ```
 *
 * Graphs may have mechanisms to cache commonly used values, or those with significant storage
 * overhead. In such cases they provide a value factory that should be used to construct new values
 * for use in the associated graph. It is possible that all graphs provided by some graph store share
 * a common value factory by store rather than by graph.
 */

use crate::error::Error;
use rdftk_iri::{Iri, Name, QName};
use rdftk_names::{rdf, xsd};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::time::Duration;

#[cfg(feature = "binary_types")]
use base64::{engine::general_purpose::STANDARD, Engine as _};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// Re-export this
pub use language_tags::LanguageTag;

///
/// A subset of known datatypes based on XML Schema, part 2.
///
/// TODO: gDay, gMonth, etc.
/// TODO: .. dateTimeStamp (chrono)
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataType {
    // --------------------------------------------------------------------------------------------
    // Top-Level Data types (https://www.w3.org/TR/xmlschema11-2/#built-in-datatypes)
    // --------------------------------------------------------------------------------------------
    ///
    /// Denotes a literal of type `xsd::anyURI`.
    ///
    Iri,

    ///
    /// Denotes a literal of type `xsd::base64Binary`.
    ///
    Base64Binary,

    ///
    /// Denotes a literal of type `xsd::boolean`.
    ///
    Boolean,

    ///
    /// Denotes a literal of type `xsd::date`.
    ///
    Date,

    ///
    /// Denotes a literal of type `xsd::dateTime`.
    ///
    DateTime,

    ///
    /// Denotes a literal of type `xsd::decimal`.
    ///
    Decimal,

    ///
    /// Denotes a literal of type `xsd::double`.
    ///
    Double,

    ///
    /// Denotes a literal of type `xsd::duration`.
    ///
    /// Note the types `dayTimeDuration`, and `yearMonthDuration` are not provided.
    ///
    Duration,

    ///
    /// Denotes a literal of type `xsd::float`.
    ///
    Float,

    ///
    /// Denotes a literal of type `xsd::hexBinary`.
    ///
    HexBinary,

    ///
    /// Denotes a literal of type `xsd::q_name`.
    ///
    QName,

    ///
    /// Denotes a literal of type `xsd::string`.
    ///
    String,

    ///
    /// Denotes a literal of type `xsd::time`.
    ///
    Time,

    // --------------------------------------------------------------------------------------------
    // Decimal sub-types >> integer ..
    // --------------------------------------------------------------------------------------------
    ///
    /// Denotes a literal of type `xsd::long`.
    ///
    /// `decimal ⇽ integer ⇽ long`
    ///
    Long,

    ///
    /// Denotes a literal of type `xsd::int`.
    ///
    /// `decimal ⇽ integer ⇽ long ⇽ int`
    ///
    Int,

    ///
    /// Denotes a literal of type `xsd::short`.
    ///
    /// `decimal ⇽ integer ⇽ long ⇽ int ⇽ short`
    ///
    Short,

    ///
    /// Denotes a literal of type `xsd::byte`.
    ///
    /// `decimal ⇽ integer ⇽ long ⇽ int ⇽ short ⇽ byte`
    ///
    Byte,

    // --------------------------------------------------------------------------------------------
    // Decimal sub-types >> integer >> nonNegativeInteger ..
    // --------------------------------------------------------------------------------------------
    ///
    /// Denotes a literal of type `xsd::unsignedLong`.
    ///
    UnsignedLong,

    ///
    /// Denotes a literal of type `xsd::unsignedInt`.
    ///
    UnsignedInt,

    ///
    /// Denotes a literal of type `xsd::unsignedShort`.
    ///
    UnsignedShort,

    ///
    /// Denotes a literal of type `xsd::unsignedByte`.
    ///
    UnsignedByte,

    // --------------------------------------------------------------------------------------------
    // String sub-types >> normalizedString >> token ..
    // --------------------------------------------------------------------------------------------
    ///
    /// Denotes a literal of type `xsd::language`.
    ///
    /// `string ⇽ normalizedString ⇽ token ⇽ language`
    ///
    Language,

    ///
    /// Denotes a literal of type `xsd::name`.
    ///
    /// `string ⇽ normalizedString ⇽ token ⇽ Name`
    ///
    Name,

    // --------------------------------------------------------------------------------------------
    // RDF Data Types
    // --------------------------------------------------------------------------------------------
    ///
    /// Denotes an escaped string containing XML content.
    ///
    XmlLiteral,

    // --------------------------------------------------------------------------------------------
    // Everything else
    // --------------------------------------------------------------------------------------------
    ///
    /// Denotes a literal where the type is indicated by the provided `Iri`.
    ///
    Other(Iri),
}

///
/// This trait describes an RDF literal which may be the object of a statement.
///
#[derive(Clone, Debug)]
pub struct Literal {
    lexical_form: String,
    data_type: Option<DataType>,
    language: Option<LanguageTag>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Data Types
// ------------------------------------------------------------------------------------------------

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Iri => xsd::any_uri_qname(),
                Self::Base64Binary => xsd::base64_binary_qname(),
                Self::Boolean => xsd::boolean_qname(),
                Self::Date => xsd::date_qname(),
                Self::DateTime => xsd::date_time_qname(),
                Self::Decimal => xsd::decimal_qname(),
                Self::Double => xsd::double_qname(),
                Self::Duration => xsd::duration_qname(),
                Self::Float => xsd::float_qname(),
                Self::HexBinary => xsd::hex_binary_qname(),
                Self::QName => xsd::q_name_qname(),
                Self::String => xsd::string_qname(),
                Self::Time => xsd::time_qname(),
                Self::Long => xsd::long_qname(),
                Self::Int => xsd::int_qname(),
                Self::Short => xsd::short_qname(),
                Self::Byte => xsd::byte_qname(),
                Self::UnsignedLong => xsd::unsigned_long_qname(),
                Self::UnsignedInt => xsd::unsigned_int_qname(),
                Self::UnsignedShort => xsd::unsigned_short_qname(),
                Self::UnsignedByte => xsd::unsigned_byte_qname(),
                Self::Language => xsd::language_qname(),
                Self::Name => xsd::name_qname(),
                Self::XmlLiteral => rdf::xml_literal_qname(),
                Self::Other(iri) => iri.as_ref(),
            }
        )
    }
}

impl From<Iri> for DataType {
    fn from(iri: Iri) -> Self {
        if &iri == xsd::string() {
            Self::String
        } else if &iri == xsd::q_name() {
            Self::QName
        } else if &iri == xsd::name() {
            Self::Name
        } else if &iri == xsd::any_uri() {
            Self::Iri
        } else if &iri == xsd::boolean() {
            Self::Boolean
        } else if &iri == xsd::float() {
            Self::Float
        } else if &iri == xsd::double() {
            Self::Double
        } else if &iri == xsd::long() {
            Self::Long
        } else if &iri == xsd::int() {
            Self::Int
        } else if &iri == xsd::short() {
            Self::Short
        } else if &iri == xsd::byte() {
            Self::Byte
        } else if &iri == xsd::unsigned_long() {
            Self::UnsignedLong
        } else if &iri == xsd::unsigned_int() {
            Self::UnsignedInt
        } else if &iri == xsd::unsigned_short() {
            Self::UnsignedShort
        } else if &iri == xsd::unsigned_byte() {
            Self::UnsignedByte
        } else if &iri == xsd::duration() {
            Self::Duration
        } else if &iri == rdf::xml_literal() {
            Self::XmlLiteral
        } else {
            Self::Other(iri)
        }
    }
}

impl DataType {
    ///
    /// Return the Iri representing this data type. Primarily these are the XML Schema data types
    /// used for literal values.
    ///
    pub fn as_iri(&self) -> &Iri {
        match self {
            Self::Iri => xsd::any_uri(),
            Self::Base64Binary => xsd::base64_binary(),
            Self::Boolean => xsd::boolean(),
            Self::Date => xsd::date(),
            Self::DateTime => xsd::date_time(),
            Self::Decimal => xsd::decimal(),
            Self::Double => xsd::double(),
            Self::Duration => xsd::duration(),
            Self::Float => xsd::float(),
            Self::HexBinary => xsd::hex_binary(),
            Self::QName => xsd::q_name(),
            Self::String => xsd::string(),
            Self::Time => xsd::time(),
            Self::Long => xsd::long(),
            Self::Int => xsd::int(),
            Self::Short => xsd::short(),
            Self::Byte => xsd::byte(),
            Self::UnsignedLong => xsd::unsigned_long(),
            Self::UnsignedInt => xsd::unsigned_int(),
            Self::UnsignedShort => xsd::unsigned_short(),
            Self::UnsignedByte => xsd::unsigned_byte(),
            Self::Language => xsd::language(),
            Self::Name => xsd::name(),
            Self::XmlLiteral => rdf::xml_literal(),
            Self::Other(iri) => iri,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Literals
// ------------------------------------------------------------------------------------------------

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Iri
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<Iri> for Literal {
    fn from(v: Iri) -> Self {
        Self::with_data_type(v.as_ref(), DataType::Iri)
    }
}

impl From<&Iri> for Literal {
    fn from(v: &Iri) -> Self {
        Self::from(v.clone())
    }
}

impl PartialEq<Iri> for Literal {
    fn eq(&self, other: &Iri) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Iri)
    }
}

impl TryFrom<Literal> for Iri {
    type Error = Error;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value.data_type() {
            Some(dt) => {
                if *dt == DataType::Iri {
                    Ok(Iri::parse(value.lexical_form())?)
                } else {
                    Err(Error::InvalidLiteralTypeCooercion {
                        from_type: dt.to_string(),
                        to_type: DataType::Iri.to_string(),
                    })
                }
            }
            _ => Err(Error::InvalidLiteralTypeCooercion {
                from_type: rdf::plain_literal_qname().to_string(),
                to_type: DataType::Iri.to_string(),
            }),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Base64Binary & HexBinary
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl TryFrom<Literal> for Vec<u8> {
    type Error = Error;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value.data_type() {
            Some(dt) => {
                if *dt == DataType::Base64Binary {
                    Ok(STANDARD.decode(value.lexical_form())?)
                } else if *dt == DataType::HexBinary {
                    Ok(hex_decode(value.lexical_form())?)
                } else {
                    Err(Error::InvalidLiteralTypeCooercion {
                        from_type: dt.to_string(),
                        to_type: "binary".to_string(),
                    })
                }
            }
            _ => Err(Error::InvalidLiteralTypeCooercion {
                from_type: rdf::plain_literal_qname().to_string(),
                to_type: "binary".to_string(),
            }),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Boolean
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<bool> for Literal {
    fn from(v: bool) -> Self {
        Self::with_data_type(v.to_string(), DataType::Boolean)
    }
}

impl PartialEq<bool> for Literal {
    fn eq(&self, other: &bool) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Boolean)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Date
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant DateTime
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Decimal
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[cfg(feature = "decimal_types")]
impl From<rust_decimal::Decimal> for Literal {
    fn from(v: rust_decimal::Decimal) -> Self {
        Self::with_data_type(v.to_string(), DataType::Decimal)
    }
}

#[cfg(feature = "decimal_types")]
impl From<&rust_decimal::Decimal> for Literal {
    fn from(v: &rust_decimal::Decimal) -> Self {
        Self::from(*v)
    }
}

#[cfg(feature = "decimal_types")]
impl PartialEq<rust_decimal::Decimal> for Literal {
    fn eq(&self, other: &rust_decimal::Decimal) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Decimal)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Double
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<f64> for Literal {
    fn from(v: f64) -> Self {
        Self::with_data_type(v.to_string(), DataType::Double)
    }
}

impl PartialEq<f64> for Literal {
    fn eq(&self, other: &f64) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Double)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Duration
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<Duration> for Literal {
    fn from(v: Duration) -> Self {
        let seconds = v.as_secs();
        let nanos = v.subsec_nanos();
        Self::with_data_type(
            if nanos != 0 {
                format!("PT{seconds}.{nanos}S")
            } else {
                format!("PT{seconds}S")
            },
            DataType::Duration,
        )
    }
}

impl From<&Duration> for Literal {
    fn from(v: &Duration) -> Self {
        Self::from(*v)
    }
}

#[cfg(feature = "chrono_types")]
impl From<chrono::Duration> for Literal {
    fn from(v: chrono::Duration) -> Self {
        Self::with_data_type(v.to_string(), DataType::Duration)
    }
}

#[cfg(feature = "chrono_types")]
impl From<&chrono::Duration> for Literal {
    fn from(v: &chrono::Duration) -> Self {
        Self::from(*v)
    }
}

impl PartialEq<Duration> for Literal {
    fn eq(&self, other: &Duration) -> bool {
        *self.lexical_form() == other.as_nanos().to_string()
            && self.data_type() == Some(&DataType::Duration)
    }
}

#[cfg(feature = "chrono_types")]
impl PartialEq<chrono::Duration> for Literal {
    fn eq(&self, other: &chrono::Duration) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Duration)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Float
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<f32> for Literal {
    fn from(v: f32) -> Self {
        Self::with_data_type(v.to_string(), DataType::Float)
    }
}

impl PartialEq<f32> for Literal {
    fn eq(&self, other: &f32) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Float)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant QName
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<QName> for Literal {
    fn from(v: QName) -> Self {
        Self::with_data_type(v, DataType::QName)
    }
}

impl From<&QName> for Literal {
    fn from(v: &QName) -> Self {
        Self::from(v.clone())
    }
}

impl PartialEq<QName> for Literal {
    fn eq(&self, other: &QName) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::QName)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant String
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<String> for Literal {
    fn from(v: String) -> Self {
        Self::with_data_type(v, DataType::String)
    }
}

impl From<&str> for Literal {
    fn from(v: &str) -> Self {
        Self::with_data_type(v, DataType::String)
    }
}

impl PartialEq<String> for Literal {
    fn eq(&self, other: &String) -> bool {
        self.lexical_form() == other
            && (self.data_type().is_none() || self.data_type() == Some(&DataType::String))
    }
}

impl PartialEq<str> for Literal {
    fn eq(&self, other: &str) -> bool {
        self.lexical_form() == other
            && (self.data_type().is_none() || self.data_type() == Some(&DataType::String))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Time
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Long
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<i64> for Literal {
    fn from(v: i64) -> Self {
        Self::with_data_type(v.to_string(), DataType::Long)
    }
}

impl PartialEq<i64> for Literal {
    fn eq(&self, other: &i64) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Long)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Int
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<i32> for Literal {
    fn from(v: i32) -> Self {
        Self::with_data_type(v.to_string(), DataType::Int)
    }
}

impl PartialEq<i32> for Literal {
    fn eq(&self, other: &i32) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Int)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Short
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<i16> for Literal {
    fn from(v: i16) -> Self {
        Self::with_data_type(v.to_string(), DataType::Short)
    }
}

impl PartialEq<i16> for Literal {
    fn eq(&self, other: &i16) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Short)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Byte
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<i8> for Literal {
    fn from(v: i8) -> Self {
        Self::with_data_type(v.to_string(), DataType::Byte)
    }
}

impl PartialEq<i8> for Literal {
    fn eq(&self, other: &i8) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Byte)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant UnsignedLong
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<u64> for Literal {
    fn from(v: u64) -> Self {
        Self::with_data_type(v.to_string(), DataType::UnsignedLong)
    }
}

impl PartialEq<u64> for Literal {
    fn eq(&self, other: &u64) -> bool {
        *self.lexical_form() == other.to_string()
            && self.data_type() == Some(&DataType::UnsignedLong)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant UnsignedInt
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<u32> for Literal {
    fn from(v: u32) -> Self {
        Self::with_data_type(v.to_string(), DataType::UnsignedInt)
    }
}

impl PartialEq<u32> for Literal {
    fn eq(&self, other: &u32) -> bool {
        *self.lexical_form() == other.to_string()
            && self.data_type() == Some(&DataType::UnsignedInt)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant UnsignedShort
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<u16> for Literal {
    fn from(v: u16) -> Self {
        Self::with_data_type(v.to_string(), DataType::UnsignedShort)
    }
}

impl PartialEq<u16> for Literal {
    fn eq(&self, other: &u16) -> bool {
        *self.lexical_form() == other.to_string()
            && self.data_type() == Some(&DataType::UnsignedShort)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant UnsignedByte
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<u8> for Literal {
    fn from(v: u8) -> Self {
        Self::with_data_type(v.to_string(), DataType::UnsignedByte)
    }
}

impl PartialEq<u8> for Literal {
    fn eq(&self, other: &u8) -> bool {
        *self.lexical_form() == other.to_string()
            && self.data_type() == Some(&DataType::UnsignedByte)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Language
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<LanguageTag> for Literal {
    fn from(v: LanguageTag) -> Self {
        Self::with_data_type(v.to_string(), DataType::Language)
    }
}

impl PartialEq<LanguageTag> for Literal {
    fn eq(&self, other: &LanguageTag) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Language)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Name
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl From<Name> for Literal {
    fn from(v: Name) -> Self {
        Self::with_data_type(v, DataType::Name)
    }
}

impl From<&Name> for Literal {
    fn from(v: &Name) -> Self {
        Self::from(v.clone())
    }
}

impl PartialEq<Name> for Literal {
    fn eq(&self, other: &Name) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Name)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant XmlLiteral
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Variant Other
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// ------------------------------------------------------------------------------------------------

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.lexical_form == other.lexical_form
            && self.data_type == other.data_type
            && self.language == other.language
    }
}

impl Eq for Literal {}

// ------------------------------------------------------------------------------------------------

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.data_type() {
            Some(DataType::Iri) => write!(f, "<{}>", self.lexical_form()),
            Some(DataType::Boolean)
            | Some(DataType::Long)
            | Some(DataType::Int)
            | Some(DataType::Short)
            | Some(DataType::Byte)
            | Some(DataType::UnsignedLong)
            | Some(DataType::UnsignedInt)
            | Some(DataType::UnsignedShort)
            | Some(DataType::UnsignedByte)
            | Some(DataType::Float)
            | Some(DataType::Double)
            | Some(DataType::Decimal) => write!(f, "{}", self.lexical_form()),
            _ => {
                write!(
                    f,
                    "\"{}\"{}",
                    self.lexical_form(),
                    match (self.data_type(), self.language()) {
                        (Some(data_type), None) => format!("^^<{}>", data_type.as_iri()),
                        (None, Some(language)) => format!("@{}", language),
                        _ => String::new(),
                    }
                )
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Hash for Literal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lexical_form.hash(state);
        self.data_type.hash(state);
        self.language.hash(state);
    }
}

// ------------------------------------------------------------------------------------------------

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Literal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.data_type, &other.data_type) {
            (Some(this), Some(other)) => match this.cmp(other) {
                Ordering::Equal => {}
                ord => return ord,
            },
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => {}
        }

        match (&self.language, &other.language) {
            (Some(this), Some(other)) => match this.to_string().cmp(&other.to_string()) {
                Ordering::Equal => {}
                ord => return ord,
            },
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => {}
        }

        self.lexical_form.cmp(&other.lexical_form)
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Literal {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    ///
    /// Returns a cached *plain* literal value with the provided string.
    ///
    pub fn plain<S>(v: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: escape_string(v),
            data_type: None,
            language: None,
        }
    }

    ///
    /// Returns a cached literal value with the provided string and language.
    ///
    pub fn with_language<S>(v: S, lang: LanguageTag) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: escape_string(v),
            data_type: None,
            language: Some(lang),
        }
    }

    ///
    /// Returns a cached literal value with the provided string and data type.
    ///
    pub fn with_data_type<S>(v: S, data_type: DataType) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: escape_string(v),
            data_type: Some(data_type),
            language: None,
        }
    }

    ///
    /// Returns a cached literal value with the provided string and data type IRI.
    ///
    pub fn with_data_type_iri<S>(v: S, data_type: Iri) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: escape_string(v),
            data_type: Some(DataType::Other(data_type)),
            language: None,
        }
    }

    pub fn hex_encoded(v: &[u8]) -> Self {
        Self::with_data_type(hex_encode(v), DataType::HexBinary)
    }

    #[cfg(feature = "binary_types")]
    pub fn base64_encoded(v: &[u8]) -> Self {
        Self::with_data_type(STANDARD.encode(v), DataType::HexBinary)
    }

    // --------------------------------------------------------------------------------------------
    // Accessors
    // --------------------------------------------------------------------------------------------

    ///
    /// Return the lexical form of this literal.
    ///
    pub fn lexical_form(&self) -> &String {
        &self.lexical_form
    }

    ///
    /// Returns `true` if this literal has a specified data type, else `false`.
    ///
    pub fn has_data_type(&self) -> bool {
        self.data_type.is_some()
    }

    ///
    /// Returns this literal's data type, if present.
    ///
    pub fn data_type(&self) -> Option<&DataType> {
        self.data_type.as_ref()
    }

    ///
    /// Returns `true` if this literal has a specified language tag, else `false`.
    ///
    pub fn has_language(&self) -> bool {
        self.language().is_some()
    }

    ///
    /// Return this literal's language tag, if present.
    ///
    pub fn language(&self) -> Option<&LanguageTag> {
        self.language.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn escape_string<S>(value: S) -> String
where
    S: Into<String>,
{
    let formatted = format!("{:?}", value.into());
    formatted[1..formatted.len() - 1].to_string()
}

fn hex_encode(value: &[u8]) -> String {
    if !value.is_empty() {
        let mut buffer = String::with_capacity(value.len() * 3);
        let last = value.len() - 1;
        value.iter().enumerate().for_each(|(i, b)| {
            buffer.push_str(&format!("{:02X}{}", b, if i == last { "" } else { " " }))
        });
        buffer
    } else {
        String::default()
    }
}

fn hex_decode<S>(value: S) -> Result<Vec<u8>, Error>
where
    S: AsRef<str>,
{
    let value = value.as_ref();
    let mut buffer = Vec::with_capacity((value.len() + 1) / 3);
    for (index, hex_pair) in value.split(" ").enumerate() {
        if hex_pair.len() == 2 {
            let byte = u8::from_str_radix(hex_pair, 16).map_err(|_| Error::HexDecoder {
                value: hex_pair.to_string(),
                index,
            })?;
            buffer.push(byte);
        } else {
            return Err(Error::HexDecoder {
                value: hex_pair.to_string(),
                index,
            });
        }
    }
    Ok(buffer)
}
