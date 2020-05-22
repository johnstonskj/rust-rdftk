/*!
This crate provides a set of modules that contain the URIs and QName strings for commonly used
vocabularies. It also provides [macro](macro.namespace.html) support for defining new namespaces
in the same style as this library.

The vocabularies supported can be found [below](#modules).

# Macro Example

The following example replicates the `geo` module using the `namespace!` macro.

```rust,ignore
#[macro_use]
extern crate rdftk_names;

namespace! {
    "geo",
    "http://www.w3.org/2003/01/geo/wgs84_pos#",
    {
        spatial_thing, "SpatialThing",
        temporal_thing, "TemporalThing",
        event, "Event",
        point, "Point",
        lat, "lat",
        location, "location",
        long, "long",
        alt, "alt",
        lat_long, "lat_long"
    }
}
```

*/

#[allow(unused_imports)]
#[macro_use]
extern crate paste;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

///
/// This macro produces the constants and functions to describe a vocabulary. It creates the
/// following items.
///
/// 1. A constant, `PREFIX` that contains the string passed in the `$prefix` parameter.
/// 1. A constant, `NAMESPACE` that contains the string passed in the `namespace` parameter.
/// 1. For each pair of `$fn_name`, `$name` (assuming `foo` and `"Foo"`):
///    1. create a function `fn foo() -> IRI` that returns the name as a full IRI. This concatenates
///       `NAMESPACE` and `$name`.
///    1. create a function `fn foo_qname() -> String` that returns a qualified name String. This
///       concatenates the `PREFIX`, `":"`, and `$name`.
///
/// # Example
///
/// Given the following namespace invocation,
///
/// ```rust,ignore
/// #[macro_use]
/// extern crate rdftk_names;
///
/// namespace!("fb", "http://example.com/schema/FooBar#", { foo, "Foo" } );
/// ```
///
/// The folowing would be generated.
///
/// ```rust
/// use rdftk_iri::IRI;
/// use std::str::FromStr;
///
/// #[doc = "The default prefix for this namespace"]
/// pub const PREFIX: &str = "fb";
///
/// #[doc = "The URI for this namespace"]
/// pub const NAMESPACE: &str = "http://example.com/schema/FooBar#";
///
/// #[doc = "Construct a URI for this name"]
/// #[inline]
/// pub fn foo() -> IRI {
///     IRI::from_str(&format!("{}{}", NAMESPACE, "Foo")).unwrap()
/// }
///
/// #[doc = "Construct a prefixed qualified name String for this name"]
/// #[inline]
/// pub fn foo_qname() -> String {
///     format!("{}:{}", PREFIX, "Foo")
/// }
/// ```
///
#[macro_export]
macro_rules! namespace {
    ($prefix:expr, $namespace:expr, { $($fn_name:ident, $name:expr),* }) => {

    use rdftk_iri::IRI;
    use std::str::FromStr;

    #[doc = "The default prefix for this namespace"]
    pub const PREFIX: &str = $prefix;

    #[doc = "The URI for this namespace"]
    pub const NAMESPACE: &str = $namespace;

    $(
        nsname!($fn_name, $name);
    )*
    };
}

#[doc(hidden)]
macro_rules! nsname {
    ($fn_name:ident, $name:expr) => {
        #[doc = "Construct a URI for this name"]
        #[inline]
        pub fn $fn_name() -> IRI {
            IRI::from_str(&format!("{}{}", NAMESPACE, $name)).unwrap()
        }

        paste::item! {
        #[doc = "Construct a prefixed qualified name String for this name"]
        #[inline]
        pub fn [<$fn_name _qname>]() -> String {
            format!("{}:{}", PREFIX, $name)
        }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod dc;

pub mod foaf;

pub mod geo;

pub mod owl;

pub mod rdf;

pub mod rdfs;

pub mod xsd;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    namespace!("p", "heep://schema/com/p#", { foo, "Foo", bar, "Bar" } );

    #[test]
    fn test_expansion() {
        assert_eq!(foo().to_string(), "heep://schema/com/p#Foo");
        assert_eq!(foo_qname(), "p:Foo");
        assert_eq!(bar().to_string(), "heep://schema/com/p#Bar");
        assert_eq!(bar_qname(), "p:Bar");
    }
}
