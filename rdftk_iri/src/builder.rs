/*!
One-line description.

More detailed description, with

# Example

*/

#![allow(clippy::module_name_repetitions)]

// use ...

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::error::{Error as UriError, Result as UriResult};
use crate::{Authority, Fragment, Host, Path, Port, Query, QueryPart, Scheme, IRI};
use std::convert::TryFrom;
use std::str::FromStr;

pub struct UriBuilder {
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
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for UriBuilder {
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

impl TryFrom<&mut UriBuilder> for IRI {
    type Error = UriError;

    fn try_from(builder: &mut UriBuilder) -> Result<Self, Self::Error> {
        let mut uri = match &builder.path {
            None => IRI::default(),
            Some(path) => IRI::new(path),
        };
        let mut authority = Authority::default();
        if let Some(user_name) = &builder.user_name {
            authority.set_user_name(user_name)?;
        }
        if let Some(password) = &builder.password {
            authority.set_password(password)?;
        }
        if let Some(host) = &builder.host {
            authority.set_host(host.clone());
        }
        if let Some(port) = builder.port {
            authority.set_port(port);
        }
        if !authority.is_empty() {
            uri.set_authority(Some(authority));
        }
        if let Some(scheme) = &builder.scheme {
            uri.set_scheme(Some(scheme.clone()));
        }

        if let Some(query) = &builder.query {
            uri.set_query(Some(query.clone()));
        }
        if let Some(fragment) = &builder.fragment {
            uri.set_fragment(Some(fragment.clone()));
        }
        Ok(uri)
    }
}

impl UriBuilder {
    pub fn scheme(&mut self, scheme: &Scheme) -> &mut Self {
        self.scheme = Some(scheme.clone());
        self
    }
    pub fn scheme_str(&mut self, scheme: &str) -> UriResult<&mut Self> {
        self.scheme = Some(Scheme::from_str(scheme)?);
        Ok(self)
    }
    pub fn host_name(&mut self, host: &str) -> UriResult<&mut Self> {
        self.host = Some(Host::Name(host.to_string()));
        Ok(self)
    }
    pub fn ip_address(&mut self, host: &str) -> UriResult<&mut Self> {
        self.host = Some(Host::IPV4(host.to_string()));
        Ok(self)
    }
    pub fn ipv6_address(&mut self, host: &str) -> UriResult<&mut Self> {
        self.host = Some(Host::IPV6(host.to_string()));
        Ok(self)
    }
    pub fn port(&mut self, port: Port) -> &mut Self {
        self.port = Some(port);
        self
    }
    pub fn user(&mut self, user_name: &str, password: &str) -> &mut Self {
        self.user_name = Some(user_name.to_string());
        self.password = Some(password.to_string());
        self
    }
    pub fn user_name(&mut self, user_name: &str) -> &mut Self {
        self.user_name = Some(user_name.to_string());
        self
    }
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_string());
        self
    }
    pub fn path(&mut self, path: &Path) -> &mut Self {
        self.path = Some(path.clone());
        self
    }
    pub fn path_str(&mut self, path: &str) -> UriResult<&mut Self> {
        self.path = Some(Path::from_str(path)?);
        Ok(self)
    }
    pub fn append_segment(&mut self, segment: &str) -> &mut Self {
        match &mut self.path {
            None => self.path = Some(Path::from_str(segment).unwrap()),
            Some(path) => path.push(segment),
        }
        self
    }
    pub fn query(&mut self, query: &Query) -> &mut Self {
        self.query = Some(query.clone());
        self
    }
    pub fn query_str(&mut self, query: &str) -> UriResult<&mut Self> {
        self.query = Some(Query::from_str(query)?);
        Ok(self)
    }
    pub fn append_query(&mut self, key: &str, value: &str) -> &mut Self {
        let part = QueryPart::with_value(key, value);
        match &mut self.query {
            None => self.query = Some(Query::new(&part)),
            Some(query) => query.push(&part),
        }
        self
    }
    pub fn fragment(&mut self, fragment: &Fragment) -> &mut Self {
        self.fragment = Some(fragment.clone());
        self
    }
    pub fn fragment_str(&mut self, fragment: &str) -> UriResult<&mut Self> {
        self.fragment = Some(Fragment::from_str(fragment)?);
        Ok(self)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KnownSchemes;
    use std::convert::TryInto;

    #[test]
    fn test_http_url() {
        fn inner() -> UriResult<IRI> {
            let mut builder = UriBuilder::default();
            let result: UriResult<IRI> = builder
                .scheme(&KnownSchemes::Https.into())
                .user_name("john.doe")
                .host_name("www.example.com")?
                .port(123)
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
        let mut builder = UriBuilder::default();
        let result: UriResult<IRI> = builder
            .scheme(&KnownSchemes::Ldap.into())
            .host_name("[2001:db8::7]")
            .unwrap()
            .path_str("/c=GB")
            .unwrap()
            .query_str("objectClass?one")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(
            iri.to_string(),
            "ldap://[2001:db8::7]/c=GB?objectClass?one".to_string()
        );
    }

    #[test]
    fn test_mailto_uri() {
        let mut builder = UriBuilder::default();
        let result: UriResult<IRI> = builder
            .scheme(&KnownSchemes::Mailto.into())
            .path_str("John.Doe@example.com")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(iri.to_string(), "mailto:John.Doe@example.com".to_string());
    }

    #[test]
    fn test_news_uri() {
        let mut builder = UriBuilder::default();
        let result: UriResult<IRI> = builder
            .scheme(&KnownSchemes::News.into())
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
    fn test_tel_uri() {
        let mut builder = UriBuilder::default();
        let result: UriResult<IRI> = builder
            .scheme(&KnownSchemes::Tel.into())
            .path_str("+1-816-555-1212")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(iri.to_string(), "tel:+1-816-555-1212".to_string());
    }

    #[test]
    fn test_telnet_uri() {
        let mut builder = UriBuilder::default();
        let result: UriResult<IRI> = builder
            .scheme(&KnownSchemes::Telnet.into())
            .ip_address("192.0.2.16")
            .unwrap()
            .port(80)
            .path_str("/")
            .unwrap()
            .try_into();
        assert!(result.is_ok());
        let iri = result.unwrap();
        assert_eq!(iri.to_string(), "telnet://192.0.2.16:80/".to_string());
    }

    #[test]
    fn test_urn_uri() {
        let mut builder = UriBuilder::default();
        let result: UriResult<IRI> = builder
            .scheme(&KnownSchemes::Urn.into())
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
