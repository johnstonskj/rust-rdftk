/*!
The `Literal` type used in the object component of a statement. Literal values are always strings,
although an optional data type can be provided to allow consumers to convert from string
lexical forms.

Note that duration values can be provided using `std::time::Duration`, however the
[chrono](https://crates.io/crates/chrono) crate's `chrono::Duration` may also be used. This
additional dependency also allows for correct formatting of duration lexical forms by converting
all standard duration values to chrono durations which support the correct `to_string` form.

# Example

```rust
use rdftk_core::{Literal, DataType};
let string_literal: Literal = "string value".into();
assert_eq!(string_literal.lexical_form(), "string value");
assert_eq!(string_literal.data_type().as_ref(), None);

let string_literal: Literal = Literal::with_language("string value", "en_US");
assert_eq!(string_literal.language().as_ref().unwrap(), "en_US");
assert_eq!(string_literal.data_type().as_ref(), None);

let typed_string_literal: Literal = Literal::string("string value");
assert_eq!(typed_string_literal.data_type().as_ref().unwrap(), &DataType::String);

let long_literal: Literal = Literal::with_type("212", DataType::Long);
assert_eq!(long_literal.data_type().as_ref().unwrap(), &DataType::Long);

let long_literal: Literal = 212u64.into();
assert_eq!(long_literal.lexical_form(), "212");

use std::time::Duration;

let duration_literal: Literal = Duration::from_secs(63542).into();
assert_eq!(duration_literal.lexical_form(), "PT63542S");
assert_eq!(duration_literal.data_type().as_ref().unwrap(), &DataType::Duration);
```

*/

use crate::qname::QName;
use crate::statement::ObjectNodeRef;
use rdftk_iri::IRIRef;
use rdftk_names::xsd;
use std::fmt::{Display, Formatter};
use std::time::Duration;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
///
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Denotes a literal of type `xsd::string`.
    String,
    /// Denotes a literal of type `xsd::qname`.
    QName,
    /// Denotes a literal of type `xsd::anyURI`.
    #[allow(clippy::upper_case_acronyms)]
    IRI,
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
    /// Denotes a literal where the type is indicated by the provided `IRI`.
    Other(IRIRef),
}

///
///
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Literal {
    lexical_form: String,
    data_type: Option<DataType>,
    language: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_iri())
    }
}

impl From<IRIRef> for DataType {
    fn from(v: IRIRef) -> Self {
        DataType::Other(v)
    }
}

impl From<DataType> for IRIRef {
    fn from(v: DataType) -> Self {
        v.as_iri().clone()
    }
}

impl DataType {
    ///
    /// Return the IRI representing this data type. Primarily these are the XML Schema data types
    /// used for literal values.
    ///
    pub fn as_iri(&self) -> &IRIRef {
        match &self {
            DataType::String => xsd::string(),
            DataType::QName => xsd::q_name(),
            DataType::IRI => xsd::any_uri(),
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
            DataType::Other(iri) => iri,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"{}",
            self.lexical_form(),
            match (self.data_type(), self.language()) {
                (Some(data_type), None) => format!(
                    "^^<{}>",
                    match data_type {
                        DataType::String => xsd::string(),
                        DataType::QName => xsd::q_name(),
                        DataType::IRI => xsd::any_uri(),
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
                        DataType::Other(iri) => iri,
                    }
                ),
                (None, Some(language)) => format!("@{}", language.to_lowercase()),
                _ => String::new(),
            }
        )
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::new(&value)
    }
}

impl From<IRIRef> for Literal {
    fn from(value: IRIRef) -> Self {
        Self::iri(value)
    }
}

impl From<QName> for Literal {
    fn from(value: QName) -> Self {
        Self::qname(value)
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self::boolean(value)
    }
}

impl From<f32> for Literal {
    fn from(value: f32) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::Float),
            language: None,
        }
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::Double),
            language: None,
        }
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::Long),
            language: None,
        }
    }
}

impl From<i32> for Literal {
    fn from(value: i32) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::Int),
            language: None,
        }
    }
}

impl From<i16> for Literal {
    fn from(value: i16) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::Short),
            language: None,
        }
    }
}

impl From<i8> for Literal {
    fn from(value: i8) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::Byte),
            language: None,
        }
    }
}

impl From<u64> for Literal {
    fn from(value: u64) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::UnsignedLong),
            language: None,
        }
    }
}

impl From<u32> for Literal {
    fn from(value: u32) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::UnsignedInt),
            language: None,
        }
    }
}

impl From<u16> for Literal {
    fn from(value: u16) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::UnsignedShort),
            language: None,
        }
    }
}

impl From<u8> for Literal {
    fn from(value: u8) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::UnsignedByte),
            language: None,
        }
    }
}

impl From<Duration> for Literal {
    fn from(value: Duration) -> Self {
        Self::duration(value)
    }
}

impl From<chrono::Duration> for Literal {
    fn from(value: chrono::Duration) -> Self {
        Self::chrono_duration(value)
    }
}

impl Literal {
    /// Construct a new, untyped, literal value.
    pub fn new(value: &str) -> Self {
        Self {
            lexical_form: Self::escape_string(value),
            data_type: None,
            language: None,
        }
    }

    /// Construct a new typed literal value.
    pub fn with_type(value: &str, data_type: DataType) -> Self {
        Self {
            lexical_form: Self::escape_string(value),
            data_type: Some(data_type),
            language: None,
        }
    }

    /// Construct a new string literal value with the associated language.
    pub fn with_language(value: &str, language: &str) -> Self {
        Self {
            lexical_form: Self::escape_string(value),
            data_type: None,
            language: Some(language.to_string()),
        }
    }

    /// Construct a new `DataType::String` value.
    pub fn string(value: &str) -> Self {
        Self::with_type(value, DataType::String)
    }

    /// Construct a new `DataType::QName` value.
    pub fn qname(value: QName) -> Self {
        Self::with_type(&value.to_string(), DataType::QName)
    }

    /// Construct a new `DataType::IRI` value.
    pub fn iri(value: IRIRef) -> Self {
        Self::with_type(&value.to_string(), DataType::IRI)
    }

    /// Construct a new `DataType::Boolean` value.
    pub fn boolean(value: bool) -> Self {
        Self::with_type(&value.to_string(), DataType::Boolean)
    }

    /// Construct a new `DataType::Duration` value.
    pub fn duration(value: Duration) -> Self {
        Self::chrono_duration(chrono::Duration::from_std(value).unwrap())
    }

    /// Construct a new `DataType::Duration` value.
    pub fn chrono_duration(value: chrono::Duration) -> Self {
        Self::with_type(&value.to_string(), DataType::Duration)
    }

    /// Return the lexical form of this literal.
    pub fn lexical_form(&self) -> &String {
        &self.lexical_form
    }

    /// Returns `true` if this literal has a specified data type, else `false`.
    pub fn has_data_type(&self) -> bool {
        self.data_type.is_some()
    }

    /// Returns this literal's data type, if present.
    pub fn data_type(&self) -> &Option<DataType> {
        &self.data_type
    }

    /// Returns `true` if this literal has a specified language, else `false`.
    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    /// Returns this literal's language, if present.
    pub fn language(&self) -> &Option<String> {
        &self.language
    }

    /// Create a new object node from this literal.
    pub fn as_object(&self) -> ObjectNodeRef {
        ObjectNodeRef::from(self.clone())
    }

    fn escape_string(value: &str) -> String {
        let formatted = format!("{:?}", value);
        formatted[1..formatted.len() - 1].to_string()
    }
}
