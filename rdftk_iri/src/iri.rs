//!
//! This module provides the `IriRef` enum, `Iri` type, and `IriExtra` trait.
//!

#[cfg(not(feature = "std"))]
use alloc::format;

use crate::{Name, pname::PrefixedName};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use strum::{EnumIs, EnumTryAs};

#[cfg(feature = "genid")]
use crate::error::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ IRIRef
// ------------------------------------------------------------------------------------------------

///
/// This type is used where either a full IRI (`<...>`) is supplied, or a prefixed name (`pre:name`)
/// is used.
///
/// ## Specification
///
/// ```text
/// [67]  	IRIref	  ::=  	IRI_REF | PrefixedName
/// ```
///
/// For `IRI_REF` see [`Iri`], and for `PrefixedName` see [`PrefixedName`].
///
#[derive(Clone, Debug, PartialEq, Eq, EnumIs, EnumTryAs)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[allow(missing_docs)] // EnumIs/EnumTryAs generate undocumented methods.
pub enum IriRef {
    /// A full IRI of the form `<...>`.
    Iri(Iri),
    /// A prefixed name of the form `prefix:local`.
    PrefixedName(PrefixedName),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ IRI
// ------------------------------------------------------------------------------------------------

///
/// The common type for IRI values used throughout the RDFtk packages.
///
pub type Iri = url::Url;

///
/// Additional, mainly constructor, functions for the [`Iri`] type.
///
pub trait IriExtra {
    ///
    /// Returns a copy of the current IRI with the path component replaced by `path`.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let iri = Iri::from_str("https://example.org/old").unwrap();
    /// assert_eq!(
    ///     iri.with_new_path("/new").to_string(),
    ///     "https://example.org/new",
    /// );
    /// ```
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
    /// Returns a copy of the current IRI with the fragment component replaced
    /// by an empty string, so the IRI ends with `#`.
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
    ///
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
    ///     ns_name.name(),
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
    #[cfg(feature = "genid")]
    fn genid(&self) -> Result<Self, Error>
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ IriRef
// ------------------------------------------------------------------------------------------------

impl Display for IriRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Iri(v) => v.fmt(f),
            Self::PrefixedName(v) => v.fmt(f),
        }
    }
}

impl From<Iri> for IriRef {
    fn from(value: Iri) -> Self {
        Self::Iri(value)
    }
}

impl From<&Iri> for IriRef {
    fn from(value: &Iri) -> Self {
        Self::from(value.clone())
    }
}

impl From<PrefixedName> for IriRef {
    fn from(value: PrefixedName) -> Self {
        Self::PrefixedName(value)
    }
}

impl From<&PrefixedName> for IriRef {
    fn from(value: &PrefixedName) -> Self {
        Self::from(value.clone())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Iri
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

    #[cfg(feature = "genid")]
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
        self.join(&path).map_err(|e| e.into())
    }
}
