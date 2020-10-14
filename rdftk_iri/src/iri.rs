/*!
The overall `IRI` type comprised of a [`Scheme`](scheme/struct.Scheme.html),
[`Authority`](authority/struct.Authority.html), [`Path`](path/struct.Path.html),
[`Query`](query/struct.Query.html), and [`Fragment`](fragment/struct.Fragment.html).
*/

use crate::error::{Error as IriError, ErrorKind, Result as IriResult};
use crate::{Authority, Fragment, Normalize, Path, Query, Scheme};
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::Arc;

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

pub type IRIRef = Arc<IRI>;

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
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_iri(s)
    }
}

impl Normalize for IRI {
    fn normalize(self) -> IriResult<Self> {
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

    pub fn resolve(&self, path: &Path) -> IriResult<Self> {
        if !self.has_scheme() {
            Err(ErrorKind::NotValidBase.into())
        } else {
            Ok(self.with_new_path(self.path().resolve(path)?))
        }
    }

    pub fn is_absolute(&self) -> bool {
        self.has_scheme() && self.has_authority() && !self.has_fragment()
    }

    pub fn to_absolute(&self) -> Option<Self> {
        if self.has_scheme() && self.has_authority() {
            if self.fragment.is_some() {
                Some(Self {
                    fragment: None,
                    ..self.clone()
                })
            } else {
                Some(self.clone())
            }
        } else {
            None
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

fn parse_iri(s: &str) -> IriResult<IRI> {
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
    fn test_parse_iri_simple() {
        let result = parse_iri(
            "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
        );
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_iri("ldap://[2001:db8::7]/c=GB?objectClass?one");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_iri("mailto:John.Doe@example.com");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_iri("news:comp.infosystems.www.servers.unix");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_iri("tel:+1-816-555-1212");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_iri("telnet://192.0.2.16:80/");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_iri("urn:oasis:names:specification:docbook:dtd:xml:4.1.2");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_iri("https://en.wiktionary.org/wiki/Ῥόδος");
        assert!(result.is_ok());
        println!("{:#?}", result);

        let result = parse_iri("http://www.myfictionαlbank.com/");
        assert!(result.is_ok());
        println!("{:#?}", result);
    }
}
