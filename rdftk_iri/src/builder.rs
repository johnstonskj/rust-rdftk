/*!
Provides a builder experience for creating `IRI` instances. The [`IriBuilder`](builder/struct.IriBuilder.html)
type provides a simple API to create new `IRI` instances in a fluent style.

# Example

```rust
use rdftk_iri::{builder::IriBuilder, IRI, error::Result as IriResult, Scheme};
use std::convert::TryInto;

fn make_example_iri() -> IriResult<IRI> {
    let mut builder = IriBuilder::default();
    builder
        .scheme(&Scheme::https())
        .user_name("john.doe")
        .host("www.example.com")?
        .port(123.into())
        .path_str("/forum/questions/")?
        .query_str("tag=networking&order=newest")?
        .fragment_str("top")?
        .try_into()
}
```

*/

#![allow(clippy::module_name_repetitions)]

use crate::error::{Error as IriError, Result as IriResult};
use crate::{Authority, Fragment, Host, Path, Port, Query, Scheme, UserInfo, IRI};
use std::convert::TryFrom;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The builder type, this provides simple API access to create new `IRI` instances in a
/// fluent style.
///
#[derive(Debug)]
pub struct IriBuilder {
    scheme: Option<Scheme>,
    host: Option<Host>,
    user_name: Option<String>,
    password: Option<String>,
    port: Option<Port>,
    path: Option<Path>,
    query: Option<Query>,
    fragment: Option<Fragment>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for IriBuilder {
    fn default() -> Self {
        Self {
            scheme: None,
            host: None,
            user_name: None,
            password: None,
            port: None,
            path: None,
            query: None,
            fragment: None,
        }
    }
}

impl TryFrom<&mut IriBuilder> for IRI {
    type Error = IriError;

    fn try_from(builder: &mut IriBuilder) -> Result<Self, Self::Error> {
        let mut iri = match &builder.path {
            None => IRI::default(),
            Some(path) => IRI::new(path),
        };

        if let Some(scheme) = &builder.scheme {
            iri.set_scheme(Some(scheme.clone()));
        }

        if let Some(host) = &builder.host {
            let mut authority = Authority::new(host.clone());

            if let Some(port) = &builder.port {
                authority.set_port(port.clone());
            }

            if let Some(user_name) = &builder.user_name {
                let mut user_info = UserInfo::new(user_name)?;
                if let Some(password) = &builder.password {
                    user_info.set_password(password)?;
                }
                authority.set_user_info(user_info);
            }
            iri.set_authority(Some(authority));
        }

        if let Some(query) = &builder.query {
            iri.set_query(Some(query.clone()));
        }

        if let Some(fragment) = &builder.fragment {
            iri.set_fragment(Some(fragment.clone()));
        }

        Ok(iri)
    }
}

impl IriBuilder {
    /// Use the provided scheme for this IRI.
    pub fn scheme(&mut self, scheme: &Scheme) -> &mut Self {
        self.scheme = Some(scheme.clone());
        self
    }

    /// Use the provided scheme, parsed from a string, for this IRI.
    pub fn scheme_str(&mut self, scheme: &str) -> IriResult<&mut Self> {
        self.scheme = Some(Scheme::from_str(scheme)?);
        Ok(self)
    }

    /// Use the provided host name for this IRI's authority.
    pub fn host(&mut self, host: &str) -> IriResult<&mut Self> {
        match Host::from_str(host) {
            Err(e) => Err(e),
            Ok(host) => {
                self.host = Some(host);
                Ok(self)
            }
        }
    }

    /// Use the provided port number for this IRI's authority.
    pub fn port(&mut self, port: Port) -> &mut Self {
        self.port = Some(port);
        self
    }

    /// Use the provided user name and password for this IRI's authority.
    pub fn user(&mut self, user_name: &str, password: &str) -> &mut Self {
        self.user_name = Some(user_name.to_string());
        self.password = Some(password.to_string());
        self
    }

    /// Use the provided user name for this IRI's authority.
    pub fn user_name(&mut self, user_name: &str) -> &mut Self {
        self.user_name = Some(user_name.to_string());
        self
    }

    /// Use the provided password for this IRI's authority.
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_string());
        self
    }

    /// Use the provided path for this IRI.
    pub fn path(&mut self, path: &Path) -> &mut Self {
        self.path = Some(path.clone());
        self
    }

    /// Use the provided path, parsed from a string, for this IRI.
    pub fn path_str(&mut self, path: &str) -> IriResult<&mut Self> {
        self.path = Some(Path::from_str(path)?);
        Ok(self)
    }

    /// Append a segment to the path for this IRI.
    pub fn append_segment(&mut self, segment: &str) -> &mut Self {
        match &mut self.path {
            None => self.path = Some(Path::from_str(segment).unwrap()),
            Some(path) => path.push(segment).unwrap(),
        }
        self
    }

    /// Use the provided query for this IRI.
    pub fn query(&mut self, query: &Query) -> &mut Self {
        self.query = Some(query.clone());
        self
    }

    /// Use the provided query, parsed from a string, for this IRI.
    pub fn query_str(&mut self, query: &str) -> IriResult<&mut Self> {
        self.query = Some(Query::from_str(query)?);
        Ok(self)
    }

    /// Use the provided fragment for this IRI.
    pub fn fragment(&mut self, fragment: &Fragment) -> &mut Self {
        self.fragment = Some(fragment.clone());
        self
    }

    /// Use the provided fragment, parsed from a string, for this IRI.
    pub fn fragment_str(&mut self, fragment: &str) -> IriResult<&mut Self> {
        self.fragment = Some(Fragment::from_str(fragment)?);
        Ok(self)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheme::Scheme;
    use std::convert::TryInto;

    #[test]
    fn test_http_url() {
        fn inner() -> IriResult<IRI> {
            let mut builder = IriBuilder::default();
            let result: IriResult<IRI> = builder
                .scheme(&Scheme::https())
                .user_name("john.doe")
                .host("www.example.com")?
                .port(123.into())
                .path_str("/forum/questions/")?
                .query_str("tag=networking&order=newest")?
                .fragment_str("top")?
                .try_into();
            assert!(result.is_ok());
            result
        }
        let iri = inner().unwrap();
        assert_eq!(
            iri.to_string(),
            "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
                .to_string()
        );
    }

    #[test]
    fn test_ldap_url() {
        let mut builder = IriBuilder::default();
        let builder = builder
            .scheme(&Scheme::ldap())
            .host("[2001:db8::7]")
            .unwrap()
            .path_str("/c=GB")
            .unwrap()
            .query_str("objectClass?one")
            .unwrap();
        println!("{:#?}", builder);
        let result: IriResult<IRI> = builder.try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        println!("{:#?}", iri);
        assert_eq!(
            iri.to_string(),
            "ldap://[2001:db8::7]/c=GB?objectClass?one".to_string()
        );
    }

    #[test]
    fn test_mailto_iri() {
        let mut builder = IriBuilder::default();
        let result: IriResult<IRI> = builder
            .scheme(&Scheme::mailto())
            .path_str("John.Doe@example.com")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(iri.to_string(), "mailto:John.Doe@example.com".to_string());
    }

    #[test]
    fn test_news_iri() {
        let mut builder = IriBuilder::default();
        let result: IriResult<IRI> = builder
            .scheme(&Scheme::news())
            .path_str("comp.infosystems.www.servers.unix")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(
            iri.to_string(),
            "news:comp.infosystems.www.servers.unix".to_string()
        );
    }

    #[test]
    fn test_tel_iri() {
        let mut builder = IriBuilder::default();
        let result: IriResult<IRI> = builder
            .scheme(&Scheme::tel())
            .path_str("+1-816-555-1212")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(iri.to_string(), "tel:+1-816-555-1212".to_string());
    }

    #[test]
    fn test_telnet_iri() {
        let mut builder = IriBuilder::default();
        let result: IriResult<IRI> = builder
            .scheme(&Scheme::telnet())
            .host("192.0.2.16")
            .unwrap()
            .port(80.into())
            .path_str("/")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(iri.to_string(), "telnet://192.0.2.16:80/".to_string());
    }

    #[test]
    fn test_urn_iri() {
        let mut builder = IriBuilder::default();
        let result: IriResult<IRI> = builder
            .scheme(&Scheme::urn())
            .path_str("oasis:names:specification:docbook:dtd:xml:4.1.2")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(
            iri.to_string(),
            "urn:oasis:names:specification:docbook:dtd:xml:4.1.2".to_string()
        );
    }
}
