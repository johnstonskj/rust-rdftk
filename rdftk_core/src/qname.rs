/*!
Qualified names, names with the form `{prefix}:{name}` are used in a number of common serialization
forms and use many of the same production rules as those for XML.

# Example

```rust
use rdftk_core::qname::QName;

let prefixed: QName = "prefix:name".parse().expect("parse error");
let un_prefixed: QName = "name".parse().expect("parse error");

let prefixed: QName = QName::with_prefix("prefix", "name").unwrap();
let un_prefixed: QName = QName::new("name").unwrap();

assert!(QName::new("").is_err());
assert!(QName::new("hello world").is_err());
```

# Specification -- QName

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

# Specification -- Curie

1. https://www.w3.org/TR/curie/

```text
safe_curie  :=   '[' curie ']'

curie       :=   [ [ prefix ] ':' ] reference

prefix      :=   NCName

reference   :=   irelative-ref (as defined in IRI)
```

* Note that while the empty string matches the production for curie above, an empty string is NOT a valid CURIE.
* The CURIE prefix '_' is reserved for use by languages that support RDF. For this reason, the prefix '_' SHOULD be avoided by authors.

*/

use crate::error::{Error as RdfError, ErrorKind};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// QNames are valid identifiers with an optional prefix identifier. e.g. "`xsd:integer`",
/// "`rdfs:Class`", "`:subPropertyOf`".
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QName {
    prefix: Option<String>,
    name: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
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
    type Err = RdfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ErrorKind::EmptyQName.into())
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            match parts.len() {
                NAME_ONLY => {
                    let name = *parts.get(0).unwrap();
                    if QName::is_valid(name) {
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
                    if QName::is_valid(prefix) && QName::is_valid(name) {
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
    /// Construct a new unqualified QName: "`:{name}`". This will return an error if either
    /// the name is empty or is an invalid QName part.
    pub fn new(name: &str) -> Result<Self, RdfError> {
        if name.is_empty() {
            Err(ErrorKind::EmptyQName.into())
        } else if !QName::is_valid(name) {
            Err(ErrorKind::InvalidQName(name.to_string()).into())
        } else {
            Ok(Self::new_unchecked(None, name))
        }
    }

    ///
    /// Construct a new QName **without** any validation checks on the given values.
    pub fn new_unchecked(prefix: Option<&str>, name: &str) -> Self {
        Self {
            prefix: prefix.map(str::to_string),
            name: name.to_string(),
        }
    }

    /// Construct a new qualified QName: "`{prefix}:{name}`". This will return an error if either
    /// the prefix or name is empty or is an invalid QName part.
    pub fn with_prefix(prefix: &str, name: &str) -> Result<Self, RdfError> {
        if prefix.is_empty() || name.is_empty() {
            Err(ErrorKind::EmptyQName.into())
        } else if !QName::is_valid(prefix) {
            Err(ErrorKind::InvalidQName(prefix.to_string()).into())
        } else if !QName::is_valid(name) {
            Err(ErrorKind::InvalidQName(name.to_string()).into())
        } else {
            Ok(Self::new_unchecked(Some(prefix), name))
        }
    }

    /// Returns `true` if this QName has a prefix, else `false`.
    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    /// Returns the prefix part of this QName, if present.
    pub fn prefix(&self) -> &Option<String> {
        &self.prefix
    }

    /// Returns the name part of this QName.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Format this QName as a Curie string.
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

    /// Returns true if `part` is a valid identifier for either name or prefix.
    pub fn is_valid(part: &str) -> bool {
        is_xml_name(part)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn is_xml_name_start_char(c: char) -> bool {
    c == ':'
        || ('A'..='Z').contains(&c)
        || c == '_'
        || ('a'..='z').contains(&c)
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

pub(crate) fn is_xml_name_char(c: char) -> bool {
    is_xml_name_start_char(c)
        || c == '-'
        || c == '.'
        || ('0'..='9').contains(&c)
        || c == '\u{B7}'
        || ('\u{0300}'..='\u{036F}').contains(&c)
        || ('\u{203F}'..='\u{2040}').contains(&c)
}

pub(crate) fn is_xml_name(s: &str) -> bool {
    !s.is_empty() && s.starts_with(is_xml_name_start_char) && s[1..].chars().all(is_xml_name_char)
}
