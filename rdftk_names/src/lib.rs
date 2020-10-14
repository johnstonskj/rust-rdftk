/*!
This crate provides a set of modules that contain the IRIs and QName strings for commonly used
vocabularies. It also provides [macro](macro.namespace.html) support for defining new namespaces
in the same style as this library.

The vocabularies supported can be found [below](#modules).

# Macro Example

The following example replicates the `geo` module using the `namespace!` macro. Note that as this
macro uses `paste::item` the client will need to have a dependency on the
[paste crate](https://crates.io/crates/paste).


```rust,ignore
#[macro_use]
extern crate paste;

#[macro_use]
extern crate rdftk_names;

use rdftk_names::Vocabulary;

namespace! {
    GeoSpatialVocabulary,
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
extern crate lazy_static;

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
/// 1. An identifier for the vocabulary struct.
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
/// namespace!(FooBarVocab, "fb", "http://example.com/schema/FooBar#", { foo, "Foo" } );
/// ```
///
/// The following would be generated.
///
/// ```rust, ignore
/// use rdftk_iri::IRI;
/// use std::str::FromStr;
///
/// pub struct FooBarVocab { ... }
///
/// impl Default for FooBarVocab { ... }
///
/// impl Vocabulary for FooBarVocab { ... }
///
/// impl FooBarVocab {
///     pub fn foo(&self) -> &Arc<IRI> { ... }
///     pub foo_qname(&self) -> &String { ... }
/// }
/// ```
///
#[macro_export]
macro_rules! namespace {
    ($prefix:expr, $namespace:expr, { $($fn_name:ident, $name:expr),* }) => {

    use rdftk_iri::{IRI, IRIRef};
    use std::str::FromStr;
    use std::collections::HashMap;

    const PREFIX: &str = $prefix;

    const NAMESPACE: &str = $namespace;

    lazy_static! {
        static ref NS_IRI: IRIRef = IRIRef::new(NAMESPACE.parse().unwrap());
        static ref NS_CACHE: HashMap<String, (IRIRef, String)> = make_cache();
    }

    fn make_cache() -> HashMap<String, (IRIRef, String)> {
        let mut cache: HashMap<String, (IRIRef, String)> = Default::default();
        $(
        cache.insert($name.to_string(), (
            IRIRef::new(IRI::from_str(&format!("{}{}", NAMESPACE, $name)).unwrap()),
            format!("{}:{}", PREFIX, $name),
            )
        );
        )*
        cache
    }

        #[inline]
    pub fn default_prefix() -> &'static str { PREFIX }

        #[inline]
    pub fn namespace_str() -> &'static str { NAMESPACE }

        #[inline]
    pub fn namespace_iri() -> &'static IRIRef { &NS_IRI }

    $(
        nsname!($fn_name, $name);
    )*
    };
}

///
/// This macro *should only* called by the `namespace!` macro. It takes an identifier and a
/// string and produces:
///
/// 1. a function with the same identifier which returns a complete IRI using the
///    value of `NAMESPACE` in the current scope, and
/// 1. a function with the same identifier, but the suffix `_qname` which returns a qualified name
///    using the value of `PREFIX` in the current scope.
///

#[macro_export]
macro_rules! nsname {
    ($fn_name:ident, $name:expr) => {
        #[inline]
        pub fn $fn_name() -> &'static IRIRef {
            &NS_CACHE.get($name).unwrap().0
        }

        paste::item! {
        #[inline]
        pub fn [<$fn_name _qname>]() -> &'static String {
            &NS_CACHE.get($name).unwrap().1
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
