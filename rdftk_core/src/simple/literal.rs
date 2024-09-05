/*!
Simple, in-memory implementation of the `Literal` and `LiteralFactory` traits.
*/
use crate::model::features::Featured;
use crate::model::literal::{DataType, Literal, LiteralFactory, LiteralFactoryRef, LiteralRef};
use crate::model::Provided;
use language_tags::LanguageTag;
use lazy_static::lazy_static;
use rdftk_iri::IriRef;
use std::rc::Rc;
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `Literal` trait.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SimpleLiteral {
    lexical_form: String,
    data_type: Option<DataType>,
    language: Option<LanguageTag>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Retrieve the `LiteralFactory` factory for simple `simple::SimpleLiteral` instances.
///
pub fn literal_factory() -> LiteralFactoryRef {
    FACTORY.clone()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

///
/// Simple, in-memory implementation of the `LiteralFactory` trait.
///
#[derive(Clone, Debug, Default)]
struct SimpleLiteralFactory {}

lazy_static! {
    static ref FACTORY: Arc<SimpleLiteralFactory> = Arc::new(SimpleLiteralFactory::default());
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Provided for SimpleLiteralFactory {
    fn provider_id(&self) -> &'static str {
        super::PROVIDER_ID
    }
}

impl Featured for SimpleLiteralFactory {
    fn supports_feature(&self, _: &IriRef) -> bool {
        false
    }
}

impl LiteralFactory for SimpleLiteralFactory {
    fn literal(&self, v: &str) -> LiteralRef {
        Rc::new(SimpleLiteral {
            lexical_form: escape_string(v),
            data_type: None,
            language: None,
        })
    }

    fn with_language(&self, v: &str, lang: LanguageTag) -> LiteralRef {
        Rc::new(SimpleLiteral {
            lexical_form: escape_string(v),
            data_type: None,
            language: Some(lang),
        })
    }

    fn with_data_type(&self, v: &str, data_type: DataType) -> LiteralRef {
        Rc::new(SimpleLiteral {
            lexical_form: escape_string(v),
            data_type: Some(data_type),
            language: None,
        })
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

    fn factory(&self) -> LiteralFactoryRef {
        literal_factory()
    }
}

fn escape_string(value: &str) -> String {
    let formatted = format!("{:?}", value);
    formatted[1..formatted.len() - 1].to_string()
}
