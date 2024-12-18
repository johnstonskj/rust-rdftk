/*!
![iri](https://img.shields.io/badge/RDFtk-iri-BD1B89?logo=data:image/png;base64,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)
This crate provides an `IriExtra` trait, `Iri`, `Name`, and `QName` types and associated errors.

# IRI Type

The [`Iri`] type is a wrapper around the `url::Url` type, and as can be constructed in the
same manner.

## Examples

```rust
use rdftk_iri::Iri;
use std::str::FromStr;

let result = Iri::from_str(
    "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
).unwrap();
```

The [`IriExtra`] trait provides a number of additional methods useful to the IRI as a namespace
and namespaced-name identifier.

```rust
use rdftk_iri::{Iri, IriExtra as _, Name};
use std::str::FromStr;

let namespace = Iri::from_str(
    "https://example.org/ns/things#",
).unwrap();
assert!(namespace.looks_like_namespace());

let name = namespace.make_name(Name::from_str("ThisThing").unwrap()).unwrap();
assert_eq!(
    name.to_string(),
    "https://example.org/ns/things#ThisThing".to_string(),
);

assert_eq!(
    name.namespace(),
    Some(namespace),
);

assert_eq!(
    name.name(),
    Some(Name::from_str("ThisThing").unwrap()),
);
```

# Name (NCName) Type

TBD

# QName Type

Qualified names, names with the form `{prefix}:{name}` are used in a number of common serialization
forms and use many of the same production rules as those for XML.

## Specification

1. <https://www.w3.org/TR/REC-xml-names/>
2. <https://www.w3.org/TR/REC-xml/#NT-Name>
3. <https://www.w3.org/2001/tag/doc/qnameids>

From (1):

```text
/* Attribute Names for Namespace Declaration */

[4]  NCName          ::=  Name - (Char* ':' Char*)     /* An XML Name, minus the ":" */

/* Qualified Name */

[7]  QName           ::=  PrefixedName
                          | UnprefixedName
[8]  PrefixedName    ::=  Prefix ':' LocalPart
[9]  UnprefixedName  ::=  LocalPart
[10] Prefix          ::=  NCName
[11] LocalPart       ::=  NCName
```

From (2):

```text

[4]  NameStartChar   ::=  ":" | [A-Z] | "_" | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF]
                          | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F]
                          | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD]
                          | [#x10000-#xEFFFF]
[4a] NameChar        ::=  NameStartChar | "-" | "." | [0-9] | #xB7 | [#x0300-#x036F]
                          | [#x203F-#x2040]
[5]  Name            ::=  NameStartChar (NameChar)*
```

## Example

```rust
use rdftk_iri::{Name, QName};
use std::str::FromStr;

let prefixed: QName = "prefix:name".parse().expect("parse error");
let un_prefixed: QName = "name".parse().expect("parse error");

let prefixed: QName = QName::new(
    Name::new_unchecked("prefix"),
    Name::new_unchecked("name"),
).unwrap();
let un_prefixed: QName = QName::new_unqualified(Name::new_unchecked("name")).unwrap();

assert!(QName::from_str("").is_err());
assert!(QName::from_str("hello world").is_err());
```

# Curie

Specification: <https://www.w3.org/TR/curie/>

```text
safe_curie  :=   '[' curie ']'

curie       :=   [ [ prefix ] ':' ] reference

prefix      :=   NCName

reference   :=   irelative-ref (as defined in IRI)
```

Note that while the empty string matches the production for curie above, an empty string is NOT a
valid CURIE. The CURIE prefix '_' is reserved for use by languages that support RDF. For this
reason, the prefix '_' SHOULD be avoided by authors.

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Errors
// ------------------------------------------------------------------------------------------------

///
/// Errors reported while parsing a string into an IRI.
///
pub type Error = url::ParseError;

// ------------------------------------------------------------------------------------------------
// Public Types ❱ IRI
// ------------------------------------------------------------------------------------------------

///
/// The common type for IRI values used throughout the RDFtk packages.
///
pub type Iri = url::Url;

///
/// Additional, mainly constructor functions for the [`Iri`] type.
///
pub trait IriExtra {
    ///
    /// Returns a copy of the current IRI with the path component replaced by `path`.
    ///
    fn with_new_path<S>(&self, path: S) -> Self
    where
        S: AsRef<str>;

    ///
    /// Returns a copy of the current IRI with the fragment component replaced by `fragment`.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let no_fragment = Iri::from_str("https://example.org/ns").unwrap();
    /// let empty_fragment = Iri::from_str("https://example.org/ns#").unwrap();
    /// let some_fragment = Iri::from_str("https://example.org/ns#name").unwrap();
    ///
    /// assert_eq!(some_fragment, no_fragment.with_new_fragment("name"));
    ///
    /// assert_eq!(some_fragment, empty_fragment.with_new_fragment("name"));
    ///
    /// assert_eq!(some_fragment, some_fragment.with_new_fragment("name"));
    /// ```
    ///
    fn with_new_fragment<S>(&self, fragment: S) -> Self
    where
        S: AsRef<str>;

    ///
    /// Returns a copy of the current IRI with the fragment component replaced by an empty string.
    ///

    ///
    /// Returns a copy of the current IRI with the fragment component removed.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let no_fragment = Iri::from_str("https://example.org/ns").unwrap();
    /// let empty_fragment = Iri::from_str("https://example.org/ns#").unwrap();
    /// let some_fragment = Iri::from_str("https://example.org/ns#name").unwrap();
    ///
    /// assert_eq!(empty_fragment, no_fragment.with_empty_fragment());
    ///
    /// assert_eq!(empty_fragment, empty_fragment.with_empty_fragment());
    ///
    /// assert_eq!(empty_fragment, some_fragment.with_empty_fragment());
    /// ```
    ///
    fn with_empty_fragment(&self) -> Self;

    ///
    /// Returns a copy of the current IRI with the fragment component removed.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let no_fragment = Iri::from_str("https://example.org/ns").unwrap();
    /// let empty_fragment = Iri::from_str("https://example.org/ns#").unwrap();
    /// let some_fragment = Iri::from_str("https://example.org/ns#name").unwrap();
    ///
    /// assert_eq!(no_fragment, no_fragment.with_no_fragment());
    ///
    /// assert_eq!(no_fragment, empty_fragment.with_no_fragment());
    ///
    /// assert_eq!(no_fragment, some_fragment.with_no_fragment());
    /// ```
    ///
    fn with_no_fragment(&self) -> Self;

    ///
    /// Returns `true` if this IRI may be used as a valid namespace. A valid namespace follows the
    /// format:
    ///
    /// 1. Has an empty, but present, fragment identifier.
    /// 1. Or, it has a path ending with the character `'/'`,
    /// 1. and, it does not have a query part.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let maybe = Iri::from_str("https://example.org/ns/").unwrap();
    /// assert!(maybe.looks_like_namespace());
    ///
    /// let maybe = Iri::from_str("https://example.org/ns#").unwrap();
    /// assert!(maybe.looks_like_namespace());
    ///
    /// let maybe = Iri::from_str("https://example.org/ns/Name").unwrap();
    /// assert!(!maybe.looks_like_namespace());
    ///
    /// let maybe = Iri::from_str("https://example.org?q=10").unwrap();
    /// assert!(!maybe.looks_like_namespace());
    /// ```
    ///
    fn looks_like_namespace(&self) -> bool;

    ///
    /// IF this IRI represents a namespaced-name, return a (namespace, name) pair, else `None`.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra, Name};
    /// use std::str::FromStr;
    ///
    /// let namespace = Iri::from_str("https://example.org/ns/Name").unwrap();
    /// assert_eq!(
    ///     namespace.split(),
    ///     Some((
    ///         Iri::from_str("https://example.org/ns/").unwrap(),
    ///         Name::from_str("Name").unwrap(),
    ///     )),
    /// );
    ///
    /// let namespace = Iri::from_str("https://example.org/ns#Name").unwrap();
    /// assert_eq!(
    ///     namespace.split(),
    ///     Some((
    ///         Iri::from_str("https://example.org/ns#").unwrap(),
    ///         Name::from_str("Name").unwrap(),
    ///     )),
    /// );
    ///
    /// let namespace = Iri::from_str("https://example.org").unwrap();
    /// assert_eq!(
    ///     namespace.split(),
    ///     None,
    /// );
    /// ```
    ///
    fn split(&self) -> Option<(Self, Name)>
    where
        Self: Sized;

    ///
    /// IF this IRI represents a namespaced-name, return the namespace part, else `None`.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let ns_name = Iri::from_str("https://example.org/ns/Name").unwrap();
    /// assert_eq!(
    ///     ns_name.namespace(),
    ///     Some(Iri::from_str("https://example.org/ns/").unwrap()),
    /// );
    ///
    /// let ns_name = Iri::from_str("https://example.org/ns#Name").unwrap();
    /// assert_eq!(
    ///     ns_name.namespace(),
    ///     Some(Iri::from_str("https://example.org/ns#").unwrap()),
    /// );
    ///
    /// let ns_name = Iri::from_str("https://example.org").unwrap();
    /// assert_eq!(
    ///     ns_name.namespace(),
    ///     None,
    /// );
    /// ```
    ///39M
    fn namespace(&self) -> Option<Self>
    where
        Self: Sized,
    {
        self.split().map(|(u, _)| u)
    }

    ///
    /// IF this IRI represents a namespaced-name, return the name part, else `None`.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra, Name};
    /// use std::str::FromStr;
    ///
    /// let ns_name = Iri::from_str("https://example.org/ns/Name").unwrap();
    /// assert_eq!(
    ///     ns_name.name(),
    ///     Some(Name::from_str("Name").unwrap()),
    /// );
    ///
    /// let ns_name = Iri::from_str("https://example.org/ns#Name").unwrap();
    /// assert_eq!(
    ///     ns_name.name(),
    ///     Some(Name::from_str("Name").unwrap()),
    /// );
    ///
    /// let ns_name = Iri::from_str("https://example.org").unwrap();
    /// assert_eq!(
    ///     ns_name.namespace(),
    ///     None,
    /// );
    /// ```
    ///
    fn name(&self) -> Option<Name>
    where
        Self: Sized,
    {
        self.split().map(|(_, n)| n)
    }

    ///
    /// Assuming this IRI is a namespace, add the provided name.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra, Name};
    /// use std::str::FromStr;
    ///
    /// let namespace = Iri::from_str("https://example.org/ns/").unwrap();
    /// assert_eq!(
    ///     namespace.make_name(Name::from_str("Name").unwrap()).map(|s|s.to_string()),
    ///     Some("https://example.org/ns/Name".to_string()),
    /// );
    ///
    /// let namespace = Iri::from_str("https://example.org/ns#").unwrap();
    /// assert_eq!(
    ///     namespace.make_name(Name::from_str("Name").unwrap()).map(|s|s.to_string()),
    ///     Some("https://example.org/ns#Name".to_string()),
    /// );
    ///
    /// let namespace = Iri::from_str("https://example.org/ns").unwrap();
    /// assert_eq!(
    ///     namespace.make_name(Name::from_str("Name").unwrap()).map(|s|s.to_string()),
    ///     None,
    /// );
    /// ```
    ///
    fn make_name(&self, name: Name) -> Option<Self>
    where
        Self: Sized;

    ///
    /// Returns a new IRI with a well-known path of `"genid"` using the scheme and server
    /// components from `base`.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let base = Iri::from_str("https://example.org/path#fragment").unwrap();
    ///
    /// assert!(
    ///     base.genid().unwrap().to_string().starts_with(
    ///         "https://example.org/.well-known/genid/"
    ///     )
    /// );
    /// ```
    ///
    fn genid(&self) -> Result<Self, Error>
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Name
// ------------------------------------------------------------------------------------------------

///
/// This type represents the name component of an IRI used as a namespaced identifier.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(String);

///
/// This enum represents the different rules used to validate a string as a potential `Name`
/// value.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum NameParser {
    ///
    /// This rule uses the `NAME` production from XML 1.1, which is more restrictive than that
    /// allowed by other rules. However, restricting to this set for now allows guaranteed
    /// correctness for names in the XML representation, as such it is the default rule.
    ///
    #[default]
    Xml,
    ///
    /// Almost identical to the Xml parser, except that it allows ASCII digits as the first
    /// character in a name.
    ///
    BlankNode,
}

///
/// Denotes an error generated by the [`NameParser`]'s `parse_str` method.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NameParseError {
    ///
    /// An empty string was passed, this is not a valid name.
    ///
    EmptyString,
    ///
    /// The parser encountered a character which is not valid according to it's grammar. The
    /// particular parser generating the error is included.
    ///
    InvalidCharacter(NameParser),
    TooManySeparators,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ QName
// ------------------------------------------------------------------------------------------------

///
/// QNames are valid identifiers with an optional prefix identifier. e.g. "`xsd:integer`",
/// "`rdfs:Class`", "`:subPropertyOf`".
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QName {
    prefix: Option<Name>,
    name: Name,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ IRI
// ------------------------------------------------------------------------------------------------

impl IriExtra for Iri {
    fn with_new_path<S>(&self, path: S) -> Self
    where
        S: AsRef<str>,
    {
        let mut new_self = self.clone();
        new_self.set_path(path.as_ref());
        new_self
    }

    fn with_new_fragment<S>(&self, fragment: S) -> Self
    where
        S: AsRef<str>,
    {
        let mut new_self = self.clone();
        new_self.set_fragment(Some(fragment.as_ref()));
        new_self
    }

    fn with_empty_fragment(&self) -> Self {
        self.with_new_fragment("")
    }

    fn with_no_fragment(&self) -> Self {
        let mut new_self = self.clone();
        new_self.set_fragment(None);
        new_self
    }

    fn looks_like_namespace(&self) -> bool {
        self.fragment() == Some("") || (self.path().ends_with("/") && self.query().is_none())
    }

    fn split(&self) -> Option<(Self, Name)>
    where
        Self: Sized,
    {
        if self.fragment().map(|s| !s.is_empty()).unwrap_or_default() {
            if let Ok(name) = Name::from_str(self.fragment().unwrap()) {
                Some((self.with_empty_fragment(), name))
            } else {
                None
            }
        } else if !self.path().is_empty() && !self.path().ends_with("/") && self.query().is_none() {
            if let Ok(name) = Name::from_str(self.path_segments().unwrap().last().unwrap()) {
                let path = self.path();
                let path = &path[0..path.len() - name.as_ref().len()];
                Some((self.with_new_path(path), name))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn make_name(&self, name: Name) -> Option<Self>
    where
        Self: Sized,
    {
        if self.fragment().is_some() {
            Some(self.with_new_fragment(name.as_ref()))
        } else if self.path().ends_with("/") && self.query().is_none() {
            Some(self.with_new_path(format!("{}{}", self.path(), name.as_ref())))
        } else {
            None
        }
    }

    fn genid(&self) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let new_uuid = uuid::Uuid::new_v4();
        let new_uuid = new_uuid
            .simple()
            .encode_lower(&mut uuid::Uuid::encode_buffer())
            .to_string();
        let path = format!("/.well-known/genid/{new_uuid}");
        self.join(&path)
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
impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for Name {
    type Err = NameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_str(s, Default::default())
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
    /// Returns a new `Name` instance from the string `s` **without** any validation.
    ///
    pub fn new_unchecked<S>(s: S) -> Name
    where
        S: AsRef<str>,
    {
        Self(s.as_ref().to_string())
    }

    ///
    /// Returns `true` if the string in `s` is valid according to `parser`.
    ///
    pub fn is_valid_str<S>(s: S, parser: NameParser) -> bool
    where
        S: AsRef<str>,
    {
        parser.is_valid_str(s)
    }

    ///
    /// Returns a new `Name` instance, if the string in `s` is valid according to `parser`.
    ///
    pub fn parse_str<S>(s: S, rule: NameParser) -> Result<Self, NameParseError>
    where
        S: AsRef<str>,
    {
        rule.parse_str(s)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for NameParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Xml => "XML 1.1",
                Self::BlankNode => "RDF 1.1 BlankNode",
            }
        )
    }
}

impl NameParser {
    ///
    /// Returns `true` if the string in `s` is valid.
    ///
    pub fn is_valid_str<S>(&self, s: S) -> bool
    where
        S: AsRef<str>,
    {
        match self {
            NameParser::Xml => is_xml_name(s.as_ref(), false),
            NameParser::BlankNode => is_xml_name(s.as_ref(), true),
        }
    }

    ///
    /// Returns a new `Name` instance, if the string in `s` is valid.
    ///
    pub fn parse_str<S>(&self, s: S) -> Result<Name, NameParseError>
    where
        S: AsRef<str>,
    {
        if s.as_ref().is_empty() {
            Err(NameParseError::EmptyString)
        } else if s.as_ref().contains(':') {
            Err(NameParseError::TooManySeparators)
        } else if self.is_valid_str(&s) {
            Ok(Name(s.as_ref().to_string()))
        } else {
            Err(NameParseError::InvalidCharacter(*self))
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for NameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EmptyString => "An empty string is not a valid `Name`.".into(),
                Self::InvalidCharacter(rule) => format!(
                    "An invalid character (according to {rule}) is present in the `Name` string."
                ),
                Self::TooManySeparators => "Too many `:` characters in the `Name` string.".into(),
            }
        )
    }
}

impl std::error::Error for NameParseError {}

// ------------------------------------------------------------------------------------------------
// Implementations > QName
// ------------------------------------------------------------------------------------------------

const NAME_ONLY: usize = 1;
const PREFIX_AND_NAME: usize = 2;

impl Display for QName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}",
            if let Some(prefix) = &self.prefix {
                prefix.to_string()
            } else {
                String::new()
            },
            &self.name
        )
    }
}

impl FromStr for QName {
    type Err = NameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name_parser = NameParser::Xml;
        if s.is_empty() {
            Err(NameParseError::EmptyString)
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            match parts.len() {
                NAME_ONLY => {
                    let name = *parts.first().unwrap();
                    if Name::is_valid_str(name, name_parser) {
                        Ok(QName {
                            prefix: None,
                            name: Name::from_str(name)?,
                        })
                    } else {
                        Err(NameParseError::InvalidCharacter(name_parser))
                    }
                }
                PREFIX_AND_NAME => {
                    let prefix = *parts.first().unwrap();
                    let name = *parts.get(1).unwrap();
                    if Name::is_valid_str(prefix, name_parser)
                        && Name::is_valid_str(name, name_parser)
                    {
                        Ok(QName {
                            prefix: Some(Name::from_str(prefix)?),
                            name: Name::from_str(name)?,
                        })
                    } else {
                        Err(NameParseError::InvalidCharacter(name_parser))
                    }
                }
                _ => Err(NameParseError::TooManySeparators),
            }
        }
    }
}

impl From<QName> for String {
    fn from(value: QName) -> Self {
        value.to_string()
    }
}

impl From<&QName> for String {
    fn from(value: &QName) -> Self {
        value.to_string()
    }
}

const BLANK_NODE_NAMESPACE: &str = "_";

impl QName {
    /// Construct a new qualified QName: "`{prefix}:{name}`". This will return an error if either
    /// the prefix or name is empty or is an invalid QName part.
    pub fn new(prefix: Name, name: Name) -> Result<Self, NameParseError> {
        Ok(Self::new_unchecked(Some(prefix), name))
    }

    /// Construct a new unqualified QName: "`:{name}`". This will return an error if either
    /// the name is empty or is an invalid QName part.
    pub fn new_unqualified(name: Name) -> Result<Self, NameParseError> {
        Ok(Self::new_unchecked(None, name))
    }

    /// Construct a new QName **without** any validation checks on the given values.
    pub fn new_unchecked(prefix: Option<Name>, name: Name) -> Self {
        Self { prefix, name }
    }

    /// Construct a new blank node as a QName.
    pub fn new_blank(name: Name) -> Result<Self, NameParseError> {
        Ok(Self::new_unchecked(
            Some(Name::new_unchecked(BLANK_NODE_NAMESPACE)),
            name,
        ))
    }

    /// Returns `true` if this QName is a blank node, else `false`.
    pub fn is_blank(&self) -> bool {
        self.prefix
            .as_ref()
            .map(|p| p.as_ref() == BLANK_NODE_NAMESPACE)
            .is_some()
    }

    /// Returns `true` if this QName has a prefix, else `false`.
    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    /// Returns the prefix part of this QName, if present.
    pub fn prefix(&self) -> Option<&Name> {
        self.prefix.as_ref()
    }

    /// Returns the name part of this QName.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Format this QName as a Curie string.
    pub fn as_curie(&self) -> String {
        format!(
            "[{}:{}]",
            match &self.prefix {
                None => "",
                Some(prefix) => prefix.as_ref(),
            },
            &self.name
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn is_xml_name(s: &str, allow_initial_digit: bool) -> bool {
    !s.is_empty()
        && s.starts_with(|c| is_xml_name_start_char(c, allow_initial_digit))
        && s[1..]
            .chars()
            .all(|c| is_xml_name_char(c, allow_initial_digit))
}

fn is_xml_name_char(c: char, allow_initial_digit: bool) -> bool {
    is_xml_name_start_char(c, allow_initial_digit)
        || c == '-'
        || c == '.'
        || c.is_ascii_digit()
        || c == '\u{B7}'
        || ('\u{0300}'..='\u{036F}').contains(&c)
        || ('\u{203F}'..='\u{2040}').contains(&c)
}

fn is_xml_name_start_char(c: char, allow_initial_digit: bool) -> bool {
    c == ':'
        || (allow_initial_digit && c.is_ascii_digit())
        || c.is_ascii_uppercase()
        || c == '_'
        || c.is_ascii_lowercase()
        || ('\u{C0}'..='\u{D6}').contains(&c)
        || ('\u{D8}'..='\u{F6}').contains(&c)
        || ('\u{0F8}'..='\u{2FF}').contains(&c)
        || ('\u{370}'..='\u{37D}').contains(&c)
        || ('\u{037F}'..='\u{1FFF}').contains(&c)
        || ('\u{200C}'..='\u{200D}').contains(&c)
        || ('\u{2070}'..='\u{218F}').contains(&c)
        || ('\u{2C00}'..='\u{2FEF}').contains(&c)
        || ('\u{3001}'..='\u{D7FF}').contains(&c)
        || ('\u{F900}'..='\u{FDCF}').contains(&c)
        || ('\u{FDF0}'..='\u{FFFD}').contains(&c)
        || ('\u{10000}'..='\u{EFFFF}').contains(&c)
}
