use crate::error::{Error as IriError, ErrorKind, Result as IriResult};
use crate::{Authority, Fragment, Normalize, Path, PercentEncoding, Port, Query, Scheme};
use regex::Regex;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The IRI type comprised of [`Scheme`](scheme/struct.Scheme.html),
/// [`Authority`](authority/struct.Authority.html), [`Path`](path/struct.Path.html),
/// [`Query`](query/struct.Query.html), and [`Fragment`](fragment/struct.Fragment.html) components.
/// Note that for most APIs the use of [`IRIRef`](type.IRIRef.html) is preferred over `IRI` directly.
///
/// # Example
///
/// The following example creates a new `IRI` from a string, but also then uses that as a template
/// to create an additional one. The `normalize` method in this example will convert the host name
/// to lower case. The `without_query` method creates a new IRI with all components _except_ the
/// query copied over.
///
/// ```rust,no_run
/// use rdftk_iri::{IRI, Normalize};
/// use std::str::FromStr;
///
/// let iri = IRI::from_str(
///     "https://john.doe@www.EXAMPLE.com:123/forum/questions/?tag=networking&order=newest#top",
/// ).unwrap();
///
/// let new_uri = iri.normalize().unwrap().without_query();
/// ```
///
/// # Definitions (from Wikipedia)
///
/// Each URI begins with a scheme name that refers to a specification for assigning identifiers
/// within that scheme. As such, the URI syntax is a federated and extensible naming system wherein
/// each scheme's specification may further restrict the syntax and semantics of identifiers using
/// that scheme. The URI generic syntax is a superset of the syntax of all URI schemes. It was
/// first defined in RFC 2396, and finalized in RFC 3986.
///
/// The URI generic syntax consists of a hierarchical sequence of five components:[8]
///
/// > `URI = scheme:[//authority]path[?query][#fragment]`
///
/// where the authority component divides into three subcomponents:
///
/// > `authority = [userinfo@]host[:port]`
///
/// The URI comprises:
///
/// * A non-empty **scheme** component followed by a colon (:), consisting of a sequence of characters
///   beginning with a letter and followed by any combination of letters, digits, plus (+), period
///   (.), or hyphen (-). Although schemes are case-insensitive, the canonical form is lowercase
///   and documents that specify schemes must do so with lowercase letters. Examples of popular
///   schemes include http, https, ftp, mailto, file, data, and irc. URI schemes should be
///   registered with the Internet Assigned Numbers Authority (IANA), although non-registered
///   schemes are used in practice.
/// * An optional **authority** component preceded by two slashes (//), comprising:
///   * An optional **userinfo** subcomponent that may consist of a user name and an optional password
///     preceded by a colon (:), followed by an at symbol (@). Use of the format username:password
///     in the userinfo subcomponent is deprecated for security reasons. Applications should not
///     render as clear text any data after the first colon (:) found within a userinfo subcomponent
///     unless the data after the colon is the empty string (indicating no password).
///   * A **host** subcomponent, consisting of either a registered name (including but not limited to a
///     hostname), or an IP address. IPv4 addresses must be in dot-decimal notation, and IPv6
///     addresses must be enclosed in brackets ([]).
///   * An optional **port** subcomponent preceded by a colon (:).
/// * A **path** component, consisting of a sequence of path segments separated by a slash (/). A
///   path is always defined for a URI, though the defined path may be empty (zero length). A
///   segment may also be empty, resulting in two consecutive slashes (//) in the path component. A
///   path component may resemble or map exactly to a file system path, but does not always imply a
///   relation to one. If an authority component is present, then the path component must either be
///   empty or begin with a slash (/). If an authority component is absent, then the path cannot
///   begin with an empty segment, that is with two slashes (//), as the following characters would
///   be interpreted as an authority component. The final segment of the path may be referred to as
///   a 'slug'.
/// * An optional **query** component preceded by a question mark (?), containing a query string of
///   non-hierarchical data. Its syntax is not well defined, but by convention is most often a
///   sequence of attribute–value pairs separated by a delimiter.
/// * An optional **fragment** component preceded by a hash (#). The fragment contains a fragment
///   identifier providing direction to a secondary resource, such as a section heading in an
///   article identified by the remainder of the URI. When the primary resource is an HTML document,
///   the fragment is often an id attribute of a specific element, and web browsers will scroll this
///   element into view.
///
/// Strings of data octets within a URI are represented as characters. Permitted characters within a
/// URI are the ASCII characters[*] for the lowercase and uppercase letters of the modern English
/// alphabet, the Arabic numerals, hyphen, period, underscore, and tilde. Octets represented by any
/// other character must be percent-encoded.
///
/// Of the ASCII character set, the characters : / ? # [ ] @ are reserved for use as delimiters of
/// the generic URI components and must be percent-encoded – for example, %3F for a question mark.
/// The characters ! $ & ' ( ) * + , ; = are permitted by generic URI syntax to be used unencoded in
/// the user information, host, and path as delimiters. Additionally, : and @ may appear
/// unencoded within the path, query, and fragment; and ? and / may appear unencoded as data within
/// the query or fragment.
///
/// [*] While URIs are limited to a subset of the ASCII character set, IRIs may additionally contain
/// most characters from the Universal Character Set (Unicode/ISO 10646), including Chinese,
/// Japanese, Korean, and Cyrillic characters.
///
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IRI {
    scheme: Option<Scheme>,
    authority: Option<Authority>,
    path: Path,
    query: Option<Query>,
    fragment: Option<Fragment>,
}

///
/// A preferred reference-counted type to wrap an `IRI`. For RDF where IRIs are extensively reused
/// as graph nodes, the requirement to use a reference type is very important to reduce duplication.
///
/// As such, the APIs across the RDFtk crates use `IRIRef` exclusively.
///
#[allow(clippy::upper_case_acronyms)]
pub type IRIRef = Arc<IRI>;

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
            "{}{}",
            match &self.scheme {
                None => String::new(),
                Some(scheme) => scheme.to_string(),
            },
            &self.scheme_specific_part()
        )
    }
}

impl FromStr for IRI {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_iri(s)
    }
}

impl From<Path> for IRI {
    fn from(path: Path) -> Self {
        Self::from(&path)
    }
}

impl From<&Path> for IRI {
    fn from(path: &Path) -> Self {
        Self::new(path)
    }
}

#[cfg(feature = "path_iri")]
impl TryFrom<PathBuf> for IRI {
    type Error = IriError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(&path)
    }
}

#[cfg(feature = "path_iri")]
impl TryFrom<&PathBuf> for IRI {
    type Error = IriError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        Self::new_file(path)
    }
}

#[cfg(feature = "uuid_iri")]
impl TryFrom<Uuid> for IRI {
    type Error = IriError;

    fn try_from(path: Uuid) -> Result<Self, Self::Error> {
        Self::try_from(&path)
    }
}

#[cfg(feature = "uuid_iri")]
impl TryFrom<&Uuid> for IRI {
    type Error = IriError;

    fn try_from(path: &Uuid) -> Result<Self, Self::Error> {
        Self::new_name("uuid", &path.to_hyphenated().to_string())
    }
}

impl Normalize for IRI {
    fn normalize(self) -> IriResult<Self> {
        let scheme = match &self.scheme {
            None => None,
            Some(scheme) => Some(scheme.clone().normalize()?),
        };
        let authority = match &self.authority {
            None => None,
            Some(authority) => {
                let mut authority = authority.clone().normalize()?;
                if self.has_scheme() && !authority.has_port() {
                    let scheme = self.scheme().as_ref().unwrap();
                    let scheme = scheme.clone().normalize().unwrap();
                    if let Some(port) = Port::default_for(&scheme) {
                        authority.set_port(port);
                    }
                }
                Some(authority)
            }
        };
        let mut path = self.path.normalize()?;
        if let Some(scheme) = &scheme {
            if vec!["file", "ftp", "http", "https", "tftp"].contains(&scheme.value().as_str())
                && path.is_empty()
            {
                path = Path::root();
            }
        }
        let query = match self.query {
            None => None,
            Some(query) => Some(query.normalize()?),
        };
        let fragment = match self.fragment {
            None => None,
            Some(fragment) => Some(fragment.normalize()?),
        };

        Ok(Self {
            scheme,
            authority,
            path,
            query,
            fragment,
        })
    }
}

impl PercentEncoding for IRI {
    fn encode(&self, for_uri: bool) -> Self
    where
        Self: Sized,
    {
        Self {
            scheme: self.scheme.clone(),
            authority: self.authority.as_ref().map(|a| a.encode(for_uri)),
            path: self.path.encode(for_uri),
            query: self.query.as_ref().map(|q| q.encode(for_uri)),
            fragment: self.fragment.as_ref().map(|f| f.encode(for_uri)),
        }
    }
}

impl IRI {
    ///
    /// Create a new `IRI` with only the specified path, this is a valid relative value
    /// -- see [is_absolute](#method.is_absolute).
    ///
    /// This is also provided as an implementation of `From<&Path>` and `From<Path>` for `IRI`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rdftk_iri::{IRI, Path};
    /// use std::str::FromStr;
    ///
    /// let iri = IRI::new(
    ///     &Path::from_str("/forum/questions/").unwrap()
    /// );
    /// ```
    ///
    pub fn new(path: &Path) -> Self {
        Self {
            scheme: None,
            authority: None,
            path: path.clone(),
            query: None,
            fragment: None,
        }
    }

    ///
    /// Construct a new `IRI` with the "file" scheme and the provided file system path.
    ///
    /// This is also provided as an implementation of `TryFrom<&PathBuf>` and `TryFrom<PathBuf>`
    /// for `IRI` if the "path_iri" feature has been enabled.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rdftk_iri::IRI;
    /// use std::path::PathBuf;
    ///
    /// let iri = IRI::new_file(&PathBuf::from("Documents/test-plan.md")).unwrap();
    /// ```
    ///
    /// This results in the `IRI`  `file://Documents/test-plan.md`.
    ///
    #[cfg(feature = "path_iri")]
    pub fn new_file(path: &std::path::Path) -> IriResult<Self> {
        Ok(Self {
            scheme: Some(Scheme::file()),
            authority: None,
            path: Path::from_str(&path.to_string_lossy().to_string())?,
            query: None,
            fragment: None,
        })
    }

    ///
    /// Construct a new URN `IRI` with the given namespace identifier and namespace-specific
    /// string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rdftk_iri::IRI;
    ///
    /// let iri = IRI::new_name("uuid", "f3b4958c-52a1-11e7-802a-010203040506").unwrap();
    /// ```
    ///
    /// This results in the `IRI`  `urn:uuid:f3b4958c-52a1-11e7-802a-010203040506`.
    ///
    pub fn new_name(
        namespace_identifier: &str,
        namespace_specific_string: &str,
    ) -> IriResult<Self> {
        Ok(Self {
            scheme: Some(Scheme::urn()),
            authority: None,
            path: Path::from_str(&format!(
                "{}:{}",
                namespace_identifier, namespace_specific_string
            ))?,
            query: None,
            fragment: None,
        })
    }

    ///
    /// Create a new well-known `IRI` using the `"genid"` form. This creates a new UUID value and
    /// replaces the path of the provided base `IRI` with the new path.
    ///
    /// ```rust
    /// use rdftk_iri::{IRI, Path};
    /// use std::str::FromStr;
    ///
    /// let base_iri = IRI::from_str("http://example.org/").unwrap();
    /// let gen_id = IRI::new_genid(&base_iri).unwrap();
    ///
    /// // http://example.org/.well-known/genid/{uuid}
    /// println!("{}", gen_id);
    ///
    /// ```
    ///
    #[cfg(feature = "genid")]
    pub fn new_genid(base: &IRI) -> IriResult<Self> {
        let new_uuid = Uuid::new_v4();
        let new_uuid = new_uuid
            .to_simple()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();
        let mut path = Path::well_known();
        path.push("genid")?;
        path.push(&new_uuid)?;
        let mut iri: IRI = base.clone();
        iri.set_path(path);
        Ok(iri)
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Return a new `IRI` as a copy of `self` with the path component replaced by the provided
    /// `path` value.
    ///
    pub fn with_new_path(&self, path: Path) -> Self {
        Self {
            path,
            ..self.clone()
        }
    }

    ///
    /// Return a new `IRI` as a copy of `self` with the path component replaced with the value of
    /// `Path::default()`.
    ///
    pub fn without_path(&self) -> Self {
        Self {
            path: Path::default(),
            ..self.clone()
        }
    }

    ///
    /// Return a new `IRI` as a copy of `self` with the query component replaced by the provided
    /// `query` value.
    ///
    pub fn with_new_query(&self, query: Query) -> Self {
        Self {
            query: Some(query),
            ..self.clone()
        }
    }

    ///
    /// Return a new `IRI` as a copy of `self` with the query component removed.
    ///
    pub fn without_query(&self) -> Self {
        Self {
            query: None,
            ..self.clone()
        }
    }

    ///
    /// Return a new `IRI` as a copy of `self` with the fragment component replaced by the provided
    /// `fragment` value.
    ///
    pub fn with_new_fragment(&self, fragment: Fragment) -> Self {
        Self {
            fragment: Some(fragment),
            ..self.clone()
        }
    }

    ///
    /// Return a new `IRI` as a copy of `self` with the fragment component removed.
    ///
    pub fn without_fragment(&self) -> Self {
        Self {
            fragment: None,
            ..self.clone()
        }
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Resolves the `IRI` value `relative` using `self` as the base.
    ///
    /// If the given URI is already absolute, or if this URI is opaque, then the given URI is
    /// returned. Otherwise this method constructs a new hierarchical URI in a manner consistent
    /// with RFC 2396, §5.2;.
    ///
    pub fn resolve(&self, relative: &IRI) -> IriResult<Self> {
        if relative.is_absolute() || self.is_opaque() {
            Ok(relative.clone())
        } else if !relative.has_scheme()
            && !relative.has_authority()
            && relative.path().is_empty()
            && !relative.has_query()
            && relative.has_fragment()
        {
            Ok(self.with_new_fragment(relative.fragment().as_ref().unwrap().clone()))
        } else {
            // Otherwise construct a new hierarchical URI in a manner consistent with RFC 2396, section 5.2.
            unimplemented!()
        }
    }

    ///
    /// Relativizes the `IRI` `other` against this `IRI`.
    ///
    /// The relativization of the `IRI` `other` against this `IRI` is computed as follows:
    ///
    /// 1. If either this `IRI` or the given `IRI` are opaque, or if the scheme and authority
    ///    components of the two `IRI`s are not identical, or if the path of this `IRI` is not a
    ///    prefix of the path of the given `IRI`, then the given `IRI` is returned.
    /// 2. Otherwise a new relative hierarchical `IRI` is constructed with query and fragment
    ///    components taken from the given `IRI` and with a path component computed by removing
    ///    this `IRI`'s path from the beginning of the given `IRI`'s path.
    ///
    pub fn relativize(&self, _other: &IRIRef) -> IriResult<Self> {
        unimplemented!()
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Returns `true` if this is an absolute `IRI`, else `false`. An `IRI` is absolute if, and only
    /// if, it has a scheme component and does not have a fragment component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rdftk_iri::IRI;
    /// # use std::str::FromStr;
    /// assert!(IRI::from_str("mailto:a@b.com").unwrap().is_absolute());
    /// assert!(IRI::from_str("http://example.com").unwrap().is_absolute());
    /// assert!(IRI::from_str("http://example.com/path").unwrap().is_absolute());
    /// assert!(IRI::from_str("scheme://example.com").unwrap().is_absolute());
    /// assert!(IRI::from_str("scheme:example.com").unwrap().is_absolute());
    /// assert!(IRI::from_str("scheme:example.com/path").unwrap().is_absolute());
    ///
    /// assert!(!IRI::from_str("//example.com/path#foo").unwrap().is_absolute());
    /// assert!(!IRI::from_str("http://example.com/path#foo").unwrap().is_absolute());
    /// assert!(!IRI::from_str("scheme:example.com#foo").unwrap().is_absolute());
    /// assert!(!IRI::from_str("path").unwrap().is_absolute());
    /// assert!(!IRI::from_str("/path").unwrap().is_absolute());
    /// ```
    ///
    pub fn is_absolute(&self) -> bool {
        self.has_scheme() && !self.has_fragment()
    }

    ///
    /// Returns `true` if this is a relative `IRI` reference, else `false`. An `IRI` is a relative
    /// reference if, and only if, it does not have a scheme component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rdftk_iri::IRI;
    /// # use std::str::FromStr;
    /// assert!(IRI::from_str("//example.com/path#foo").unwrap().is_relative_reference());
    /// assert!(IRI::from_str("path").unwrap().is_relative_reference());
    /// assert!(IRI::from_str("/path").unwrap().is_relative_reference());
    ///
    /// assert!(!IRI::from_str("mailto:a@b.com").unwrap().is_relative_reference());
    /// assert!(!IRI::from_str("http://example.com").unwrap().is_relative_reference());
    /// assert!(!IRI::from_str("http://example.com/path").unwrap().is_relative_reference());
    /// assert!(!IRI::from_str("scheme://example.com").unwrap().is_relative_reference());
    /// assert!(!IRI::from_str("scheme:example.com").unwrap().is_relative_reference());
    /// assert!(!IRI::from_str("scheme:example.com/path").unwrap().is_relative_reference());
    /// assert!(!IRI::from_str("http://example.com/path#foo").unwrap().is_relative_reference());
    /// assert!(!IRI::from_str("scheme:example.com#foo").unwrap().is_relative_reference());
    /// ```
    ///
    pub fn is_relative_reference(&self) -> bool {
        !self.has_scheme()
    }

    ///
    /// Returns `true` if this is an opaque `IRI`, else `false`. An `IRI` is opaque if, and only if,
    /// it is absolute and its scheme-specific part does not begin with a slash character ('/'). An
    /// opaque `IRI` has a scheme, a scheme-specific part, and possibly a fragment; all other
    /// components are undefined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rdftk_iri::IRI;
    /// # use std::str::FromStr;
    /// assert!(IRI::from_str("mailto:a@b.com").unwrap().is_opaque());
    /// assert!(IRI::from_str("scheme:example.com").unwrap().is_opaque());
    /// assert!(IRI::from_str("scheme:example.com/path").unwrap().is_opaque());
    ///
    /// assert!(!IRI::from_str("http://example.com").unwrap().is_opaque());
    /// assert!(!IRI::from_str("http://example.com/path").unwrap().is_opaque());
    /// assert!(!IRI::from_str("scheme://example.com").unwrap().is_opaque());
    /// assert!(!IRI::from_str("path").unwrap().is_opaque());
    /// assert!(!IRI::from_str("/path").unwrap().is_opaque());
    /// ```
    ///
    pub fn is_opaque(&self) -> bool {
        let ssp = self.scheme_specific_part();
        self.is_absolute() && !ssp.is_empty() && !ssp.starts_with('/')
    }

    ///
    /// Returns true if this IRI's path starts with the well-known prefix defined in
    /// [RFC-8615: Well-Known Uniform Resource Identifiers (URIs)](https://datatracker.ietf.org/doc/html/rfc8615).
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rdftk_iri::IRI;
    /// # use std::str::FromStr;
    /// assert!(
    ///     IRI::from_str("http://example.com/.well-known/genid/d26a2d0e98334696f4ad70a677abc1f6")
    ///         .unwrap()
    ///         .is_well_known()
    /// );
    /// ```
    ///
    pub fn is_well_known(&self) -> bool {
        self.path().is_well_known()
    }

    // --------------------------------------------------------------------------------------------

    /// Return `true` if this `IRI` include a scheme, else `false`.
    pub fn has_scheme(&self) -> bool {
        self.scheme.is_some()
    }

    /// Return the current value of the scheme component.
    pub fn scheme(&self) -> &Option<Scheme> {
        &self.scheme
    }

    /// Return the scheme-specific part of the `IRI`, basically any part of the `IRI` that isn't
    /// the scheme itself.
    pub fn scheme_specific_part(&self) -> String {
        format!(
            "{}{}{}{}",
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

    /// Return `true` if this `IRI` include a authority, else `false`.
    pub fn has_authority(&self) -> bool {
        self.authority.is_some()
    }

    /// Return the current value of the authority component.
    pub fn authority(&self) -> &Option<Authority> {
        &self.authority
    }

    /// Return `true` if this `IRI` include a path, else `false`.
    pub fn has_path(&self) -> bool {
        !self.path.is_empty()
    }

    /// Return the current value of the path component.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Return `true` if this `IRI` include a query, else `false`.
    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    /// Return the current value of the query component.
    pub fn query(&self) -> &Option<Query> {
        &self.query
    }

    /// Return `true` if this `IRI` include a fragment, else `false`.
    pub fn has_fragment(&self) -> bool {
        self.fragment.is_some()
    }

    /// Return the current value of the fragment component.
    pub fn fragment(&self) -> &Option<Fragment> {
        &self.fragment
    }

    // --------------------------------------------------------------------------------------------

    /// Set the value of the scheme component.
    pub fn set_scheme(&mut self, scheme: Option<Scheme>) {
        self.scheme = scheme;
    }

    /// Set the value of the authority component.
    pub fn set_authority(&mut self, authority: Option<Authority>) {
        self.authority = authority;
    }

    /// Set the value of the path component.
    pub fn set_path(&mut self, path: Path) {
        self.path = path;
    }

    /// Set the value of the query component.
    pub fn set_query(&mut self, query: Option<Query>) {
        self.query = query;
    }

    /// Set the value of the fragment component.
    pub fn set_fragment(&mut self, fragment: Option<Fragment>) {
        self.fragment = fragment;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const GRP_SCHEME: usize = 2;
const GRP_AUTHORITY: usize = 4;
const GRP_PATH: usize = 5;
const GRP_QUERY: usize = 7;
const GRP_FRAGMENT: usize = 9;

lazy_static! {
    static ref PARSE_IRI_REGEX: Regex =
        Regex::new(r"^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?").unwrap();
}

fn parse_iri(s: &str) -> IriResult<IRI> {
    // From RFC-2396, appendix B. Parsing a URI Reference with a Regular Expression
    let regex = &PARSE_IRI_REGEX;
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
