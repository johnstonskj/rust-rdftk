/*!
The `Literal` type used in the object component of a statement.

# Example

TBD

*/

use crate::QName;
use rdftk_iri::IRI;
use rdftk_names::xsd;
use std::fmt::{Display, Formatter};
use std::time::Duration;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    String,
    QName,
    URI,
    Boolean,
    Float,
    Double,
    Long,
    Int,
    Short,
    Byte,
    UnsignedLong,
    UnsignedInt,
    UnsignedShort,
    UnsignedByte,
    Duration,
    Other(Box<IRI>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Literal {
    lexical_form: String,
    data_type: Option<DataType>,
    language: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
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
                        DataType::URI => xsd::any_uri(),
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
                        DataType::Other(uri) => uri.as_ref().clone(),
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
        Self {
            lexical_form: value,
            data_type: Some(DataType::String),
            language: None,
        }
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::String),
            language: None,
        }
    }
}

impl From<IRI> for Literal {
    fn from(value: IRI) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::URI),
            language: None,
        }
    }
}

impl From<QName> for Literal {
    fn from(value: QName) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::QName),
            language: None,
        }
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(DataType::Boolean),
            language: None,
        }
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
        Self {
            lexical_form: format!("T{}.{:09}S", value.as_secs(), value.subsec_nanos()),
            data_type: Some(DataType::Duration),
            language: None,
        }
    }
}

impl Literal {
    pub fn new(value: &str) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: None,
            language: None,
        }
    }

    pub fn new_typed(value: &str, data_type: DataType) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: Some(data_type),
            language: None,
        }
    }

    pub fn new_string(value: &str, language: &str) -> Self {
        Self {
            lexical_form: value.to_string(),
            data_type: None,
            language: Some(language.to_string()),
        }
    }

    pub fn lexical_form(&self) -> &String {
        &self.lexical_form
    }

    pub fn has_data_type(&self) -> bool {
        self.data_type.is_some()
    }

    pub fn data_type(&self) -> &Option<DataType> {
        &self.data_type
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    pub fn language(&self) -> &Option<String> {
        &self.language
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn test_to_string(literal: Literal, result: &str) {
        assert_eq!(literal.to_string(), result)
    }

    #[test]
    fn test_from_types() {
        test_to_string(
            "a string".to_string().into(),
            "\"a string\"^^<http://www.w3.org/2001/XMLSchema#string>",
        );
        test_to_string(
            true.into(),
            "\"true\"^^<http://www.w3.org/2001/XMLSchema#boolean>",
        );
        test_to_string(
            1u64.into(),
            "\"1\"^^<http://www.w3.org/2001/XMLSchema#unsignedLong>",
        );

        let start = Instant::now();
        std::thread::sleep(Duration::from_secs(2));
        let duration = start.elapsed();
        println!("Duration  In: {:#?}", duration);
        let duration: Literal = duration.into();
        println!("Duration Out: {}", duration);
        let duration = duration.to_string();
        assert!(duration.starts_with("\"T2."));
        assert!(duration.ends_with("S\"^^<http://www.w3.org/2001/XMLSchema#duration>"));
    }
}
