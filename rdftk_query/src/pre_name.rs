/*!
An implementation of the Prefixed Name used to compress IRIs. This encompasses the `PrefixedName`
production in the [SPARQL 1.1 specification](https://www.w3.org/TR/sparql11-query/#rPrefixedName).
which is, in turn, used in the [Turtle](https://www.w3.org/TR/turtle/) language.

Specifically, within the Turtle specification, it notes:

> Prefixed names are a superset of XML QNames. They differ in that the local part of prefixed names
> may include:
>
> * leading digits, e.g. `leg:3032571` or `isbn13:9780136019701`
> * non leading colons, e.g. `og:video:height`
> * reserved character escape sequences, e.g. `wgs:lat\-long`

# Example

TBD

# Specifications

1. https://www.w3.org/TR/sparql11-query/#rPrefixedName
2. https://www.w3.org/TR/turtle/#grammar-production-PrefixedName
3. https://www.w3.org/2001/tag/doc/qnameids
4. https://www.w3.org/TR/REC-xml-names/
5. https://www.w3.org/TR/curie

From (1):

```text
[137]  PrefixedName     ::=  PNAME_LN | PNAME_NS

[140]  PNAME_NS         ::=  PN_PREFIX? ':'
[141]  PNAME_LN         ::=  PNAME_NS PN_LOCAL

[164]  PN_CHARS_BASE    ::=  [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6] | [#x00F8-#x02FF]
                             | [#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F]
                             | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD]
                             | [#x10000-#xEFFFF]
[165]  PN_CHARS_U       ::=  PN_CHARS_BASE | '_'

[167]  PN_CHARS         ::=  PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]
[168]  PN_PREFIX        ::=  PN_CHARS_BASE ((PN_CHARS | '.')* PN_CHARS)?
[169]  PN_LOCAL         ::=  (PN_CHARS_U | ':' | [0-9] | PLX )
                             ((PN_CHARS | '.' | ':' | PLX)* (PN_CHARS | ':' | PLX) )?
[170]  PLX              ::=  PERCENT | PN_LOCAL_ESC
[171]  PERCENT          ::=  '%' HEX HEX
[172]  HEX              ::=  [0-9] | [A-F] | [a-f]
[173]  PN_LOCAL_ESC     ::=  '\' ( '_' | '~' | '.' | '-' | '!' | '$' | '&' | "'" | '(' | ')'
                                 | '*' | '+' | ',' | ';' | '=' | '/' | '?' | '#' | '@' | '%' )
```

As noted above, the `PrefixedName` production is s superset of the XML `QName` which has a restricted
character set. A `PrefixedName` may be converted into a `QName` string with the `as_qname` method.
From (4):

```text

[1]    NSAttName        ::=  PrefixedAttName | DefaultAttName
[2]    PrefixedAttName  ::=  'xmlns:' NCName  [NSC: Reserved Prefixes and Namespace Names]
[3]    DefaultAttName   ::=  'xmlns'
[4]    NCName           ::=  Name - (Char* ':' Char*)  /* An XML Name, minus the ":" */

[7]    QName            ::=  PrefixedName | UnprefixedName
[8]    PrefixedName     ::=  Prefix ':' LocalPart
[9]    UnprefixedName   ::=  LocalPart
[10]   Prefix           ::=  NCName
[11]   LocalPart        ::=  NCName
```

This also relates to the [CURIE](https://www.w3.org/TR/curie) specification as the `curie`
production refers to the XML: `NCName` production. A `PrefixedName` may be converted into a safe
CURIE string with the `as_curie` method. From (5):

```text
safe_curie  :=   '[' curie ']'
curie       :=   [ [ prefix ] ':' ] reference
prefix      :=   NCName
reference   :=   irelative-ref (as defined in IRI)
```

*/

use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Prefix {
    Default,
    Some(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrefixedName {
    prefix: Prefix,
    name: String,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Prefix {
    fn default() -> Self {
        Self::Default
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Prefix::Default => String::new(),
                Prefix::Some(prefix) => prefix.to_owned(),
            }
        )
    }
}

impl FromStr for Prefix {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Prefix {
    pub fn new(prefix: &str) -> IriResult<Self> {
        if prefix.is_empty() {
            Err(ErrorKind::EmptyPrefixedName.into())
        } else if !Self::is_valid(prefix) {
            Err(ErrorKind::InvalidPrefixedName(prefix.to_string()).into())
        } else {
            Ok(Self::Some(prefix.to_string()))
        }
    }

    pub fn is_default(&self) -> bool {
        matches!(*self, Prefix::Default)
    }

    pub fn is_some(&self) -> bool {
        matches!(*self, Prefix::Some(_))
    }

    pub fn contains(&self, p: &str) -> bool {
        match self {
            Prefix::Default => false,
            Prefix::Some(prefix) => prefix == p,
        }
    }

    pub fn as_ref(&self) -> Option<&String> {
        match *self {
            Prefix::Some(ref x) => Some(x),
            Prefix::Default => None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut String> {
        match *self {
            Prefix::Some(ref mut x) => Some(x),
            Prefix::Default => None,
        }
    }

    pub fn unwrap(self) -> String {
        match self {
            Prefix::Some(val) => val,
            Prefix::Default => panic!("called `Prefix::unwrap()` on a `Default` value"),
        }
    }

    pub fn is_valid(prefix: &str) -> bool {
        true
    }
}

// ------------------------------------------------------------------------------------------------

const NAME_ONLY: usize = 1;
const PREFIX_AND_NAME: usize = 2;

impl Display for PrefixedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}",
            match &self.prefix {
                Prefix::Default => String::new(),
                Prefix::Some(prefix) => prefix.to_owned(),
            },
            &self.name
        )
    }
}

impl FromStr for PrefixedName {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ErrorKind::EmptyPrefixedName.into())
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            match parts.len() {
                NAME_ONLY => {
                    let name = *parts.get(0).unwrap();
                    if is_xml_name(name) {
                        Ok(PrefixedName {
                            prefix: Default::default(),
                            name: name.to_string(),
                        })
                    } else {
                        Err(ErrorKind::InvalidPrefixedName(s.to_string()).into())
                    }
                }
                PREFIX_AND_NAME => {
                    let prefix = *parts.get(0).unwrap();
                    let name = *parts.get(1).unwrap();
                    if is_xml_ncname(prefix) && is_xml_ncname(name) {
                        Ok(PrefixedName {
                            prefix: Prefix::Some(prefix.to_string()),
                            name: name.to_string(),
                        })
                    } else {
                        Err(ErrorKind::InvalidPrefixedName(s.to_string()).into())
                    }
                }
                _ => Err(ErrorKind::InvalidPrefixedName(s.to_string()).into()),
            }
        }
    }
}

impl ValidateStr for PrefixedName {}

impl PrefixedName {
    pub fn new(name: &str) -> Self {
        assert!(is_xml_name(name));
        Self {
            prefix: Default::default(),
            name: name.to_string(),
        }
    }

    pub fn with_prefix(prefix: &str, name: &str) -> Self {
        assert!(is_xml_name(prefix));
        assert!(is_xml_name(name));
        Self {
            prefix: Prefix::Some(prefix.to_string()),
            name: name.to_string(),
        }
    }

    pub fn prefix(&self) -> &Prefix {
        &self.prefix
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn as_curie(&self) -> Option<String> {
        None
    }

    pub fn as_qname(&self) -> Option<String> {
        None
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn is_sparql_prefixed_name(s: &str) -> bool {}

fn is_sparql_pname_ns(s: &str) -> bool {}

fn is_sparql_pname_ln(s: &str) -> bool {}

pub fn is_xml_name_start_char(c: char) -> bool {
    c == ':'
        || (c >= 'A' && c <= 'Z')
        || c == '_'
        || (c >= 'a' && c <= 'z')
        || (c >= '\u{C0}' && c <= '\u{D6}')
        || (c >= '\u{D8}' && c <= '\u{F6}')
        || (c >= '\u{0F8}' && c <= '\u{2FF}')
        || (c >= '\u{370}' && c <= '\u{37D}')
        || (c >= '\u{037F}' && c <= '\u{1FFF}')
        || (c >= '\u{200C}' && c <= '\u{200D}')
        || (c >= '\u{2070}' && c <= '\u{218F}')
        || (c >= '\u{2C00}' && c <= '\u{2FEF}')
        || (c >= '\u{3001}' && c <= '\u{D7FF}')
        || (c >= '\u{F900}' && c <= '\u{FDCF}')
        || (c >= '\u{FDF0}' && c <= '\u{FFFD}')
        || (c >= '\u{10000}' && c <= '\u{EFFFF}')
}

fn is_xml_name_char(c: char) -> bool {
    is_xml_name_start_char(c)
        || c == '-'
        || c == '.'
        || (c >= '0' && c <= '9')
        || c == '\u{B7}'
        || (c >= '\u{0300}' && c <= '\u{036F}')
        || (c >= '\u{203F}' && c <= '\u{2040}')
}

fn is_xml_ncname(s: &str) -> bool {
    !s.is_empty()
        && s.starts_with(is_xml_name_start_char)
        && s[1..].chars().all(|c| is_xml_name_char(c) && c == ':')
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_qname() {
        let pre_name = PrefixedName::new("foo");
        assert_eq!(pre_name.to_string(), "foo".to_string());
        assert_eq!(pre_name.as_curie(), Some("[:foo]".to_string()));
        assert_eq!(pre_name.as_qname(), Some("foo".to_string()));

        let pre_name = PrefixedName::with_prefix("rdf", "foo");
        assert_eq!(pre_name.to_string(), "rdf:foo".to_string());
        assert_eq!(pre_name.as_curie(), Some("[rdf:foo]".to_string()));
        assert_eq!(pre_name.as_qname(), Some("rdf:foo".to_string()));
    }

    #[test]
    fn test_qname_from_str() {
        let pre_name = PrefixedName::from_str("foo");
        assert!(pre_name.is_ok());
        assert_eq!(pre_name.unwrap().to_string(), "foo".to_string());

        let pre_name = PrefixedName::from_str("rdf:foo");
        assert!(pre_name.is_ok());
        assert_eq!(pre_name.unwrap().to_string(), "rdf:foo".to_string());
    }

    #[test]
    fn test_qname_from_str_fail() {
        let pre_name = PrefixedName::from_str("");
        assert!(pre_name.is_err());

        let pre_name = PrefixedName::from_str("rdf foo");
        assert!(pre_name.is_err());

        let pre_name = PrefixedName::from_str(":foo");
        assert!(pre_name.is_err());

        let pre_name = PrefixedName::from_str("rdf::foo:bar");
        assert!(pre_name.is_err());
    }
}
