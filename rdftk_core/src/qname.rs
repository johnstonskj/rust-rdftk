/*!
A qualified name, `QName` implementation.

# Example

# Specification

1. https://www.w3.org/TR/REC-xml-names/
2. https://www.w3.org/TR/REC-xml/#NT-Name
3. https://www.w3.org/2001/tag/doc/qnameids

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

*/

use crate::error::{Error as RdfError, ErrorKind};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct QName {
    prefix: Option<String>,
    name: String,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const NAME_ONLY: usize = 1;
const PREFIX_AND_NAME: usize = 2;

impl Display for QName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, "{}:", prefix)?
        }
        write!(f, "{}", &self.name)
    }
}

impl FromStr for QName {
    type Err = RdfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ErrorKind::EmptyQName.into())
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            match parts.len() {
                NAME_ONLY => {
                    let name = *parts.get(0).unwrap();
                    if is_xml_name(name) {
                        Ok(QName {
                            prefix: None,
                            name: name.to_string(),
                        })
                    } else {
                        Err(ErrorKind::InvalidQName(s.to_string()).into())
                    }
                }
                PREFIX_AND_NAME => {
                    let prefix = *parts.get(0).unwrap();
                    let name = *parts.get(1).unwrap();
                    if is_xml_name(prefix) && is_xml_name(name) {
                        Ok(QName {
                            prefix: Some(prefix.to_string()),
                            name: name.to_string(),
                        })
                    } else {
                        Err(ErrorKind::InvalidQName(s.to_string()).into())
                    }
                }
                _ => Err(ErrorKind::InvalidQName(s.to_string()).into()),
            }
        }
    }
}

impl QName {
    pub fn new(name: &str) -> Self {
        assert!(is_xml_name(name));
        Self {
            prefix: None,
            name: name.to_string(),
        }
    }

    pub fn with_prefix(prefix: &str, name: &str) -> Self {
        assert!(is_xml_name(prefix));
        assert!(is_xml_name(name));
        Self {
            prefix: Some(prefix.to_string()),
            name: name.to_string(),
        }
    }

    pub fn prefix(&self) -> &Option<String> {
        &self.prefix
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn as_curie(&self) -> String {
        format!(
            "[{}:{}]",
            match &self.prefix {
                None => "",
                Some(prefix) => prefix,
            },
            &self.name
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn is_xml_name_start_char(c: char) -> bool {
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

pub(crate) fn is_xml_name_char(c: char) -> bool {
    is_xml_name_start_char(c)
        || c == '-'
        || c == '.'
        || (c >= '0' && c <= '9')
        || c == '\u{B7}'
        || (c >= '\u{0300}' && c <= '\u{036F}')
        || (c >= '\u{203F}' && c <= '\u{2040}')
}

pub(crate) fn is_xml_name(s: &str) -> bool {
    !s.is_empty() && s.starts_with(is_xml_name_start_char) && s[1..].chars().all(is_xml_name_char)
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_qname() {
        let qname = QName::new("foo");
        assert_eq!(qname.to_string(), "foo".to_string());
        assert_eq!(qname.as_curie(), "[:foo]".to_string());

        let qname = QName::with_prefix("rdf", "foo");
        assert_eq!(qname.to_string(), "rdf:foo".to_string());
        assert_eq!(qname.as_curie(), "[rdf:foo]".to_string());
    }

    #[test]
    fn test_qname_from_str() {
        let qname = QName::from_str("foo");
        assert!(qname.is_ok());
        assert_eq!(qname.unwrap().to_string(), "foo".to_string());

        let qname = QName::from_str("rdf:foo");
        assert!(qname.is_ok());
        assert_eq!(qname.unwrap().to_string(), "rdf:foo".to_string());
    }

    #[test]
    fn test_qname_from_str_fail() {
        let qname = QName::from_str("");
        assert!(qname.is_err());

        let qname = QName::from_str("rdf foo");
        assert!(qname.is_err());

        let qname = QName::from_str(":foo");
        assert!(qname.is_err());

        let qname = QName::from_str("rdf::foo:bar");
        assert!(qname.is_err());
    }
}
