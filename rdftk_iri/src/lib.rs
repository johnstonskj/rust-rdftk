/*!
![iri](https://img.shields.io/badge/RDFtk-iri-BD1B89?logo=data:image/png;base64,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)
This crate provides an implementation of the `IRI` and `URI` specifications. It provides [`IRI`](struct.IRI.html) and
[`IRIRef`](type.IRIRef.html) types that supports the semantics of the
[IRI](https://en.wikipedia.org/wiki/Internationalized_Resource_Identifier),
[URI](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier),
[URL](https://en.wikipedia.org/wiki/URL), and
[URN](https://en.wikipedia.org/wiki/Uniform_Resource_Name) specifications.

# Examples

The most common use is the parsing of an [`IRI`](struct.IRI.html) value from a string.

```rust
use rdftk_iri::IRI;
use std::str::FromStr;

let result = IRI::from_str(
    "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
);
```

Once parsed it is easy to then extract the components of the [`IRI`](struct.IRI.html), as shown below.

```rust
use rdftk_iri::IRI;
use std::str::FromStr;

let result = IRI::from_str(
    "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
);

let iri = result.unwrap();

println!("scheme:   {}", iri.scheme().as_ref().unwrap());
println!("user:     {}", iri.authority().as_ref().unwrap().user_info().as_ref().unwrap().user_name());
println!("host:     {}", iri.authority().as_ref().unwrap().host());
println!("port:     {}", iri.authority().as_ref().unwrap().port().as_ref().unwrap());
println!("path:     {}", iri.path());
println!("query:    {}", iri.query().as_ref().unwrap());
println!("fragment: {}", iri.fragment().as_ref().unwrap());
```

The previous code should result in the following:

```text
scheme:   https
user:     john.doe
host:     www.example.com
port:     123
path:     /forum/questions/
query:    tag=networking&order=newest
fragment: top
```

The `builder` module allows for more programmatic construction of [`IRI`](struct.IRI.html)s.

```rust
use rdftk_iri::{IRI, Scheme};
use rdftk_iri::builder::IriBuilder;
use rdftk_iri::error::Result as IriResult;
use std::convert::TryInto;

# fn main() -> rdftk_iri::error::Result<()> {
let mut builder = IriBuilder::default();
let result: IriResult<IRI> = builder
    .scheme(&Scheme::https())
    .user_name("john.doe")
    .host_str("www.example.com")?
    .port(123.into())
    .path_str("/forum/questions/")?
    .query_str("tag=networking&order=newest")?
    .fragment_str("top")?
    .try_into();
# Ok(())
# }
```

Note also the use of `Scheme::https()`, both the [`Scheme`](struct.Scheme.html) and
[`Port`](struct.Port.html) types include associated functions to construct well-known values.

# Features

The following features are present in this crate.

`builder` [default] -- include the [`builder`](builder/index.html) module, which in turn includes
   the [`IriBuilder`](builder/struct.IriBuilder.html) type.
`path_iri` [default] -- provides an implementation of `TryFrom<&PathBuf>` and `TryFrom<PathBuf>`
  for `IRI`.
`uuid_iri` [default] -- provides an implementation of `TryFrom<&Uuid>` and `TryFrom<Uuid>`
  for `IRI`.

# Specifications

1. RFC-1630 [Universal Resource Identifiers in WWW](https://tools.ietf.org/html/rfc1630): A Unifying Syntax
   for the Expression of Names and Addresses of Objects on the Network as used in the World-Wide Web
1. RFC-1736 [Functional Recommendations for Internet Resource Locators](https://tools.ietf.org/html/rfc1736)
1. RFC-1737 [Functional Requirements for Uniform Resource Names](https://tools.ietf.org/html/rfc1737)
1. RFC-1738 [Uniform Resource Locators (URL)](https://tools.ietf.org/html/rfc1738)
1. RFC-1808 [Relative Uniform Resource Locators](https://tools.ietf.org/html/rfc1808)
1. RFC-2141 [URN Syntax](https://tools.ietf.org/html/rfc2141)
1. RFC-2396 [Uniform Resource Identifiers (URI): Generic Syntax](https://tools.ietf.org/html/rfc2396)
1. RFC-2616 [Hypertext Transfer Protocol -- HTTP/1.1](https://tools.ietf.org/html/rfc2616); §3.2 Uniform
   Resource Identifiers
1. RFC-2717 [Registration Procedures for URL Scheme Names](https://tools.ietf.org/html/rfc2717)
1. RFC-2732 [Format for Literal IPv6 Addresses in URL's](https://tools.ietf.org/html/rfc2732)
1. RFC-3305 Report from the Joint W3C/IETF URI Planning Interest Group: Uniform Resource Identifiers (URIs),
   URLs, and Uniform Resource Names (URNs): [Clarifications and Recommendations](https://tools.ietf.org/html/rfc3305)
1. RFC-3987 [Internationalized Resource Identifiers (IRIs)](https://tools.ietf.org/html/rfc3987)
1. RFC-6963 [A Uniform Resource Name (URN) Namespace for Examples](https://tools.ietf.org/html/rfc6963)
1. RFC-8141 [Uniform Resource Names (URNs)](https://tools.ietf.org/html/rfc8141)

From RFC-2396, appendix A. _Collected BNF for URI_:

```text
URI-reference = [ absoluteURI | relativeURI ] [ "#" fragment ]
absoluteURI   = scheme ":" ( hier_part | opaque_part )
relativeURI   = ( net_path | abs_path | rel_path ) [ "?" query ]

hier_part     = ( net_path | abs_path ) [ "?" query ]
opaque_part   = uric_no_slash *uric

uric_no_slash = unreserved | escaped | ";" | "?" | ":" | "@" |
                "&" | "=" | "+" | "$" | ","

net_path      = "//" authority [ abs_path ]
abs_path      = "/"  path_segments
rel_path      = rel_segment [ abs_path ]

rel_segment   = 1*( unreserved | escaped |
                    ";" | "@" | "&" | "=" | "+" | "$" | "," )

scheme        = alpha *( alpha | digit | "+" | "-" | "." )

authority     = server | reg_name

reg_name      = 1*( unreserved | escaped | "$" | "," |
                    ";" | ":" | "@" | "&" | "=" | "+" )

server        = [ [ userinfo "@" ] hostport ]
userinfo      = *( unreserved | escaped |
                   ";" | ":" | "&" | "=" | "+" | "$" | "," )

hostport      = host [ ":" port ]
host          = hostname | IPv4address
hostname      = *( domainlabel "." ) toplabel [ "." ]
domainlabel   = alphanum | alphanum *( alphanum | "-" ) alphanum
toplabel      = alpha | alpha *( alphanum | "-" ) alphanum
IPv4address   = 1*digit "." 1*digit "." 1*digit "." 1*digit
port          = *digit

path          = [ abs_path | opaque_part ]
path_segments = segment *( "/" segment )
segment       = *pchar *( ";" param )
param         = *pchar
pchar         = unreserved | escaped |
                ":" | "@" | "&" | "=" | "+" | "$" | ","

query         = *uric

fragment      = *uric
uric          = reserved | unreserved | escaped
reserved      = ";" | "/" | "?" | ":" | "@" | "&" | "=" | "+" |
                "$" | ","
unreserved    = alphanum | mark
mark          = "-" | "_" | "." | "!" | "~" | "*" | "'" |
                "(" | ")"

escaped       = "%" hex hex
hex           = digit | "A" | "B" | "C" | "D" | "E" | "F" |
                        "a" | "b" | "c" | "d" | "e" | "f"

alphanum      = alpha | digit
alpha         = lowalpha | upalpha

lowalpha = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" |
           "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" |
           "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
upalpha  = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" |
           "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" |
           "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"
digit    = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" |
           "8" | "9"
```

Also, _Excluded US-ASCII Characters_:

```text
control  = <US-ASCII coded characters 00-1F and 7F hexadecimal>
space    = <US-ASCII coded character 20 hexadecimal>
delims   = "<" | ">" | "#" | "%" | <">
unwise   = "{" | "}" | "|" | "\" | "^" | "[" | "]" | "`"
```

To support IPv6 addresses the following changes were made in RFC-2732:

```text
The following changes to the syntax in RFC 2396 are made:
(1) change the 'host' non-terminal to add an IPv6 option:

   host          = hostname | IPv4address | IPv6reference
   ipv6reference = "[" IPv6address "]"

where IPv6address is defined as in RFC2373 [ARCH].

(2) Replace the definition of 'IPv4address' with that of RFC 2373, as
it correctly defines an IPv4address as consisting of at most three
decimal digits per segment.

(3) Add "[" and "]" to the set of 'reserved' characters:

   reserved    = ";" | "/" | "?" | ":" | "@" | "&" | "=" | "+" |
                 "$" | "," | "[" | "]"

and remove them from the 'unwise' set:

   unwise      = "{" | "}" | "|" | "\" | "^" | "`"
```

*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait is used on the [`IRI`](struct.IRI.html) and it's components to normalize their value
/// according to the relevant RFC rules.
///
/// 1. Normalization will ensure the correct case of certain components.
/// 1. Normalization will removing any unnecessary "." and ".." segments from the path component of
///    a hierarchical URI.
///
pub trait Normalize {
    /// Return a normalized version of `self`. The default for normalization is to do nothing and
    /// return `self` unchanged.
    fn normalize(self) -> error::Result<Self>
    where
        Self: Sized,
    {
        Ok(self)
    }
}

///
/// This trait is implemented by most components to provide a way to determine whether a string
/// value is valid. It can be assumed that the action is less expensive than performing the
/// [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) conversion and checking it's
/// result.
///
pub trait ValidateStr: FromStr {
    /// Return `true` if the string is a valid representation of `Self`, else `false`.
    fn is_valid(s: &str) -> bool {
        Self::from_str(s).is_ok()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod parse;

// ------------------------------------------------------------------------------------------------

pub mod error;

#[cfg(feature = "builder")]
pub mod builder;

#[doc(hidden)]
pub mod scheme;
pub use scheme::Scheme;

#[doc(hidden)]
pub mod authority;
pub use authority::{Authority, Host, HostKind, Port, UserInfo};

#[doc(hidden)]
pub mod path;
pub use path::Path;

#[doc(hidden)]
pub mod query;
pub use query::Query;

#[doc(hidden)]
pub mod fragment;
pub use fragment::Fragment;

#[allow(clippy::module_inception)]
#[doc(hidden)]
pub mod iri;
pub use iri::{IRIRef, IRI};
use std::str::FromStr;
