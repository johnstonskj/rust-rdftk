/*!
Provides a partial implementation of [RFC-5646 (BCP-47): Tags for Identifying
Languages](https://www.rfc-editor.org/rfc/rfc5646.html). It provides a `LanguageType` enum and
the enclosed structs `Tag` and `Extension`.

This implementation does parse correctly according to the RFC, section 2.1 Syntax, and implements
the correct rules for case-insensitive equality and some content checks. However, it does not
perform all the described validation or canonicalization.

# Conformance

The following sections outline areas where this implementation follows some of the rules
described in the RFC.

## Well-Formed

Well-formedness is described in the RFC as a syntactic check and primarily ensure that a provided
string passes the ABNF rules described in the syntax section. As such, any string that is
successfully parsed into a `LanguageTag` is well-formed.

## Validity

TBD

## Canonicalization

TBD

# Example

```rust
use rdftk_core::model::literal::LanguageTag;
use std::str::FromStr;

let language_tag = LanguageTag::from_str("hy-Latn-IT-arevela").unwrap();
if let LanguageTag::Tag(tag) = language_tag {
    println!("{:?}", tag);
    assert_eq!(tag.language(), &"hy".to_string());
    assert_eq!(tag.script().unwrap(), &"Latn".to_string());
    assert_eq!(tag.region().unwrap(), &"IT".to_string());
    assert_eq!(
        tag.variants().cloned().collect::<Vec<String>>(),
        vec!["arevela".to_string()]
    );
    assert_eq!(tag.extensions().count(), 0);
    assert_eq!(tag.private_use().count(), 0);
}
```

*/

#![allow(clippy::upper_case_acronyms)]

use crate::error::{Error, ErrorKind};
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A type that represents a language tag from
/// [RFC-5646](https://www.rfc-editor.org/rfc/rfc5646.html). A tag is either a *normal* language tag,
/// a private use tag, or a pre-existing tag grandfathered into the registration.
///
#[derive(Clone, Debug, Eq)]
pub enum LanguageTag {
    Tag(Tag),
    PrivateUse(Vec<String>),
    Grandfathered(String),
}

///
/// This struct corresponds to the structure of a *normal* language tag.
///
#[derive(Clone, Debug, Eq)]
pub struct Tag {
    language: String,
    script: Option<String>,
    region: Option<String>,
    variants: Vec<String>,
    extensions: Vec<Extension>,
    private_use: Vec<String>,
}

///
/// This struct corresponds to the extension type, a singleton and list of sub-tags.
#[derive(Clone, Debug, Eq)]
pub struct Extension {
    singleton: char,
    sub_tags: Vec<String>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[grammar = "model/literal/lang.pest"]
struct LanguageTagParser;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const LANG_SEP: &str = "-";

impl Display for LanguageTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageTag::Tag(t) => {
                write!(f, "{}", t)
            }
            LanguageTag::PrivateUse(s) => {
                write!(f, "x-{}", s.join(LANG_SEP))
            }
            LanguageTag::Grandfathered(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl FromStr for LanguageTag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed = LanguageTagParser::parse(Rule::language_tag, s).map_err(|e| {
            Error::with_chain(
                e,
                ErrorKind::InvalidFromStr(
                    s.to_string(),
                    std::any::type_name::<LanguageTag>().to_string(),
                ),
            )
        })?;
        let top_node = parsed.next().unwrap();
        language_tag(top_node)
    }
}

impl PartialEq for LanguageTag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tag(lhs), Self::Tag(rhs)) => lhs.eq(rhs),
            (Self::PrivateUse(lhs), Self::PrivateUse(rhs)) => vec_eq_ignore_case(lhs, rhs),
            (Self::Grandfathered(lhs), Self::Grandfathered(rhs)) => lhs.eq_ignore_ascii_case(rhs),
            _ => false,
        }
    }
}

impl Hash for LanguageTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            LanguageTag::Tag(tag) => tag.hash(state),
            LanguageTag::PrivateUse(ss) => ss
                .iter()
                .map(|s| s.to_ascii_lowercase())
                .collect::<Vec<String>>()
                .hash(state),
            LanguageTag::Grandfathered(s) => s.hash(state),
        }
    }
}

impl LanguageTag {
    ///
    /// Returns `true` if this instance is a *normal* language tag, else `false`.
    ///
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(_))
    }

    ///
    /// Returns `true` if this instance is a grandfathered registration, else `false`.
    ///
    pub fn is_grandfathered(&self) -> bool {
        matches!(self, Self::Grandfathered(_))
    }

    ///
    /// Returns `true` if this instance is a private use tag, else `false`.
    ///
    pub fn is_private_use(&self) -> bool {
        matches!(self, Self::PrivateUse(_))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.language)?;
        if let Some(region) = &self.region {
            write!(f, "{}{}", LANG_SEP, region)?;
        }
        for variant in &self.variants {
            write!(f, "{}{}", LANG_SEP, variant)?;
        }
        for extension in &self.extensions {
            write!(f, "{}{}", LANG_SEP, extension)?;
        }
        for private_use in &self.private_use {
            write!(f, "{}{}", LANG_SEP, private_use)?;
        }
        Ok(())
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.language.eq_ignore_ascii_case(&other.language)
            && option_eq_ignore_case(&self.script, &other.script)
            && option_eq_ignore_case(&self.region, &other.region)
            && vec_eq_ignore_case(&self.variants, &other.variants)
            && self.extensions == other.extensions
            && vec_eq_ignore_case(&self.private_use, &other.private_use)
    }
}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.language.to_ascii_lowercase().hash(state);
        self.script
            .as_ref()
            .map(|s| s.to_ascii_lowercase())
            .hash(state);
        self.variants
            .iter()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<String>>()
            .hash(state);
        self.extensions.iter().for_each(|e| e.hash(state));
        self.private_use
            .iter()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<String>>()
            .hash(state);
    }
}

impl From<Tag> for LanguageTag {
    fn from(tag: Tag) -> Self {
        LanguageTag::Tag(tag)
    }
}

impl Tag {
    ///
    /// Return the language (required) part of the language tag.
    pub fn language(&self) -> &String {
        &self.language
    }

    ///
    /// Returns `true` if this language tag has a *script* part, else `false`.
    ///
    pub fn has_script(&self) -> bool {
        self.script.is_some()
    }

    ///
    /// Return the optional *script* part of the language tag.
    ///
    pub fn script(&self) -> Option<&String> {
        self.script.as_ref()
    }

    ///
    /// Returns `true` if this language tag has a *region* part, else `false`.
    ///
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }

    ///
    /// Return the optional *region* part of the language tag.
    ///
    pub fn region(&self) -> Option<&String> {
        self.region.as_ref()
    }

    ///
    /// Returns `true` if this language tag has a *variants* part, else `false`.
    ///
    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }

    ///
    /// Return an iterator over any values of the *variants* part of the language tag.
    ///
    pub fn variants(&self) -> impl Iterator<Item = &String> {
        self.variants.iter()
    }

    ///
    /// Returns `true` if this language tag has a *extensions* part, else `false`.
    ///
    pub fn has_extensions(&self) -> bool {
        !self.extensions.is_empty()
    }

    ///
    /// Return an iterator over any values of the *extensions* part of the language tag.
    ///
    pub fn extensions(&self) -> impl Iterator<Item = &Extension> {
        self.extensions.iter()
    }

    ///
    /// Returns `true` if this language tag has a *private use* part, else `false`.
    ///
    pub fn has_private_use(&self) -> bool {
        !self.private_use.is_empty()
    }

    ///
    /// Return an iterator over any values of the *private use* part of the language tag.
    ///
    pub fn private_use(&self) -> impl Iterator<Item = &String> {
        self.private_use.iter()
    }

    ///
    /// 4.5.  Canonicalization of Language Tags
    ///
    pub fn to_canonical_format(&self) -> Self {
        // When performing canonicalization of language tags, processors MAY regularize the case of
        // the subtags ... (see Section 2.1.1).

        // [ISO639-1] recommends that language codes be written in lowercase ('mn' Mongolian).
        let language = self.language.to_lowercase();

        // [ISO15924] recommends that script codes use lowercase with the initial letter
        // capitalized ('Cyrl' Cyrillic).
        let script = self.script.as_ref().map(|s| {
            let mut cs = s.chars();
            format!(
                "{}{}",
                cs.next().unwrap().to_uppercase(),
                cs.map(|c| c.to_lowercase()).flatten().collect::<String>()
            )
        });

        // [ISO3166-1] recommends that country codes be capitalized ('MN' Mongolia).
        let region = self.region.as_ref().map(|s| s.to_uppercase());

        // Extension sequences are ordered into case-insensitive ASCII order by singleton subtag.
        let mut extensions = self.extensions.clone();
        extensions.sort_by_key(|e| e.singleton);

        Self {
            language,
            script,
            region,
            variants: self.variants.clone(),
            extensions,
            private_use: self.private_use.clone(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Extension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.singleton)?;
        for sub_tag in &self.sub_tags {
            write!(f, "{}{}", LANG_SEP, sub_tag)?;
        }
        Ok(())
    }
}

impl PartialEq for Extension {
    fn eq(&self, other: &Self) -> bool {
        self.singleton == other.singleton && vec_eq_ignore_case(&self.sub_tags, &other.sub_tags)
    }
}

impl Hash for Extension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.singleton.hash(state);
        self.sub_tags
            .iter()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<String>>()
            .hash(state);
    }
}

impl Extension {
    ///
    /// Return the singleton character part of this extension.
    ///
    pub fn singleton(&self) -> &char {
        &self.singleton
    }

    ///
    /// Return an iterator over all the sub-tags of this extension.
    ///
    pub fn sub_tags(&self) -> impl Iterator<Item = &String> {
        self.sub_tags.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn language_tag(input_pair: Pair<'_, Rule>) -> Result<LanguageTag, Error> {
    if input_pair.as_rule() == Rule::language_tag {
        let inner_pair = input_pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::lang_tag => Ok(LanguageTag::Tag(lang_tag(inner_pair)?)),
            Rule::private_use => Ok(LanguageTag::PrivateUse(private_use(inner_pair)?)),
            Rule::grandfathered => Ok(LanguageTag::Grandfathered(inner_pair.as_str().to_string())),
            _ => unreachable!(),
        }
    } else {
        Err(ErrorKind::InvalidFromStr(
            input_pair.as_str().to_string(),
            std::any::type_name::<LanguageTag>().to_string(),
        )
        .into())
    }
}

fn lang_tag(input_pair: Pair<'_, Rule>) -> Result<Tag, Error> {
    if input_pair.as_rule() == Rule::lang_tag {
        let mut inner_pairs = input_pair.into_inner();
        let language = inner_pairs.next().unwrap().as_str().to_string();
        let mut script: Option<String> = None;
        let mut region: Option<String> = None;
        let mut variants: Vec<String> = Default::default();
        let mut extensions: Vec<Extension> = Default::default();
        let mut v_private_use: Vec<String> = Default::default();
        for inner_pair in inner_pairs {
            match inner_pair.as_rule() {
                Rule::script => {
                    script = Some(inner_pair.as_str().to_string());
                }
                Rule::region => {
                    region = Some(inner_pair.as_str().to_string());
                }
                Rule::variant => {
                    variants.push(inner_pair.as_str().to_string());
                }
                Rule::extension => {
                    extensions.push(extension(inner_pair)?);
                }
                Rule::private_use => {
                    v_private_use = private_use(inner_pair)?;
                }
                Rule::SEP => {}
                _ => {
                    unreachable!()
                }
            }
        }
        let variant_set: HashSet<String> = variants.iter().cloned().collect();
        if variants.len() != variant_set.len() {
            return Err(ErrorKind::InvalidFromStr(
                format!("duplicate variants: {:?}", variants),
                std::any::type_name::<LanguageTag>().to_string(),
            )
            .into());
        }
        Ok(Tag {
            language,
            script,
            region,
            variants,
            extensions,
            private_use: v_private_use,
        })
    } else {
        Err(ErrorKind::InvalidFromStr(
            input_pair.as_str().to_string(),
            std::any::type_name::<LanguageTag>().to_string(),
        )
        .into())
    }
}

#[allow(clippy::unnecessary_wraps)]
fn extension(input_pair: Pair<'_, Rule>) -> Result<Extension, Error> {
    let singleton = &input_pair.as_str()[0..1].chars().next().unwrap();
    let sub_tags = &input_pair.as_str()[2..];

    Ok(Extension {
        singleton: *singleton,
        sub_tags: sub_tags.split(LANG_SEP).map(str::to_string).collect(),
    })
}

#[allow(clippy::unnecessary_wraps)]
fn private_use(input_pair: Pair<'_, Rule>) -> Result<Vec<String>, Error> {
    let sub_tags = &input_pair.as_str()[2..];
    Ok(sub_tags.split(LANG_SEP).map(str::to_string).collect())
}

fn option_eq_ignore_case(lhs: &Option<String>, rhs: &Option<String>) -> bool {
    match (lhs, rhs) {
        (None, None) => true,
        (Some(lhs), Some(rhs)) => lhs.eq_ignore_ascii_case(rhs),
        _ => false,
    }
}

fn vec_eq_ignore_case(lhs: &[String], rhs: &[String]) -> bool {
    if lhs.len() == rhs.len() {
        lhs.iter()
            .zip(rhs.iter())
            .all(|(lhs, rhs)| lhs.eq_ignore_ascii_case(rhs))
    } else {
        false
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
