//!
//! This module provides the types `PrefixedName`, `Namespace`, `Name`, and `LocalName`.
//!

use crate::error::NameParseError;
use std::{fmt::Display, str::FromStr};
use strum::{EnumIs, EnumTryAs};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A [`PrefixedName`], from the [SPARQL](https://www.w3.org/TR/rdf-sparql-query/) specification,
/// is either a namespace reference or a namespace-qualified local name.
///
/// ## Examples
///
/// TBD
///
/// ## Specification
///
/// ```text
/// [68]  	PrefixedName	  ::=  	PNAME_LN | PNAME_NS
/// ```
/// For `PNAME_LN` see [`LocalName`], for `PNAME_NS` see [`Namespace`].
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIs, EnumTryAs)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum PrefixedName {
    Namespace(Namespace),
    Local(LocalName),
}

///
/// A [`Namespace`], `PNAME_NS` from the [SPARQL](https://www.w3.org/TR/rdf-sparql-query/)
/// specification, is a prefix string used to reference a namespace IRI in languages such as
/// OWL, Turtle, SPARQL and so forth.
///
/// ## Examples
///
/// ```rust
/// use rdftk_iri::{Namespace};
/// use std::str::FromStr;
///
/// let ns = Namespace::from_str(":").unwrap();
/// assert!(ns.is_default());
/// assert_eq!(ns, Namespace::new_default());
/// ```
///
/// ```rust
/// use rdftk_iri::{LocalName, Name, Namespace};
/// use std::str::FromStr;
///
/// let ns = Namespace::from_str("rdf:").unwrap();
/// assert!(!ns.is_default());
/// ```
///
/// ```rust
/// use rdftk_iri::{LocalName, Name, Namespace};
/// use std::str::FromStr;
///
/// let ns = Namespace::from_str("rdf:").unwrap();
/// let name: LocalName = ns.qualify(&Name::from_str("type").unwrap());
/// assert_eq!("rdf:type".to_string(), name.to_string());
/// ```
///
/// ## Specification
///
/// ```text
/// [71]  	PNAME_NS	  ::=  	PN_PREFIX? ':'
/// [99]  	PN_PREFIX	  ::=  	PN_CHARS_BASE ((PN_CHARS|'.')* PN_CHARS)?
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Namespace(String);

pub const NAMESPACE_SEPARATOR_CHAR: char = ':';
pub const NAMESPACE_DEFAULT_STRING: &str = ":";

///
/// A [`Name`], `PN_LOCAL` from the [SPARQL](https://www.w3.org/TR/rdf-sparql-query/)
/// specification, represents that portion of a [`LocalName`] that refers to the
/// locally defined or referenced *thing*.
///
/// The `Name` type is the name of a *thing* defined within some namespace.
///
/// ```rust
/// use rdftk_iri::Name;
/// use std::str::FromStr;
///
/// assert!(Name::from_str("subClassOf").is_ok());
/// ```
///
/// ```rust
/// use rdftk_iri::Name;
/// use std::str::FromStr;
///
/// assert!(Name::from_str("rdf:").is_err());
/// ```
///
/// ```rust
/// use rdftk_iri::{LocalName, Name, Namespace};
/// use std::str::FromStr;
///
/// let name = Name::from_str("type").unwrap();
/// let name: LocalName = name.qualify(&Namespace::from_str("rdf:").unwrap());
/// assert_eq!("rdf:type".to_string(), name.to_string());
/// ```
///
/// ## Specification
///
/// ```text
/// PN_LOCAL	  ::=  	( PN_CHARS_U | [0-9] ) ((PN_CHARS|'.')* PN_CHARS)?
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Name(String);

///
/// A [`LocalName`], `PNAME_LN` from the [SPARQL](https://www.w3.org/TR/rdf-sparql-query/)
/// specification, represents the tuple of [`Namespace`] and [`Name`], such that every
/// name is qualified with the namespace it is defined within.
///
/// ## Examples
///
///
/// ```rust
/// use rdftk_iri::{LocalName, Name};
/// use std::str::FromStr;
///
/// let name: LocalName = ":name".parse().expect("parse error");
/// assert!(name.is_namespace_default());
/// assert_eq!(":", name.namespace().as_ref());
///
/// assert_eq!(name, LocalName::new_in_default(Name::from_str("name").unwrap()))
/// ```
///
/// ```rust
/// use rdftk_iri::{LocalName, Name, Namespace};
/// use std::str::FromStr;
///
/// let name: LocalName = "prefix:name".parse().expect("parse error");
/// assert!(!name.is_namespace_default());
/// assert_eq!("prefix:", name.namespace().as_ref());
/// assert_eq!("name", name.name().as_ref());
/// ```
/// ## Specification
///
/// ```text
/// [72]  	PNAME_LN	  ::=  	PNAME_NS PN_LOCAL
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LocalName {
    namespace: Namespace,
    name: Name,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ PrefixedName
// ------------------------------------------------------------------------------------------------

impl Display for PrefixedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Namespace(v) => v.fmt(f),
            Self::Local(v) => v.fmt(f),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Namespace
// ------------------------------------------------------------------------------------------------

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Namespace {
    type Err = NameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(NameParseError::EmptyString)
        } else if s == NAMESPACE_DEFAULT_STRING {
            // reserved values, check these first.
            Ok(Self(s.to_string()))
        } else if let Some(pn_prefix) = s.strip_suffix(':') {
            // The following is PN_PREFIX
            if pn_prefix.contains(NAMESPACE_SEPARATOR_CHAR) {
                Err(NameParseError::TooManySeparators(s.to_string()))
            } else if pn_prefix.is_empty() {
                Ok(Self(s.to_string()))
            } else {
                let mut chars = pn_prefix.chars();
                if is_pn_chars_base(chars.next().unwrap()) && chars.all(is_pn_prefix_local_rest) {
                    Ok(Self(s.to_string()))
                } else {
                    Err(NameParseError::InvalidCharacter(s.to_string()))
                }
            }
        } else {
            Err(NameParseError::MissingSeparator(s.to_string()))
        }
    }
}

impl AsRef<str> for Namespace {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Namespace> for String {
    fn from(value: Namespace) -> Self {
        value.0
    }
}

impl From<&Namespace> for String {
    fn from(value: &Namespace) -> Self {
        value.0.clone()
    }
}

impl Namespace {
    pub fn new_named<S>(s: S) -> Result<Self, NameParseError>
    where
        S: AsRef<str>,
    {
        Self::from_str(&format!("{}{NAMESPACE_SEPARATOR_CHAR}", s.as_ref()))
    }

    pub fn new_default() -> Self {
        Self(NAMESPACE_DEFAULT_STRING.to_string())
    }

    ///
    /// Returns a new `NamespaceName` instance from the string `s` **without** any validation.
    ///
    pub fn new_unchecked<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        let s = s.into();
        if s.ends_with(NAMESPACE_SEPARATOR_CHAR) {
            Self(s)
        } else {
            Self(format!("{s}:"))
        }
    }

    pub fn is_default(&self) -> bool {
        self.0 == NAMESPACE_DEFAULT_STRING
    }

    pub fn name_string(&self) -> Option<&str> {
        let name = self.0.strip_suffix(NAMESPACE_SEPARATOR_CHAR).unwrap();
        if name.is_empty() { None } else { Some(name) }
    }

    pub fn qualify(&self, name: &Name) -> LocalName {
        LocalName::new(self.clone(), name.clone())
    }

    ///
    /// Returns `true` if the string in `s` is a valid `PNAME_NS`.
    ///
    pub fn is_valid_str<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        Self::from_str(s.as_ref()).is_ok()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Name
// ------------------------------------------------------------------------------------------------

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Name {
    type Err = NameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(NameParseError::EmptyString)
        } else if s.ends_with('.') {
            Err(NameParseError::InvalidCharacter(s.to_string()))
        } else {
            let mut chars = s.chars();
            if is_pn_local_first(chars.next().unwrap()) && chars.all(is_pn_prefix_local_rest) {
                // TODO: ensure it does not end with '
                Ok(Self(s.to_string()))
            } else {
                Err(NameParseError::InvalidCharacter(s.to_string()))
            }
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Name> for String {
    fn from(value: Name) -> Self {
        value.0
    }
}

impl From<&Name> for String {
    fn from(value: &Name) -> Self {
        value.0.clone()
    }
}

impl Name {
    ///
    /// Returns a new `NamespaceName` instance from the string `s` **without** any validation.
    ///
    pub fn new_unchecked<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    ///
    /// Returns `true` if the string in `s` is a valid `PNAME_NS`.
    ///
    pub fn is_valid_str<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        Self::from_str(s.as_ref()).is_ok()
    }

    pub fn qualify(&self, in_namespace: &Namespace) -> LocalName {
        LocalName::new(in_namespace.clone(), self.clone())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations > LocalName
// ------------------------------------------------------------------------------------------------

impl Display for LocalName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.namespace, self.name)
    }
}

impl FromStr for LocalName {
    type Err = NameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(NameParseError::EmptyString)
        } else if let Some(separator) = s.find(':') {
            Ok(Self {
                namespace: Namespace::from_str(&s[0..=separator])?,
                name: Name::from_str(&s[separator + 1..])?,
            })
        } else {
            Err(NameParseError::MissingSeparator(s.to_string()).into())
        }
    }
}

impl From<LocalName> for String {
    fn from(value: LocalName) -> Self {
        value.to_string()
    }
}

impl From<&LocalName> for String {
    fn from(value: &LocalName) -> Self {
        value.to_string()
    }
}

impl LocalName {
    ///
    /// Construct a new qualified `LocalName`: "`{prefix}:{name}`". This will return an error if either
    /// the prefix or name is empty or is an invalid `LocalName` part.
    ///
    pub fn new(namespace: Namespace, name: Name) -> Self {
        Self { namespace, name }
    }

    pub fn new_in_default(name: Name) -> Self {
        Self {
            namespace: Namespace::new_default(),
            name,
        }
    }

    ///
    /// Construct a new `LocalName` **without** any validation checks on the given values.
    ///
    pub fn new_unchecked<S1, S2>(namespace: S1, name: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            namespace: Namespace::new_unchecked(namespace),
            name: Name::new_unchecked(name),
        }
    }

    ///
    /// Returns the `namespace` part of this `LocalName`, if present.
    ///
    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn is_namespace_default(&self) -> bool {
        self.namespace.is_default()
    }

    ///
    /// Returns the `name` part of this `LocalName`.
    ///
    pub fn name(&self) -> &Name {
        &self.name
    }

    ///
    /// Format this `LocalName` as a Curie string.
    ///
    pub fn as_curie(&self) -> String {
        format!("[{}{}]", self.namespace, self.name)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn is_pn_chars_base(c: char) -> bool {
    match c {
        'A'..'Z'
        | 'a'..'z'
        | '\u{00C0}'..='\u{00D6}'
        | '\u{00D8}'..='\u{00F6}'
        | '\u{00F8}'..='\u{02FF}'
        | '\u{0370}'..='\u{037D}'
        | '\u{037F}'..='\u{1FFF}'
        | '\u{200C}'..='\u{200D}'
        | '\u{2070}'..='\u{218F}'
        | '\u{2C00}'..='\u{2FEF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{F900}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}'
        | '\u{10000}'..='\u{EFFFF}' => true,
        _ => false,
    }
}

fn is_pn_chars_u(c: char) -> bool {
    is_pn_chars_base(c) || c == '_'
}

fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c)
        || c == '-'
        || c.is_ascii_digit()
        || c == '\u{00B7}'
        || ('\u{0300}'..='\u{036F}').contains(&c)
        || ('\u{203F}'..='\u{2040}').contains(&c)
}

fn is_pn_local_first(c: char) -> bool {
    is_pn_chars_u(c) || c.is_ascii_digit()
}

fn is_pn_prefix_local_rest(c: char) -> bool {
    is_pn_chars(c) || c == '.'
}
