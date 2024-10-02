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
* use rdftk_core::model::literal::{Literal, LiteralFactory, DataType};
* use rdftk_core::simple::literal::SimpleLiteralFactory;
* use std::time::Duration;
*
* let factory = SimpleLiteralFactory::default();
*
* let string_literal = factory.literal("string value");
* assert_eq!(string_literal.lexical_form(), "string value");
* assert_eq!(string_literal.data_type(), None);
*
* let string_literal = factory.with_language_str("string value", "en-US").unwrap();
* assert_eq!(string_literal.language().unwrap().to_string(), "en-US".to_string());
* assert_eq!(string_literal.data_type(), None);
*
* let typed_string_literal = factory.string("string value");
* assert_eq!(typed_string_literal.data_type(), Some(&DataType::String));
*
* let long_literal = factory.with_data_type("212", DataType::Long);
* assert_eq!(long_literal.data_type(), Some(&DataType::Long));
*
* let long_literal = factory.unsigned_long(212);
* assert_eq!(long_literal.lexical_form(), "212");
*
* let duration_literal = factory.duration(Duration::from_secs(63542));
* assert_eq!(duration_literal.lexical_form(), "PT63542S");
* assert_eq!(duration_literal.data_type(), Some(&DataType::Duration));
* ```
*
* Graphs may have mechanisms to cache commonly used values, or those with significant storage
* overhead. In such cases they provide a value factory that should be used to construct new values
* for use in the associated graph. It is possible that all graphs provided by some graph store share
* a common value factory by store rather than by graph.
*/

use crate::error::Result;
use crate::model::Provided;
use rdftk_iri::Iri;
use rdftk_names::{rdf, xsd};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use std::time::Duration;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Literals
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// Re-export this
pub use language_tags::LanguageTag;

///
/// The set of known datatypes based on XML Schema, part 2.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Denotes a literal of type `xsd::string`.
    String,
    /// Denotes a literal of type `xsd::qname`.
    QName,
    /// Denotes a literal of type `xsd::anyURI`.
    #[allow(clippy::upper_case_acronyms)]
    Iri,
    /// Denotes a literal of type `xsd::boolean`.
    Boolean,
    /// Denotes a literal of type `xsd::float`.
    Float,
    /// Denotes a literal of type `xsd::double`.
    Double,
    /// Denotes a literal of type `xsd::long`.
    Long,
    /// Denotes a literal of type `xsd::int`.
    Int,
    /// Denotes a literal of type `xsd::short`.
    Short,
    /// Denotes a literal of type `xsd::byte`.
    Byte,
    /// Denotes a literal of type `xsd::unsignedLong`.
    UnsignedLong,
    /// Denotes a literal of type `xsd::unsignedInt`.
    UnsignedInt,
    /// Denotes a literal of type `xsd::unsignedShort`.
    UnsignedShort,
    /// Denotes a literal of type `xsd::unsignedByte`.
    UnsignedByte,
    /// Denotes a literal of type `xsd::duration`.
    Duration,
    /// Denotes an escaped string containing XML content.
    XmlLiteral,
    /// Denotes a literal where the type is indicated by the provided `Iri`.
    Other(Iri),
}

///
/// This trait describes an RDF literal which may be the object of a statement.
///
pub trait Literal: Clone + Debug + Provided {
    /// Return the lexical form of this literal.
    fn lexical_form(&self) -> &String;

    /// Returns `true` if this literal has a specified data type, else `false`.
    fn has_data_type(&self) -> bool {
        self.data_type().is_some()
    }

    /// Returns this literal's data type, if present.
    fn data_type(&self) -> Option<&DataType>;

    /// Returns `true` if this literal has a specified language, else `false`.
    fn has_language(&self) -> bool {
        self.language().is_some()
    }

    ///
    /// Return this literal's language tag, if present.
    ///
    fn language(&self) -> Option<&LanguageTag>;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Factories
// ------------------------------------------------------------------------------------------------

///
/// A value factory can be used to provide previously cached values rather than creating duplicates
/// within a graph.
///
pub trait LiteralFactory: Debug + Provided {
    type Literal: Literal;

    /// Returns a cached *untyped* literal value with the provided string.
    fn literal(&self, v: &str) -> Self::Literal;

    /// Returns a cached literal value with the provided string and language.
    fn with_language(&self, v: &str, lang: LanguageTag) -> Self::Literal;

    /// Returns a cached literal value with the provided string and language.
    fn with_language_str(&self, v: &str, lang: &str) -> Result<Self::Literal> {
        Ok(self.with_language(v, LanguageTag::from_str(lang)?))
    }

    /// Returns a cached literal value with the provided string and data type.
    fn with_data_type(&self, v: &str, data_type: DataType) -> Self::Literal;

    /// Returns a cached literal value with the provided string and data type IRI.
    fn with_data_type_iri(&self, v: &str, data_type: Iri) -> Self::Literal;

    /// Returns a cached literal value with the provided string.
    fn string(&self, v: &str) -> Self::Literal {
        self.with_data_type(v, DataType::String)
    }

    /// Returns a cached literal value with the provided QName.
    fn qname(&self, v: &str) -> Self::Literal {
        self.with_data_type(v, DataType::QName)
    }

    /// Returns a cached literal value with the provided Iri.
    fn uri(&self, v: &Iri) -> Self::Literal {
        self.with_data_type(v.as_ref(), DataType::Iri)
    }

    /// Returns a cached literal value with the provided boolean.
    fn boolean(&self, v: bool) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::Boolean)
    }

    /// Returns a cached literal value with the provided float.
    fn float(&self, v: f32) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::Float)
    }

    /// Returns a cached literal value with the provided double.
    fn double(&self, v: f64) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::Double)
    }

    /// Returns a cached literal value with the provided long.
    fn long(&self, v: i64) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::Long)
    }

    /// Returns a cached literal value with the provided int.
    fn int(&self, v: i32) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::Int)
    }

    /// Returns a cached literal value with the provided short.
    fn short(&self, v: i16) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::Short)
    }

    /// Returns a cached literal value with the provided byte.
    fn byte(&self, v: i8) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::Byte)
    }

    /// Returns a cached literal value with the provided unsigned long.
    fn unsigned_long(&self, v: u64) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::UnsignedLong)
    }

    /// Returns a cached literal value with the provided unsigned int.
    fn unsigned_int(&self, v: u32) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::UnsignedInt)
    }

    /// Returns a cached literal value with the provided unsigned short.
    fn unsigned_short(&self, v: u16) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::UnsignedShort)
    }

    /// Returns a cached literal value with the provided unsigned byte.
    fn unsigned_byte(&self, v: u8) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::UnsignedByte)
    }

    /// Returns a cached literal value with the provided duration.
    fn duration(&self, v: Duration) -> Self::Literal {
        self.chrono_duration(chrono::Duration::from_std(v).unwrap())
    }

    /// Returns a cached literal value with the provided duration.
    #[cfg(feature = "chrono_types")]
    fn chrono_duration(&self, v: chrono::Duration) -> Self::Literal {
        self.with_data_type(&v.to_string(), DataType::Duration)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Data Types
// ------------------------------------------------------------------------------------------------

impl From<Iri> for DataType {
    fn from(iri: Iri) -> Self {
        if &iri == xsd::string() {
            DataType::String
        } else if &iri == xsd::q_name() {
            DataType::QName
        } else if &iri == xsd::any_uri() {
            DataType::Iri
        } else if &iri == xsd::boolean() {
            DataType::Boolean
        } else if &iri == xsd::float() {
            DataType::Float
        } else if &iri == xsd::double() {
            DataType::Double
        } else if &iri == xsd::long() {
            DataType::Long
        } else if &iri == xsd::int() {
            DataType::Int
        } else if &iri == xsd::short() {
            DataType::Short
        } else if &iri == xsd::byte() {
            DataType::Byte
        } else if &iri == xsd::unsigned_long() {
            DataType::UnsignedLong
        } else if &iri == xsd::unsigned_int() {
            DataType::UnsignedInt
        } else if &iri == xsd::unsigned_short() {
            DataType::UnsignedShort
        } else if &iri == xsd::unsigned_byte() {
            DataType::UnsignedByte
        } else if &iri == xsd::duration() {
            DataType::Duration
        } else if &iri == rdf::xml_literal() {
            DataType::XmlLiteral
        } else {
            DataType::Other(iri)
        }
    }
}

impl DataType {
    ///
    /// Return the Iri representing this data type. Primarily these are the XML Schema data types
    /// used for literal values.
    ///
    pub fn as_iri(&self) -> &Iri {
        match &self {
            DataType::String => xsd::string(),
            DataType::QName => xsd::q_name(),
            DataType::Iri => xsd::any_uri(),
            DataType::Boolean => xsd::boolean(),
            DataType::Float => xsd::float(),
            DataType::Double => xsd::double(),
            DataType::Long => xsd::long(),
            DataType::Int => xsd::int(),
            DataType::Short => xsd::short(),
            DataType::Byte => xsd::byte(),
            DataType::UnsignedLong => xsd::unsigned_long(),
            DataType::UnsignedInt => xsd::unsigned_int(),
            DataType::UnsignedShort => xsd::unsigned_short(),
            DataType::UnsignedByte => xsd::unsigned_byte(),
            DataType::Duration => xsd::duration(),
            DataType::XmlLiteral => rdf::xml_literal(),
            DataType::Other(iri) => iri,
        }
    }
}
