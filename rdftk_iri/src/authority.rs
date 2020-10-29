#![allow(clippy::module_name_repetitions)]

use crate::error::ResultExt;
use crate::error::{Component, Error as IriError, ErrorKind, Result as IriResult};
use crate::{parse, ValidateStr};
use crate::{Normalize, Scheme};
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This type represents the port component, it is a 16 bit unsigned integer.
///
/// # Examples
///
/// ```rust
/// use rdftk_iri::Port;
/// use std::str::FromStr;
///
/// let http_port: Port = Port::from_str("80").unwrap();
///
/// let https_port: Port = 443.into();
/// ```
///
/// A number of well-known ports are also provided as associated functions on the `Port`
/// implementation for convenience.
///
/// ```rust
/// use rdftk_iri::Port;
/// use rdftk_iri::Scheme;
///
/// let http_port: Port = Port::http();
///
/// let https_port: Port = Port::default_for(&Scheme::https()).unwrap();
/// ```
///
/// Finally, there is a difference between `to_string` that will format the value for inclusion in
/// an `IRI` string, and value that returns the raw port integer.
///
/// ```rust
/// use rdftk_iri::Port;
///
/// let http_port: Port = Port::new(80);
///
/// println!("'{}'", http_port); // prints '80'
/// println!("'{}'", http_port.value()); // prints '80'
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Port(u16);

///
/// This type holds the host details in their parsed form. It is an enumeration of the set of
/// valid host representations allowed by the IRI specification.
///
#[derive(Clone, Debug, Eq)]
pub enum HostKind {
    /// Holds a parsed IPv4 address; e.g. `127.0.0.1`, `192.0.0.10`, `16.38.10.112`.
    IPV4(Ipv4Addr),
    /// Holds a parsed IPv6 address; e.g. `[2001:db8::ff00:42:8329]`.
    IPV6(Ipv6Addr),
    /// Holds a parsed IP future address; e.g. `[v7.2001:db8::ff00:42:8329]`.
    IPVFuture(u16, String),
    /// Holds a validated domain name; e.g. `localhost`, `example.com`, `node01.us.example.org`.
    DomainName(String),
}

///
/// This type wraps the specific [`HostKind`](enum.HostKind.html) and provides a common place for
/// host-related operations.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Host(HostKind);

///
/// The user information sub-component of an IRIs [`Authority`](struct.Authority.html).
///
/// # Example
///
/// ```rust
/// use rdftk_iri::authority::{UserInfo};
///
/// let user = UserInfo::new("John.Doe").unwrap();
///
/// assert!(!user.has_password());
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserInfo {
    user_name: String,
    password: Option<String>,
}

///
/// Provides the `Authority` component of an `IRI` comprising host, user information, and port
/// sub-components. All but the host sub-components are optional.
///
/// 1. The required host component is either a domain name, or IP address in IPV4 or IPV6 format.
/// 2. The optional user information component consists of a required user name and optional password.
/// 3. The optional port is simply an unsigned integer.
///
/// # Example
///
/// ```rust
/// use rdftk_iri::authority::{Authority, Host, Port};
/// use std::str::FromStr;
///
/// let http_authority = Authority::new_with_port(
///         Host::from_str("www.example.com").unwrap(),
///         Port::http()
///     );
///
/// assert!(http_authority.has_port());
/// assert!(!http_authority.has_user_info());
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Authority {
    user_info: Option<UserInfo>,
    host: Host,
    port: Option<Port>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref IPV4: Regex =
        Regex::new(r"^(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})(:(\d+))?$").unwrap();
    static ref IPVMORE: Regex =
        Regex::new(r"^\[(v([0-9A-Fa-f]+)\.)?([0-9A-Fa-f:]+)](:(\d+))?$").unwrap();
    static ref IP_FUTURE: Regex = Regex::new(r"^[0-9A-Fa-f:]+$").unwrap();
}

impl Display for Port {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}", self.0)
    }
}

impl FromStr for Port {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match u16::from_str(s) {
            Ok(port) => Ok(Self(port)),
            Err(e) => Err(IriError::with_chain(
                e,
                ErrorKind::ParsePortError(s.to_string()),
            )),
        }
    }
}

impl From<u16> for Port {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl Port {
    ///
    /// Construct a new `Port` instance from the raw port value.
    ///
    pub fn new(raw_port: u16) -> Self {
        Self(raw_port)
    }

    /// Well-known port for FTP protocol.
    pub fn ftp_data() -> Port {
        20.into()
    }

    /// Well-known port for FTP protocol.
    pub fn ftp_control() -> Port {
        21.into()
    }

    /// Well-known port for SSH protocol.
    pub fn ssh() -> Port {
        22.into()
    }

    /// Well-known port for Telnet protocol.
    pub fn telnet() -> Port {
        23.into()
    }

    /// Well-known port for TFTP protocol.
    pub fn tftp() -> Port {
        69.into()
    }

    /// Well-known port for Gopher protocol.
    pub fn gopher() -> Port {
        70.into()
    }

    /// Well-known port for HTTP protocol.
    pub fn http() -> Port {
        80.into()
    }

    /// Well-known port for NNTP protocol.
    pub fn nntp() -> Port {
        119.into()
    }

    /// Well-known port for IMAP protocol.
    pub fn imap() -> Port {
        143.into()
    }

    /// Well-known port for SNMP protocol.
    pub fn snmp() -> Port {
        161.into()
    }

    /// Well-known port for SNMP protocol.
    pub fn snmp_trap() -> Port {
        162.into()
    }

    /// Well-known port for IMAP protocol.
    pub fn imap3() -> Port {
        220.into()
    }

    /// Well-known port for LDAP protocol.
    pub fn ldap() -> Port {
        389.into()
    }

    /// Well-known port for HTTPS protocol.
    pub fn https() -> Port {
        443.into()
    }

    /// Well-known port for RTSP protocol.
    pub fn rtsp() -> Port {
        554.into()
    }

    /// Well-known port for IPP protocol.
    pub fn ipp() -> Port {
        631.into()
    }

    /// Well-known port for BEEP protocol.
    pub fn iris_beep() -> Port {
        702.into()
    }

    /// Well-known port for Dict protocol.
    pub fn dict() -> Port {
        2628.into()
    }

    /// Well-known port for STUN protocol.
    pub fn stun() -> Port {
        3478.into()
    }

    /// Well-known port for Diameter protocol.
    pub fn diameter() -> Port {
        3868.into()
    }

    /// Well-known port for IAX protocol.
    pub fn iax() -> Port {
        4569.into()
    }

    /// Well-known port for SIP protocol.
    pub fn sip() -> Port {
        5060.into()
    }

    /// Well-known port for SIPS protocol.
    pub fn sips() -> Port {
        5061.into()
    }

    /// Well-known port for VNC protocol.
    pub fn vnc() -> Port {
        5500.into()
    }

    /// Well-known port for COAP protocol.
    pub fn coap() -> Port {
        5683.into()
    }

    /// Well-known port for COAP protocol.
    pub fn coaps() -> Port {
        5684.into()
    }

    ///
    /// Return the default port for the provided `Scheme`, if one is known.
    ///
    pub fn default_for(scheme: &Scheme) -> Option<Port> {
        let scheme = scheme.value();
        match scheme.as_str() {
            "ftp" => Some(Self::ftp_data()),
            "ssh" => Some(Self::ssh()),
            "telnet" => Some(Self::telnet()),
            "tftp" => Some(Self::tftp()),
            "gopher" => Some(Self::gopher()),
            "http" => Some(Self::http()),
            "nntp" => Some(Self::nntp()),
            "imap" => Some(Self::imap()),
            "snmp" => Some(Self::snmp()),
            "ldap" => Some(Self::ldap()),
            "https" => Some(Self::https()),
            "rtsp" => Some(Self::rtsp()),
            "ipp" => Some(Self::ipp()),
            "iris.beep" => Some(Self::iris_beep()),
            "dict" => Some(Self::dict()),
            "stun" => Some(Self::stun()),
            "aaa" => Some(Self::diameter()),
            "iax" => Some(Self::iax()),
            "sip" => Some(Self::sip()),
            "sips" => Some(Self::sips()),
            "vnc" => Some(Self::vnc()),
            "coap" => Some(Self::coap()),
            "coaps" => Some(Self::coaps()),

            "ws" => Some(Self::http()),
            "wss" => Some(Self::https()),

            _ => None,
        }
    }

    /// Return the raw port value.
    pub fn value(&self) -> &u16 {
        &self.0
    }
}

// ------------------------------------------------------------------------------------------------

impl PartialEq for HostKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DomainName(lhs), Self::DomainName(rhs)) => {
                lhs.to_lowercase() == rhs.to_lowercase()
            }
            (Self::IPV4(lhs), Self::IPV4(rhs)) => lhs == rhs,
            (Self::IPV6(lhs), Self::IPV6(rhs)) => lhs == rhs,
            (Self::IPVFuture(lv, ld), Self::IPVFuture(rv, rd)) => {
                lv == rv && ld.to_uppercase() == rd.to_uppercase()
            }
            _ => false,
        }
    }
}

impl Hash for HostKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::DomainName(v) => v.to_lowercase().hash(state),
            Self::IPV4(v) => v.hash(state),
            Self::IPV6(v) => v.hash(state),
            Self::IPVFuture(v, vv) => {
                v.hash(state);
                vv.to_uppercase().hash(state)
            }
        }
    }
}

impl Display for HostKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HostKind::IPV4(address) => write!(f, "{}", address),
            HostKind::IPV6(address) => write!(f, "[{}]", address),
            HostKind::IPVFuture(version, address) => write!(f, "[v{:X}.{}]", version, address),
            HostKind::DomainName(address) => write!(f, "{}", address),
        }
    }
}

impl Normalize for HostKind {
    fn normalize(self) -> IriResult<Self>
    where
        Self: Sized,
    {
        // SPEC: RFC-3986 §6.2.2
        Ok(match self {
            HostKind::IPVFuture(version, address) => {
                HostKind::IPVFuture(version, address.to_uppercase())
            }
            HostKind::DomainName(name) => HostKind::DomainName(name.to_lowercase()),
            _ => self,
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Host {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (host, port) = parse_ihost(s)?;
        if port.is_some() {
            Err(ErrorKind::ParseHostError(s.to_string()).into())
        } else {
            Ok(host)
        }
    }
}

impl ValidateStr for Host {
    fn is_valid(s: &str) -> bool {
        parse::is_ihost(s)
    }
}

impl Normalize for Host {
    fn normalize(self) -> IriResult<Self>
    where
        Self: Sized,
    {
        self.0.normalize().map(Self)
    }
}

impl Host {
    ///
    /// Construct a new `Host` if `name` is a valid domain name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rdftk_iri::Host;
    ///
    /// let host = Host::new_domain_name("www.example.com").unwrap();
    ///
    /// assert!(host.is_domain_name());
    /// ```
    ///
    pub fn new_domain_name(name: &str) -> IriResult<Self> {
        if parse::is_ireg_name(name) {
            Ok(Self(HostKind::DomainName(name.to_string())))
        } else {
            Err(ErrorKind::ParseHostError(name.to_string()).into())
        }
    }

    ///
    /// Construct a new `Host` if `address` is a valid IPv4 address representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rdftk_iri::Host;
    /// use std::net::Ipv4Addr;
    ///
    /// let host = Host::new_ipv4_address(Ipv4Addr::new(127, 0, 0, 1)).unwrap();
    ///
    /// assert!(host.is_ipv4_address());
    /// ```
    ///
    pub fn new_ipv4_address(address: Ipv4Addr) -> IriResult<Self> {
        Ok(Self(HostKind::IPV4(address)))
    }

    ///
    /// Construct a new `Host` if `address` is a valid IPv6 address representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rdftk_iri::Host;
    /// use std::net::Ipv6Addr;
    ///
    /// let host = Host::new_ipv6_address(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)).unwrap();
    ///
    /// assert!(host.is_ipv6_address());
    /// ```
    ///
    pub fn new_ipv6_address(address: Ipv6Addr) -> IriResult<Self> {
        Ok(Self(HostKind::IPV6(address)))
    }

    ///
    /// Construct a new `Host` if `address` is a valid IP Future address representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rdftk_iri::Host;
    ///
    /// let host = Host::new_ipv_future_address(7, "::1").unwrap();
    ///
    /// assert!(host.is_ip_future_address());
    /// ```
    ///
    pub fn new_ipv_future_address(version: u16, address: &str) -> IriResult<Self> {
        if IP_FUTURE.is_match(address) {
            Ok(Self(HostKind::IPVFuture(version, address.to_string())))
        } else {
            Err(ErrorKind::ParseIpAddressError(address.to_string()).into())
        }
    }

    /// Returns `true` if this is a named host, else `false`.
    pub fn is_domain_name(&self) -> bool {
        match &self.0 {
            HostKind::DomainName(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is an IPv4 address, else `false`.
    pub fn is_ipv4_address(&self) -> bool {
        match &self.0 {
            HostKind::IPV4(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is an IPv6 address, else `false`.
    pub fn is_ipv6_address(&self) -> bool {
        match &self.0 {
            HostKind::IPV6(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is an IPvFuture address, else `false`.
    pub fn is_ip_future_address(&self) -> bool {
        match &self.0 {
            HostKind::IPVFuture(_, _) => true,
            _ => false,
        }
    }

    ///
    /// Return the enumeration that contains the actual host address value.
    ///
    pub fn value(&self) -> &HostKind {
        &self.0
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for UserInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.user_name)?;
        if let Some(password) = &self.password {
            write!(f, "{}", password)?;
        }
        write!(f, "@")
    }
}

impl FromStr for UserInfo {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (user_name, password) = parse_iuserinfo(s)?;
        Ok(Self {
            user_name,
            password,
        })
    }
}

impl UserInfo {
    ///
    /// Construct a new `UserInfo` instance with only the user's name specified.
    ///
    pub fn new(user_name: &str) -> IriResult<Self> {
        if !user_name.is_empty() && parse::is_iuserinfo(user_name) {
            Ok(Self {
                user_name: user_name.to_string(),
                password: None,
            })
        } else {
            Err(ErrorKind::InvalidChar(Component::Authority).into())
        }
    }

    ///
    /// Construct a new `UserInfo` instance with only the user's name and password specified.
    ///
    pub fn new_with_password(user_name: &str, password: &str) -> IriResult<Self> {
        if !user_name.is_empty()
            && parse::is_iuserinfo(user_name)
            && !password.is_empty()
            && parse::is_iuserinfo(password)
        {
            Ok(Self {
                user_name: user_name.to_string(),
                password: Some(password.to_string()),
            })
        } else {
            Err(ErrorKind::InvalidChar(Component::Authority).into())
        }
    }

    ///
    /// Return the user's name.
    ///
    pub fn user_name(&self) -> &String {
        &self.user_name
    }

    ///
    /// Return `true` if this instance has a password, else `false`.
    ///
    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    ///
    /// Return the password in this instance, if present.
    ///
    pub fn password(&self) -> &Option<String> {
        &self.password
    }

    ///
    /// Set the user name value.
    ///
    pub fn set_user_name(&mut self, user_name: &str) -> IriResult<()> {
        if parse::is_iuserinfo(user_name) {
            self.user_name = user_name.to_string();
            Ok(())
        } else {
            Err(ErrorKind::InvalidChar(Component::Authority).into())
        }
    }

    ///
    /// Set the password value.
    ///
    pub fn set_password(&mut self, password: &str) -> IriResult<()> {
        if parse::is_iuserinfo(password) {
            self.password = Some(password.to_string());
            Ok(())
        } else {
            Err(ErrorKind::InvalidChar(Component::Authority).into())
        }
    }

    ///
    /// Set the password value to `None`.
    pub fn unset_password(&mut self) {
        self.password = None
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Authority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "//")?;
        if let Some(user_info) = &self.user_info {
            write!(f, "{}", user_info)?;
        }

        write!(f, "{}", self.host)?;

        if let Some(port) = &self.port {
            write!(f, "{}", port)?;
        }
        Ok(())
    }
}

impl FromStr for Authority {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_authority(s)
    }
}

impl ValidateStr for Authority {}

impl Normalize for Authority {
    fn normalize(self) -> IriResult<Self> {
        Ok(Self {
            host: self.host.normalize()?,
            ..self
        })
    }
}

impl Authority {
    ///
    /// Construct a new `Authority` instance with only the specified `Host` value. Note that the
    /// host is required, other sub-components are optional.
    ///
    pub fn new(host: Host) -> Self {
        Self {
            host,
            user_info: None,
            port: None,
        }
    }

    ///
    /// Construct a new `Authority` instance with only the required `Host` value and the optional
    /// `Port` value.
    ///
    pub fn new_with_port(host: Host, port: Port) -> Self {
        Self {
            host,
            user_info: None,
            port: Some(port),
        }
    }

    ///
    /// Construct a new `Authority` instance with only the required `Host` value and the optional
    /// `UserInfo` value.
    ///
    pub fn new_with_user_info(host: Host, user_info: UserInfo) -> Self {
        Self {
            host,
            user_info: Some(user_info),
            port: None,
        }
    }

    ///
    /// Construct a new `Authority` instance with only the required `Host` value, the optional
    /// `Port` value, and optional `UserInfo` value.
    ///
    pub fn new_with_port_and_user_info(host: Host, port: Port, user_info: UserInfo) -> Self {
        Self {
            host,
            user_info: Some(user_info),
            port: Some(port),
        }
    }

    /// Return the current host value.
    pub fn host(&self) -> &Host {
        &self.host
    }

    /// Return `true` if this authority has a port value, else `false`.
    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    /// Return the current port value, if present.
    pub fn port(&self) -> &Option<Port> {
        &self.port
    }

    /// Return the current user info value, if present.
    pub fn user_info(&self) -> &Option<UserInfo> {
        &self.user_info
    }

    /// Return `true` if this authority has a user info value, else `false`.
    pub fn has_user_info(&self) -> bool {
        self.user_info.is_some()
    }

    /// Set a new value for host.
    pub fn set_host(&mut self, host: Host) {
        self.host = host;
    }

    /// Set a new value for port.
    pub fn set_port(&mut self, port: Port) {
        self.port = Some(port);
    }

    /// Set the value for port to `None`.
    pub fn unset_port(&mut self) {
        self.port = None;
    }

    /// Set a new value for user info.
    pub fn set_user_info(&mut self, user_info: UserInfo) {
        self.user_info = Some(user_info);
    }

    /// Set the value for user info to `None`.
    pub fn unset_user_info(&mut self) {
        self.user_info = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_authority(s: &str) -> IriResult<Authority> {
    let parts = s.split('@').collect::<Vec<&str>>();
    match parts.len() {
        1 => {
            let (host, port) = parse_ihost(s)?;
            if let Some(port) = port {
                Ok(Authority::new_with_port(host, port))
            } else {
                Ok(Authority::new(host))
            }
        }
        2 => {
            let user_info = match parse_iuserinfo(parts.get(0).unwrap())? {
                (user_name, None) => UserInfo::new(&user_name)?,
                (user_name, Some(password)) => UserInfo::new_with_password(&user_name, &password)?,
            };
            let (host, port) = parse_ihost(parts.get(1).unwrap())?;
            if let Some(port) = port {
                Ok(Authority::new_with_port_and_user_info(
                    host, port, user_info,
                ))
            } else {
                Ok(Authority::new_with_user_info(host, user_info))
            }
        }
        _ => Err(ErrorKind::ParseAuthorityError(s.to_string()).into()),
    }
}

fn parse_iuserinfo(s: &str) -> IriResult<(String, Option<String>)> {
    let parts = s.split('@').collect::<Vec<&str>>();
    if parts.iter().all(|s| parse::is_iuserinfo(s)) {
        match parts.len() {
            1 => Ok((s.to_string(), None)),
            2 => Ok((
                (*parts.get(0).unwrap()).to_string(),
                Some((*parts.get(0).unwrap()).to_string()),
            )),
            _ => Err(ErrorKind::ParseUserInfoError(s.to_string()).into()),
        }
    } else {
        Err(ErrorKind::InvalidChar(Component::Authority).into())
    }
}

fn parse_ihost(s: &str) -> IriResult<(Host, Option<Port>)> {
    if let Some(captures) = IPV4.captures(s) {
        let address = captures.get(1).unwrap().as_str();
        Ok((
            Host(HostKind::IPV4(address.parse().chain_err(|| {
                ErrorKind::ParseIpAddressError(address.to_string())
            })?)),
            match captures.get(3) {
                None => None,
                Some(port) => Some(Port::from_str(port.as_str()).unwrap()),
            },
        ))
    } else if let Some(captures) = IPVMORE.captures(s) {
        if captures.get(2).is_none() {
            let address = captures.get(3).unwrap().as_str();
            Ok((
                Host(HostKind::IPV6(address.parse().chain_err(|| {
                    ErrorKind::ParseIpAddressError(address.to_string())
                })?)),
                match captures.get(5) {
                    None => None,
                    Some(port) => Some(Port::from_str(port.as_str()).unwrap()),
                },
            ))
        } else {
            let version = captures.get(1).unwrap().as_str();
            Ok((
                Host(HostKind::IPVFuture(
                    version
                        .parse()
                        .chain_err(|| ErrorKind::ParseIpAddressError(version.to_string()))?,
                    captures.get(3).unwrap().as_str().to_string(),
                )),
                match captures.get(5) {
                    None => None,
                    Some(port) => Some(Port::from_str(port.as_str()).unwrap()),
                },
            ))
        }
    } else {
        let parts = s.split(':').collect::<Vec<&str>>();
        match parts.len() {
            1 => {
                if parse::is_ireg_name(s) {
                    Ok((Host(HostKind::DomainName(s.to_string())), None))
                } else {
                    Err(ErrorKind::ParseHostError(s.to_string()).into())
                }
            }
            2 => {
                let host = parts.get(0).unwrap();
                let port = Port::from_str(parts.get(1).unwrap());
                if parse::is_ireg_name(host) && port.is_ok() {
                    Ok((
                        Host(HostKind::DomainName(host.to_string())),
                        Some(port.unwrap()),
                    ))
                } else {
                    Err(ErrorKind::ParseHostError(s.to_string()).into())
                }
            }
            _ => Err(ErrorKind::ParseHostError(s.to_string()).into()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_port_from_str() {
        assert!(Port::from_str("1").is_ok());
        assert!(Port::from_str("80").is_ok());
        assert!(Port::from_str("8080").is_ok());

        assert!(Port::from_str("http").is_err());
        assert!(Port::from_str("-1").is_err());
        assert!(Port::from_str("8888888888").is_err());
    }

    #[test]
    fn test_port_default_for() {
        assert!(Port::default_for(&Scheme::https()).is_some());

        assert!(Port::default_for(&Scheme::mailto()).is_none());
    }

    #[test]
    fn test_port_display() {
        assert_eq!(Port::new(443).to_string(), ":443");
    }
}
