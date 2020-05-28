/*!
Another implementation of the `IRI` and URI specifications. It provides an `IRI` type that supports
the semantics of the [IRI](https://en.wikipedia.org/wiki/Internationalized_Resource_Identifier),
[URI](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier),
[URL](https://en.wikipedia.org/wiki/URL), and
[URN](https://en.wikipedia.org/wiki/Uniform_Resource_Name) specifications.

# Example

TBD

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

#[macro_use]
extern crate error_chain;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Normalize {
    fn normalize(self) -> error::Result<Self>
    where
        Self: Sized;
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod parse;

pub mod error;

pub mod builder;

pub mod scheme;
pub use scheme::{KnownSchemes, Scheme};

pub mod authority;
pub use authority::{Authority, Host, KnownPorts, Port};

pub mod path;
pub use path::Path;

pub mod query;
pub use query::{Query, QueryPart};

pub mod fragment;
pub use fragment::Fragment;

#[allow(clippy::module_inception)]
pub mod iri;
pub use iri::IRI;
