/*!
Provides the `URI` type that supports the semantics of the
[URI](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier) and
[IRI](https://en.wikipedia.org/wiki/Internationalized_Resource_Identifier) specifications, and
which includes both [URL](https://en.wikipedia.org/wiki/URL)s as well as
[URN](https://en.wikipedia.org/wiki/Uniform_Resource_Name)s.

In general, URLs are written as follows:

```text
    <scheme>:<scheme-specific-part>
```

# Example

# Specification

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

use crate::error::{Error as UriError, ErrorKind, Result as UriResult};
use crate::{Authority, Fragment, Normalize, Path, Query, Scheme};
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IRI {
    scheme: Option<Scheme>,
    authority: Option<Authority>,
    path: Path,
    query: Option<Query>,
    fragment: Option<Fragment>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for IRI {
    fn default() -> Self {
        Self::new(&Path::default())
    }
}

impl Display for IRI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            match &self.scheme {
                None => String::new(),
                Some(scheme) => scheme.to_string(),
            },
            match &self.authority {
                None => String::new(),
                Some(authority) => authority.to_string(),
            },
            &self.path.to_string(),
            match &self.query {
                None => String::new(),
                Some(query) => query.to_string(),
            },
            match &self.fragment {
                None => String::new(),
                Some(fragment) => fragment.to_string(),
            },
        )
    }
}

impl FromStr for IRI {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_uri(s)
    }
}

impl Normalize for IRI {
    fn normalize(self) -> UriResult<Self> {
        // TODO: need to address this!!
        Ok(Self {
            scheme: match self.scheme {
                None => None,
                Some(scheme) => Some(scheme.normalize()?),
            },
            authority: match self.authority {
                None => None,
                Some(authority) => Some(authority.normalize()?),
            },
            path: self.path.normalize()?,
            query: match self.query {
                None => None,
                Some(query) => Some(query.normalize()?),
            },
            fragment: match self.fragment {
                None => None,
                Some(fragment) => Some(fragment.normalize()?),
            },
        })
    }
}

impl IRI {
    pub fn new(path: &Path) -> Self {
        Self {
            scheme: None,
            authority: None,
            path: path.clone(),
            query: None,
            fragment: None,
        }
    }

    pub fn new_iri(path: &Path) -> Self {
        Self {
            scheme: None,
            authority: None,
            path: path.clone(),
            query: None,
            fragment: None,
        }
    }

    pub fn has_scheme(&self) -> bool {
        self.scheme.is_some()
    }

    pub fn scheme(&self) -> &Option<Scheme> {
        &self.scheme
    }

    pub fn has_authority(&self) -> bool {
        self.authority.is_some()
    }

    pub fn authority(&self) -> &Option<Authority> {
        &self.authority
    }

    pub fn has_path(&self) -> bool {
        !self.path.is_empty()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    pub fn query(&self) -> &Option<Query> {
        &self.query
    }

    pub fn has_fragment(&self) -> bool {
        self.fragment.is_some()
    }

    pub fn fragment(&self) -> &Option<Fragment> {
        &self.fragment
    }

    pub fn set_scheme(&mut self, scheme: Option<Scheme>) {
        self.scheme = scheme;
    }

    pub fn set_authority(&mut self, authority: Option<Authority>) {
        self.authority = authority;
    }

    pub fn set_path(&mut self, path: Path) {
        self.path = path;
    }

    pub fn set_query(&mut self, query: Option<Query>) {
        self.query = query;
    }

    pub fn set_fragment(&mut self, fragment: Option<Fragment>) {
        self.fragment = fragment;
    }

    pub fn with_new_path(&self, path: Path) -> Self {
        Self {
            path,
            ..self.clone()
        }
    }

    pub fn with_new_query(&self, query: Option<Query>) -> Self {
        Self {
            query,
            ..self.clone()
        }
    }

    pub fn with_new_fragment(&self, fragment: Option<Fragment>) -> Self {
        Self {
            fragment,
            ..self.clone()
        }
    }

    pub fn to_absolute(&self) -> Self {
        if self.fragment.is_some() {
            Self {
                fragment: None,
                ..self.clone()
            }
        } else {
            self.clone()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const GRP_SCHEME: usize = 2;
const GRP_AUTHORITY: usize = 4;
const GRP_PATH: usize = 5;
const GRP_QUERY: usize = 7;
const GRP_FRAGMENT: usize = 9;

fn parse_uri(s: &str) -> UriResult<IRI> {
    // From RFC-2396, appendix B. Parsing a URI Reference with a Regular Expression
    let regex = Regex::new(r"^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?").unwrap();
    match regex.captures(s) {
        Some(captures) => Ok(IRI {
            scheme: match captures.get(GRP_SCHEME) {
                None => None,
                Some(grp) => Some(Scheme::from_str(grp.as_str())?),
            },
            authority: match captures.get(GRP_AUTHORITY) {
                None => None,
                Some(grp) => Some(Authority::from_str(grp.as_str())?),
            },
            path: match captures.get(GRP_PATH) {
                None => Path::default(),
                Some(grp) => Path::from_str(grp.as_str())?,
            },
            query: match captures.get(GRP_QUERY) {
                None => None,
                Some(grp) => Some(Query::from_str(grp.as_str())?),
            },
            fragment: match captures.get(GRP_FRAGMENT) {
                None => None,
                Some(grp) => Some(Fragment::from_str(grp.as_str())?),
            },
        }),
        None => Err(ErrorKind::Syntax(s.to_string()).into()),
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uri_simple() {
        let result = parse_uri(
            "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
        );
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_uri("ldap://[2001:db8::7]/c=GB?objectClass?one");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_uri("mailto:John.Doe@example.com");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_uri("news:comp.infosystems.www.servers.unix");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_uri("tel:+1-816-555-1212");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_uri("telnet://192.0.2.16:80/");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_uri("urn:oasis:names:specification:docbook:dtd:xml:4.1.2");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_uri("https://en.wiktionary.org/wiki/Ῥόδος");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_uri("http://www.myfictionαlbank.com/");
        assert!(result.is_ok());
        println!("{:#?}", result);
    }
}
