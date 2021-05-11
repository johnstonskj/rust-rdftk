/*!
A simple model for constructing SKOS thesauri. This is not a complete API in
that it's extensibility with OWL is limited.

Details TBD

# Example

TBD

*/

use crate::model::{Labeled, ToStatement, ToUri};
use crate::ns;
use rdftk_core::statement::{StatementRef, SubjectNodeRef};
use rdftk_core::{Literal, Statement};
use rdftk_iri::IRIRef;
use rdftk_names::dc;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LabelKind {
    Preferred,
    Alternative,
    Hidden,
    //    Other(IRIRef),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    kind: LabelKind,
    text: String,
    language: String,
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct LabelRelation {
//     kind: IRIRef,
//     a: Rc<Label>,
//     b: Rc<Label>,
//     // https://www.w3.org/TR/skos-reference/#xl-label-relations
// }

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralProperty {
    predicate: IRIRef,
    value: Literal,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn final_preferred_label(labeled: &impl Labeled, for_language: &str) -> Option<String> {
    let for_language = for_language.to_string();
    labeled
        .labels()
        .iter()
        .find(|label| label.kind() == &LabelKind::Preferred && label.language == for_language)
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
            //            Self::Other(iri) => iri,
        }
        .clone()
    }
}

// ------------------------------------------------------------------------------------------------

impl ToStatement for Label {
    fn to_statement(&self, subject: &SubjectNodeRef) -> StatementRef {
        Statement::new_ref(
            subject.clone(),
            match self.kind {
                LabelKind::Preferred => ns::pref_label(),
                LabelKind::Alternative => ns::alt_label(),
                LabelKind::Hidden => ns::hidden_label(),
            }
            .clone(),
            Literal::with_language(&self.text, &self.language).into(),
        )
    }
}

impl Label {
    pub fn preferred(text: &str, language: &str) -> Self {
        Self {
            kind: LabelKind::Preferred,
            text: text.to_string(),
            language: language.to_string(),
        }
    }
    pub fn alternative(text: &str, language: &str) -> Self {
        Self {
            kind: LabelKind::Alternative,
            text: text.to_string(),
            language: language.to_string(),
        }
    }
    pub fn hidden(text: &str, language: &str) -> Self {
        Self {
            kind: LabelKind::Hidden,
            text: text.to_string(),
            language: language.to_string(),
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn kind(&self) -> &LabelKind {
        &self.kind
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn language(&self) -> &String {
        &self.language
    }
}

// ------------------------------------------------------------------------------------------------

impl ToStatement for LiteralProperty {
    fn to_statement(&self, subject: &SubjectNodeRef) -> StatementRef {
        Statement::new_ref(
            subject.clone(),
            self.predicate.clone(),
            self.value.clone().into(),
        )
    }
}

impl LiteralProperty {
    pub fn new(predicate: IRIRef, value: Literal) -> Self {
        Self { predicate, value }
    }

    // SKOS properties

    pub fn change_note(text: &str) -> Self {
        Self::new(ns::change_note().clone(), text.into())
    }
    pub fn change_note_with(text: &str, language: &str) -> Self {
        Self::new(
            ns::change_note().clone(),
            Literal::with_language(text, language),
        )
    }
    pub fn definition(text: &str) -> Self {
        Self::new(ns::definition().clone(), text.into())
    }
    pub fn definition_with(text: &str, language: &str) -> Self {
        Self::new(
            ns::definition().clone(),
            Literal::with_language(text, language),
        )
    }
    pub fn editorial_note(text: &str) -> Self {
        Self::new(ns::editorial_note().clone(), text.into())
    }
    pub fn editorial_note_with(text: &str, language: &str) -> Self {
        Self::new(
            ns::editorial_note().clone(),
            Literal::with_language(text, language),
        )
    }
    pub fn example(text: &str) -> Self {
        Self::new(ns::example().clone(), text.into())
    }
    pub fn example_with(text: &str, language: &str) -> Self {
        Self::new(
            ns::example().clone(),
            Literal::with_language(text, language),
        )
    }
    pub fn history_note(text: &str) -> Self {
        Self::new(ns::history_note().clone(), text.into())
    }
    pub fn history_note_with(text: &str, language: &str) -> Self {
        Self::new(
            ns::history_note().clone(),
            Literal::with_language(text, language),
        )
    }
    pub fn note(text: &str) -> Self {
        Self::new(ns::note().clone(), text.into())
    }
    pub fn note_with(text: &str, language: &str) -> Self {
        Self::new(ns::note().clone(), Literal::with_language(text, language))
    }
    pub fn scope_note(text: &str) -> Self {
        Self::new(ns::scope_note().clone(), text.into())
    }
    pub fn scope_note_with(text: &str, language: &str) -> Self {
        Self::new(
            ns::scope_note().clone(),
            Literal::with_language(text, language),
        )
    }

    pub fn notation(text: &str) -> Self {
        Self::new(ns::notation().clone(), Literal::new(text))
    }

    // Dublin Core properties

    pub fn available(text: &str) -> Self {
        Self::new(dc::terms::created().clone(), Literal::new(text))
    }
    pub fn created(text: &str) -> Self {
        Self::new(dc::terms::created().clone(), Literal::new(text))
    }
    pub fn creator(text: &str) -> Self {
        Self::new(dc::terms::creator().clone(), Literal::new(text))
    }
    pub fn date_accepted(text: &str) -> Self {
        Self::new(dc::terms::date_accepted().clone(), Literal::new(text))
    }
    pub fn date_submitted(text: &str) -> Self {
        Self::new(dc::terms::date_submitted().clone(), Literal::new(text))
    }
    pub fn issued(text: &str) -> Self {
        Self::new(dc::terms::issued().clone(), Literal::new(text))
    }
    pub fn modified(text: &str) -> Self {
        Self::new(dc::terms::modified().clone(), Literal::new(text))
    }
    pub fn publisher(text: &str) -> Self {
        Self::new(dc::terms::publisher().clone(), Literal::new(text))
    }
    pub fn rights(text: &str) -> Self {
        Self::new(dc::terms::rights().clone(), Literal::new(text))
    }
    pub fn source(text: &str) -> Self {
        Self::new(dc::terms::source().clone(), Literal::new(text))
    }
    pub fn subject(text: &str) -> Self {
        Self::new(dc::terms::subject().clone(), Literal::new(text))
    }
    pub fn title(text: &str) -> Self {
        Self::new(dc::terms::title().clone(), Literal::new(text))
    }

    // Term Status properties

    pub fn term_status(text: &str) -> Self {
        Self::new(ns::term_status::term_status().clone(), Literal::new(text))
    }

    // --------------------------------------------------------------------------------------------

    pub fn predicate(&self) -> &IRIRef {
        &self.predicate
    }

    pub fn value(&self) -> &Literal {
        &self.value
    }
}
