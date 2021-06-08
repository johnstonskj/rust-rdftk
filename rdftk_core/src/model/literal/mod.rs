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
* use rdftk_core::model::literal::{Literal, DataType};
* use rdftk_core::simple::literal::literal_factory;
* use std::time::Duration;
*
* let factory = literal_factory();
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
*/

use crate::model::Equiv;
use rdftk_names::xsd;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait describes an RDF literal which may be the object of a statement.
///
pub trait Literal: Debug {
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

    ///
    /// Return the factory that creates literals using the same provider as `self`.
    ///
    /// Note that this uses Arc as a reference as factories are explicitly intended for cross-thread
    /// usage.
    ///
    fn factory(&self) -> LiteralFactoryRef;
}

///
/// The actual object storage type, reference counted for memory management.
///
pub type LiteralRef = Rc<dyn Literal>;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PartialEq<dyn Literal> for dyn Literal {
    fn eq(&self, other: &dyn Literal) -> bool {
        self.lexical_form() == other.lexical_form()
            && self.data_type() == other.data_type()
            && self.language() == other.language()
    }
}

impl Eq for dyn Literal {}

impl Hash for dyn Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lexical_form().hash(state);
        self.data_type().hash(state);
        self.language().hash(state);
    }
}

impl Display for dyn Literal {
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
                (None, Some(language)) => format!("@{}", language),
                _ => String::new(),
            }
        )
    }
}

impl Equiv<String> for dyn Literal {
    fn eqv(&self, other: &String) -> bool {
        self.lexical_form() == other && self.data_type() == Some(&DataType::String)
            || self.data_type() == None
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod data_type;
pub use data_type::*;

#[doc(hidden)]
mod factory;
pub use factory::*;

#[doc(hidden)]
mod lang;
pub use lang::*;
