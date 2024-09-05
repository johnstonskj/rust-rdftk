/*!
![iri](https://img.shields.io/badge/RDFtk-iri-BD1B89?logo=data:image/png;base64,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)
This crate provides an `Iri` and `IriRef` type.

## Examples

The [`Iri`] type is a wrapper around the `url::Url` type, and as can be constructed in the
same manner.

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
use rdftk_iri::{Iri, IriExtra as _};
use std::str::FromStr;

let namespace = Iri::from_str(
    "https://example.org/ns/things#",
).unwrap();
assert!(namespace.looks_like_namespace());

let name = namespace.make_name("ThisThing").unwrap();
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
    Some("ThisThing".to_string()),
);
```


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

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The common type for IRI values used throughout the RDFtk packages.
///
pub type Iri = url::Url;

///
/// The reference-counted type wrapping an `Iri`.
///
pub type IriRef = std::sync::Arc<Iri>;

///
/// Errors reported while parsing a string into an IRI.
///
pub type Error = url::ParseError;

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
    /// use rdftk_iri::{genid, Iri, IriExtra};
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
    /// use rdftk_iri::{genid, Iri, IriExtra};
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
    /// use rdftk_iri::{genid, Iri, IriExtra};
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
    /// use rdftk_iri::{genid, Iri, IriExtra};
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
    /// use rdftk_iri::{genid, Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let namespace = Iri::from_str("https://example.org/ns/Name").unwrap();
    /// assert_eq!(
    ///     namespace.split(),
    ///     Some((
    ///         Iri::from_str("https://example.org/ns/").unwrap(),
    ///         "Name".to_string(),
    ///     )),
    /// );
    ///
    /// let namespace = Iri::from_str("https://example.org/ns#Name").unwrap();
    /// assert_eq!(
    ///     namespace.split(),
    ///     Some((
    ///         Iri::from_str("https://example.org/ns#").unwrap(),
    ///         "Name".to_string(),
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
    fn split(&self) -> Option<(Self, String)>
    where
        Self: Sized;

    ///
    /// IF this IRI represents a namespaced-name, return the namespace part, else `None`.
    ///
    /// Example
    ///
    /// ```
    /// use rdftk_iri::{genid, Iri, IriExtra};
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
    /// use rdftk_iri::{genid, Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let ns_name = Iri::from_str("https://example.org/ns/Name").unwrap();
    /// assert_eq!(
    ///     ns_name.name(),
    ///     Some("Name".to_string()),
    /// );
    ///
    /// let ns_name = Iri::from_str("https://example.org/ns#Name").unwrap();
    /// assert_eq!(
    ///     ns_name.name(),
    ///     Some("Name".to_string()),
    /// );
    ///
    /// let ns_name = Iri::from_str("https://example.org").unwrap();
    /// assert_eq!(
    ///     ns_name.namespace(),
    ///     None,
    /// );
    /// ```
    ///
    fn name(&self) -> Option<String>
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
    /// use rdftk_iri::{genid, Iri, IriExtra};
    /// use std::str::FromStr;
    ///
    /// let namespace = Iri::from_str("https://example.org/ns/").unwrap();
    /// assert_eq!(
    ///     namespace.make_name("Name").map(|s|s.to_string()),
    ///     Some("https://example.org/ns/Name".to_string()),
    /// );
    ///
    /// let namespace = Iri::from_str("https://example.org/ns#").unwrap();
    /// assert_eq!(
    ///     namespace.make_name("Name").map(|s|s.to_string()),
    ///     Some("https://example.org/ns#Name".to_string()),
    /// );
    ///
    /// let namespace = Iri::from_str("https://example.org/ns").unwrap();
    /// assert_eq!(
    ///     namespace.make_name("Name").map(|s|s.to_string()),
    ///     None,
    /// );
    /// ```
    ///
    fn make_name<S>(&self, name: S) -> Option<Self>
    where
        S: AsRef<str>,
        Self: Sized;
}

///
/// Returns a new IRI with a well-known path of `"genid"` using the scheme and server
/// components from `base`.
///
/// Example
///
/// ```
/// use rdftk_iri::{genid, Iri, IriRef, IriExtra};
/// use std::str::FromStr;
///
/// let base: IriRef = Iri::from_str("https://example.org/path#fragment").unwrap().into();
///
/// assert!(
///     genid(&base).unwrap().to_string().starts_with(
///         "https://example.org/.well-known/genid/"
///     )
/// );
/// ```
///
pub fn genid(base: &IriRef) -> Result<IriRef, Error> {
    let new_uuid = uuid::Uuid::new_v4();
    let new_uuid = new_uuid
        .to_simple()
        .encode_lower(&mut uuid::Uuid::encode_buffer())
        .to_string();
    let path = format!("/.well-known/genid/{new_uuid}");
    Ok(IriRef::from(base.join(&path)?))
}

// ------------------------------------------------------------------------------------------------
// Implementations
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

    fn split(&self) -> Option<(Self, String)>
    where
        Self: Sized,
    {
        if self.fragment().map(|s| !s.is_empty()).unwrap_or_default() {
            let name = self.fragment().unwrap().to_string();
            Some((self.with_empty_fragment(), name))
        } else if !self.path().is_empty() && !self.path().ends_with("/") && self.query().is_none() {
            let name = self.path_segments().unwrap().last().unwrap();
            let path = self.path();
            let path = &path[0..path.len() - name.len()];
            Some((self.with_new_path(path), name.to_string()))
        } else {
            None
        }
    }

    fn make_name<S>(&self, name: S) -> Option<Self>
    where
        S: AsRef<str>,
        Self: Sized,
    {
        if self.fragment() == Some("") {
            Some(self.with_new_fragment(name.as_ref()))
        } else if self.path().ends_with("/") && self.query().is_none() {
            Some(self.with_new_path(&format!("{}{}", self.path(), name.as_ref())))
        } else {
            None
        }
    }
}
