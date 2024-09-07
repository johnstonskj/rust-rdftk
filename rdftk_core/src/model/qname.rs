/*!
* Qualified names, names with the form `{prefix}:{name}` are used in a number of common serialization
* forms and use many of the same production rules as those for XML.
*
* # Example
*
* ```rust
* use rdftk_core::model::qname::QName;
* use rdftk_iri::Name;
* use std::str::FromStr;
*
* let prefixed: QName = "prefix:name".parse().expect("parse error");
* let un_prefixed: QName = "name".parse().expect("parse error");
*
* let prefixed: QName = QName::new(
*     Name::new_unchecked("prefix"),
*     Name::new_unchecked("name"),
* ).unwrap();
* let un_prefixed: QName = QName::new_unqualified(Name::new_unchecked("name")).unwrap();
*
* assert!(QName::from_str("").is_err());
* assert!(QName::from_str("hello world").is_err());
* ```
*
* # Specification -- QName
*
* 1. <https://www.w3.org/TR/REC-xml-names/>
* 2. <https://www.w3.org/TR/REC-xml/#NT-Name>
* 3. <https://www.w3.org/2001/tag/doc/qnameids>
*
* From (1):
*
* ```text
* /* Attribute Names for Namespace Declaration */
*
* [4]  NCName          ::=  Name - (Char* ':' Char*)     /* An XML Name, minus the ":" */
*
* /* Qualified Name */
*
* [7]  QName           ::=  PrefixedName
*                           | UnprefixedName
* [8]  PrefixedName    ::=  Prefix ':' LocalPart
* [9]  UnprefixedName  ::=  LocalPart
* [10] Prefix          ::=  NCName
* [11] LocalPart       ::=  NCName
* ```
*
* From (2):
*
* ```text
*
* [4]  NameStartChar   ::=  ":" | [A-Z] | "_" | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF]
*                           | [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F]
*                           | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD]
*                           | [#x10000-#xEFFFF]
* [4a] NameChar        ::=  NameStartChar | "-" | "." | [0-9] | #xB7 | [#x0300-#x036F]
*                           | [#x203F-#x2040]
* [5]  Name            ::=  NameStartChar (NameChar)*
* ```
*
* # Specification -- Curie
*
* 1. <https://www.w3.org/TR/curie/>
*
* ```text
* safe_curie  :=   '[' curie ']'
*
* curie       :=   [ [ prefix ] ':' ] reference
*
* prefix      :=   NCName
*
* reference   :=   irelative-ref (as defined in IRI)
* ```
*
* Note that while the empty string matches the production for curie above, an empty string is NOT a
* valid CURIE. The CURIE prefix '_' is reserved for use by languages that support RDF. For this
* reason, the prefix '_' SHOULD be avoided by authors.
*
*/

use rdftk_iri::{Name, NameParser};

use crate::error::{Error as RdfError, ErrorKind};
use crate::model::statement::BLANK_NODE_NAMESPACE;
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
    prefix: Option<Name>,
    name: Name,
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
        let name_parser = NameParser::Xml;
        if s.is_empty() {
            Err(ErrorKind::EmptyQName.into())
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            match parts.len() {
                NAME_ONLY => {
                    let name = *parts.first().unwrap();
                    if Name::is_valid_str(name, name_parser) {
                        Ok(QName {
                            prefix: None,
                            name: Name::from_str(name)?,
                        })
                    } else {
                        Err(ErrorKind::InvalidQName(s.to_string()).into())
                    }
                }
                PREFIX_AND_NAME => {
                    let prefix = *parts.first().unwrap();
                    let name = *parts.get(1).unwrap();
                    if Name::is_valid_str(prefix, name_parser)
                        && Name::is_valid_str(name, name_parser)
                    {
                        Ok(QName {
                            prefix: Some(Name::from_str(prefix)?),
                            name: Name::from_str(name)?,
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
    /// Construct a new qualified QName: "`{prefix}:{name}`". This will return an error if either
    /// the prefix or name is empty or is an invalid QName part.
    pub fn new(prefix: Name, name: Name) -> Result<Self, RdfError> {
        Ok(Self::new_unchecked(Some(prefix), name))
    }

    /// Construct a new unqualified QName: "`:{name}`". This will return an error if either
    /// the name is empty or is an invalid QName part.
    pub fn new_unqualified(name: Name) -> Result<Self, RdfError> {
        Ok(Self::new_unchecked(None, name))
    }

    /// Construct a new QName **without** any validation checks on the given values.
    pub fn new_unchecked(prefix: Option<Name>, name: Name) -> Self {
        Self { prefix, name }
    }

    /// Construct a new blank node as a QName.
    pub fn new_blank(name: Name) -> Result<Self, RdfError> {
        Ok(Self::new_unchecked(
            Some(Name::new_unchecked(BLANK_NODE_NAMESPACE)),
            name,
        ))
    }

    /// Returns `true` if this QName is a blank node, else `false`.
    pub fn is_blank(&self) -> bool {
        self.prefix
            .as_ref()
            .map(|p| p.as_ref() == BLANK_NODE_NAMESPACE)
            .is_some()
    }

    /// Returns `true` if this QName has a prefix, else `false`.
    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    /// Returns the prefix part of this QName, if present.
    pub fn prefix(&self) -> Option<&Name> {
        self.prefix.as_ref()
    }

    /// Returns the name part of this QName.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Format this QName as a Curie string.
    pub fn as_curie(&self) -> String {
        format!(
            "[{}:{}]",
            match &self.prefix {
                None => "",
                Some(prefix) => prefix.as_ref(),
            },
            &self.name
        )
    }
}
