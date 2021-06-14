/*!
A simple model for constructing SKOS thesauri. This is not a complete API in
that it's extensibility with OWL is limited.

Details TBD

# Example

TBD

*/

use crate::model::{Labeled, ToStatement, ToUri};
use crate::ns;
use rdftk_core::model::literal::{DataType, LanguageTag, LiteralFactoryRef, LiteralRef};
use rdftk_core::model::statement::{StatementFactoryRef, StatementRef, SubjectNodeRef};
use rdftk_iri::IRIRef;
use rdftk_names::dc;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LabelKind {
    Preferred,
    Alternative,
    Hidden,
    Other(IRIRef),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    kind: LabelKind,
    text: String,
    language: Option<LanguageTag>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralProperty {
    predicate: IRIRef,
    lexical_form: String,
    data_type: Option<DataType>,
    language: Option<LanguageTag>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn final_preferred_label(
    labeled: &impl Labeled,
    for_language: &Option<LanguageTag>,
) -> Option<String> {
    labeled
        .labels()
        .iter()
        .find(|label| {
            label.kind() == &LabelKind::Preferred && label.language() == for_language.as_ref()
        })
        .map(|label| label.text().clone())
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for LabelKind {
    fn default() -> Self {
        Self::Preferred
    }
}

impl ToUri for LabelKind {
    fn to_uri(&self) -> IRIRef {
        match self {
            Self::Preferred => ns::pref_label(),
            Self::Alternative => ns::alt_label(),
            Self::Hidden => ns::hidden_label(),
            Self::Other(iri) => iri,
        }
        .clone()
    }
}

// ------------------------------------------------------------------------------------------------

impl ToStatement for Label {
    fn to_statement(
        &self,
        subject: &SubjectNodeRef,
        statements: &StatementFactoryRef,
        literals: &LiteralFactoryRef,
    ) -> StatementRef {
        statements
            .statement(
                subject.clone(),
                self.kind.to_uri(),
                statements.literal_object(if let Some(language) = &self.language {
                    literals.with_language(&self.text, language.clone())
                } else {
                    literals.literal(&self.text)
                }),
            )
            .unwrap()
    }
}

impl Label {
    pub fn preferred(text: &str, language: &str) -> Self {
        Self::new(LabelKind::Preferred, text, language)
    }

    pub fn alternative(text: &str, language: &str) -> Self {
        Self::new(LabelKind::Alternative, text, language)
    }

    pub fn hidden(text: &str, language: &str) -> Self {
        Self::new(LabelKind::Hidden, text, language)
    }

    pub fn other(kind: IRIRef, text: &str, language: &str) -> Self {
        Self::new(LabelKind::Other(kind), text, language)
    }

    fn new(kind: LabelKind, text: &str, language: &str) -> Self {
        Self {
            kind,
            text: text.to_string(),
            language: if language.is_empty() {
                None
            } else {
                Some(LanguageTag::from_str(language).unwrap())
            },
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn kind(&self) -> &LabelKind {
        &self.kind
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn language(&self) -> Option<&LanguageTag> {
        self.language.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl ToStatement for LiteralProperty {
    fn to_statement(
        &self,
        subject: &SubjectNodeRef,
        statements: &StatementFactoryRef,
        literals: &LiteralFactoryRef,
    ) -> StatementRef {
        statements
            .statement(
                subject.clone(),
                self.predicate.clone(),
                statements.literal_object(self.make_literal(literals)),
            )
            .unwrap()
    }
}

impl LiteralProperty {
    pub fn new(predicate: IRIRef, lexical_form: &str) -> Self {
        Self {
            predicate,
            lexical_form: lexical_form.to_string(),
            data_type: None,
            language: None,
        }
    }

    pub fn with_data_type(predicate: IRIRef, lexical_form: &str, data_type: DataType) -> Self {
        Self {
            predicate,
            lexical_form: lexical_form.to_string(),
            data_type: Some(data_type),
            language: None,
        }
    }

    pub fn with_language(predicate: IRIRef, lexical_form: &str, language: LanguageTag) -> Self {
        Self {
            predicate,
            lexical_form: lexical_form.to_string(),
            data_type: None,
            language: Some(language),
        }
    }

    // SKOS properties

    pub fn change_note(text: &str) -> Self {
        Self::new(ns::change_note().clone(), text)
    }
    pub fn change_note_with(text: &str, language: &str) -> Self {
        Self::with_language(
            ns::change_note().clone(),
            text,
            LanguageTag::from_str(language).unwrap(),
        )
    }
    pub fn definition(text: &str) -> Self {
        Self::new(ns::definition().clone(), text)
    }
    pub fn definition_with(text: &str, language: &str) -> Self {
        Self::with_language(
            ns::definition().clone(),
            text,
            LanguageTag::from_str(language).unwrap(),
        )
    }
    pub fn editorial_note(text: &str) -> Self {
        Self::new(ns::editorial_note().clone(), text)
    }
    pub fn editorial_note_with(text: &str, language: &str) -> Self {
        Self::with_language(
            ns::editorial_note().clone(),
            text,
            LanguageTag::from_str(language).unwrap(),
        )
    }
    pub fn example(text: &str) -> Self {
        Self::new(ns::example().clone(), text)
    }
    pub fn example_with(text: &str, language: &str) -> Self {
        Self::with_language(
            ns::example().clone(),
            text,
            LanguageTag::from_str(language).unwrap(),
        )
    }
    pub fn history_note(text: &str) -> Self {
        Self::new(ns::history_note().clone(), text)
    }
    pub fn history_note_with(text: &str, language: &str) -> Self {
        Self::with_language(
            ns::history_note().clone(),
            text,
            LanguageTag::from_str(language).unwrap(),
        )
    }
    pub fn note(text: &str) -> Self {
        Self::new(ns::note().clone(), text)
    }
    pub fn note_with(text: &str, language: &str) -> Self {
        Self::with_language(
            ns::note().clone(),
            text,
            LanguageTag::from_str(language).unwrap(),
        )
    }
    pub fn scope_note(text: &str) -> Self {
        Self::new(ns::scope_note().clone(), text)
    }
    pub fn scope_note_with(text: &str, language: &str) -> Self {
        Self::with_language(
            ns::scope_note().clone(),
            text,
            LanguageTag::from_str(language).unwrap(),
        )
    }

    pub fn notation(text: &str) -> Self {
        Self::new(ns::notation().clone(), text)
    }

    // Dublin Core properties

    pub fn available(text: &str) -> Self {
        Self::new(dc::terms::created().clone(), text)
    }
    pub fn created(text: &str) -> Self {
        Self::new(dc::terms::created().clone(), text)
    }
    pub fn creator(text: &str) -> Self {
        Self::new(dc::terms::creator().clone(), text)
    }
    pub fn date_accepted(text: &str) -> Self {
        Self::new(dc::terms::date_accepted().clone(), text)
    }
    pub fn date_submitted(text: &str) -> Self {
        Self::new(dc::terms::date_submitted().clone(), text)
    }
    pub fn issued(text: &str) -> Self {
        Self::new(dc::terms::issued().clone(), text)
    }
    pub fn modified(text: &str) -> Self {
        Self::new(dc::terms::modified().clone(), text)
    }
    pub fn publisher(text: &str) -> Self {
        Self::new(dc::terms::publisher().clone(), text)
    }
    pub fn rights(text: &str) -> Self {
        Self::new(dc::terms::rights().clone(), text)
    }
    pub fn source(text: &str) -> Self {
        Self::new(dc::terms::source().clone(), text)
    }
    pub fn subject(text: &str) -> Self {
        Self::new(dc::terms::subject().clone(), text)
    }
    pub fn title(text: &str) -> Self {
        Self::new(dc::terms::title().clone(), text)
    }

    // Term Status properties

    pub fn term_status(status: &str) -> Self {
        Self::new(ns::term_status::term_status().clone(), status)
    }

    // --------------------------------------------------------------------------------------------

    pub fn predicate(&self) -> &IRIRef {
        &self.predicate
    }

    pub fn lexical_form(&self) -> &String {
        &self.lexical_form
    }

    pub fn has_data_type(&self) -> bool {
        self.data_type.is_some()
    }

    pub fn data_type(&self) -> Option<&DataType> {
        self.data_type.as_ref()
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    pub fn language(&self) -> Option<&LanguageTag> {
        self.language.as_ref()
    }

    pub fn make_literal(&self, factory: &LiteralFactoryRef) -> LiteralRef {
        if let Some(data_type) = &self.data_type {
            factory.with_data_type(&self.lexical_form, data_type.clone())
        } else if let Some(language) = &self.language {
            factory.with_language(&self.lexical_form, language.clone())
        } else {
            factory.literal(&self.lexical_form)
        }
    }
}
