/*!
Simple, in-memory implementation of the `Literal` and `LiteralFactory` traits.
*/
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use crate::model::features::Featured;
use crate::model::literal::{DataType, Literal, LiteralFactory};
use crate::model::Provided;
use language_tags::LanguageTag;
use rdftk_iri::Iri;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `Literal` trait.
///
#[derive(Clone, Debug)]
pub struct SimpleLiteral {
    lexical_form: String,
    data_type: Option<DataType>,
    language: Option<LanguageTag>,
}

///
/// Simple, in-memory implementation of the `LiteralFactory` trait.
///
#[derive(Clone, Debug, Default)]
pub struct SimpleLiteralFactory {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleLiteralFactory {
    fn provider_id(&self) -> &'static str {
        super::PROVIDER_ID
    }
}

impl Featured for SimpleLiteralFactory {
    fn supports_feature(&self, _: &Iri) -> bool {
        false
    }
}

impl LiteralFactory for SimpleLiteralFactory {
    type Literal = SimpleLiteral;

    fn literal(&self, v: &str) -> Self::Literal {
        SimpleLiteral {
            lexical_form: escape_string(v),
            data_type: None,
            language: None,
        }
    }

    fn with_language(&self, v: &str, lang: LanguageTag) -> Self::Literal {
        SimpleLiteral {
            lexical_form: escape_string(v),
            data_type: None,
            language: Some(lang),
        }
    }

    fn with_data_type(&self, v: &str, data_type: DataType) -> Self::Literal {
        SimpleLiteral {
            lexical_form: escape_string(v),
            data_type: Some(data_type),
            language: None,
        }
    }

    fn with_data_type_iri(&self, v: &str, data_type: Iri) -> Self::Literal {
        SimpleLiteral {
            lexical_form: escape_string(v),
            data_type: Some(DataType::Other(data_type)),
            language: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Literal for SimpleLiteral {
    fn lexical_form(&self) -> &String {
        &self.lexical_form
    }

    fn has_data_type(&self) -> bool {
        self.data_type.is_some()
    }

    fn data_type(&self) -> Option<&DataType> {
        self.data_type.as_ref()
    }

    fn has_language(&self) -> bool {
        self.language.is_some()
    }

    fn language(&self) -> Option<&LanguageTag> {
        self.language.as_ref()
    }
}

impl Display for SimpleLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.data_type() {
            //            Some(DataType::String) => write!(f, "\"{}\"", self.lexical_form()),
            Some(DataType::Iri) => write!(f, "<{}>", self.lexical_form()),
            //            Some(DataType::Boolean) => write!(f, "{}", self.lexical_form()),
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

impl Provided for SimpleLiteral {
    fn provider_id(&self) -> &'static str {
        super::PROVIDER_ID
    }
}

impl PartialEq for SimpleLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.lexical_form == other.lexical_form
            && self.data_type == other.data_type
            && self.language == other.language
    }
}

impl PartialEq<String> for SimpleLiteral {
    fn eq(&self, other: &String) -> bool {
        self.lexical_form() == other
            && (self.data_type().is_none() || self.data_type() == Some(&DataType::String))
    }
}

impl PartialEq<str> for SimpleLiteral {
    fn eq(&self, other: &str) -> bool {
        self.lexical_form() == other
            && (self.data_type().is_none() || self.data_type() == Some(&DataType::String))
    }
}

// Name

// QName

impl PartialEq<Iri> for SimpleLiteral {
    fn eq(&self, other: &Iri) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Iri)
    }
}

impl PartialEq<bool> for SimpleLiteral {
    fn eq(&self, other: &bool) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Boolean)
    }
}

impl PartialEq<f32> for SimpleLiteral {
    fn eq(&self, other: &f32) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Float)
    }
}

impl PartialEq<f64> for SimpleLiteral {
    fn eq(&self, other: &f64) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Double)
    }
}

// Long

// Into

// Short

impl PartialEq<i8> for SimpleLiteral {
    fn eq(&self, other: &i8) -> bool {
        *self.lexical_form() == other.to_string() && self.data_type() == Some(&DataType::Byte)
    }
}

// Unsigned Long

// Unsigned Into

// Unsigned Short

impl PartialEq<u8> for SimpleLiteral {
    fn eq(&self, other: &u8) -> bool {
        *self.lexical_form() == other.to_string()
            && self.data_type() == Some(&DataType::UnsignedByte)
    }
}

// Duration

// Xml Literal

impl Eq for SimpleLiteral {}

impl Hash for SimpleLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lexical_form.hash(state);
        self.data_type.hash(state);
        self.language.hash(state);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn escape_string(value: &str) -> String {
    let formatted = format!("{:?}", value);
    formatted[1..formatted.len() - 1].to_string()
}
