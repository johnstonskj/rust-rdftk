/*!
* Provides a builder experience for creating `IRI` instances. The [`IriBuilder`](builder/struct.IriBuilder.html)
* type provides a simple API to create new `IRI` instances in a fluent style.
*
* # Example
*
* ```rust
* use rdftk_iri::{builder::IriBuilder, IRI, error::Result as IriResult, Scheme};
* use std::convert::TryInto;
*
* fn make_example_iri() -> IriResult<IRI> {
*     let mut builder = IriBuilder::default();
*     builder
*         .scheme(&Scheme::https())
*         .user_name("john.doe")
*         .host_str("www.example.com")?
*         .port(123.into())
*         .path_str("/forum/questions/")?
*         .query_str("tag=networking&order=newest")?
*         .fragment_str("top")?
*         .try_into()
* }
* ```
*
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
        Ok(self.scheme(&Scheme::from_str(scheme)?))
    }

    /// Use the provided host name, parsed from a string, for this IRI's authority.
    pub fn host(&mut self, host: &Host) -> &mut Self {
        self.host = Some(host.clone());
        self
    }

    /// Use the provided host name, parsed from a string, for this IRI's authority.
    pub fn host_str(&mut self, host: &str) -> IriResult<&mut Self> {
        Ok(self.host(&Host::from_str(host)?))
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
    pub fn path_root(&mut self) -> &mut Self {
        self.path = Some(Path::root());
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
    pub fn append_path_segment(&mut self, segment: &str) -> IriResult<&mut Self> {
        match &mut self.path {
            None => self.path = Some(Path::from_str(segment)?),
            Some(path) => path.push(segment)?,
        }
        Ok(self)
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
