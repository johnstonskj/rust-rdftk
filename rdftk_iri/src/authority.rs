/*!
One-line description.

More detailed description, with

# Example

*/

#![allow(clippy::module_name_repetitions)]

use crate::error::{Component, Error as UriError, ErrorKind, Result as UriResult};
use crate::parse;
use crate::Normalize;
use regex::Regex;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Port = u16;

#[derive(Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum KnownPorts {
    FtpData = 20,
    FtpControl = 21,
    Ssh = 22,
    Telnet = 23,
    Tftp = 69,
    Gopher = 70,
    Http = 80,
    Nntp = 119,
    Imap = 143,
    Snmp = 161,
    SnmpTrap = 162,
    Imap3 = 220,
    Ldap = 389,
    Https = 443,
    Rtsp = 554,
    Ipp = 631,
    IrisBeep = 702,
    Dict = 2628,
    Stun = 3478,
    Diameter = 3868,
    Iax = 4569,
    Sip = 5060,
    Sips = 5061,
    Vnc = 5500,
    Coap = 5683,
    Coaps = 5684,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Host {
    IPV4(String),
    IPV6(String),
    IPVFuture(String, String),
    Name(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Authority {
    host: Option<Host>,
    user_name: Option<String>,
    password: Option<String>,
    port: Option<Port>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TryFrom<Port> for KnownPorts {
    type Error = ();

    fn try_from(port: Port) -> Result<Self, Self::Error> {
        match port {
            20 => Ok(KnownPorts::FtpData),
            21 => Ok(KnownPorts::FtpControl),
            22 => Ok(KnownPorts::Ssh),
            23 => Ok(KnownPorts::Telnet),
            69 => Ok(KnownPorts::Tftp),
            70 => Ok(KnownPorts::Gopher),
            80 => Ok(KnownPorts::Http),
            119 => Ok(KnownPorts::Nntp),
            143 => Ok(KnownPorts::Imap),
            161 => Ok(KnownPorts::Snmp),
            162 => Ok(KnownPorts::SnmpTrap),
            220 => Ok(KnownPorts::Imap3),
            389 => Ok(KnownPorts::Ldap),
            443 => Ok(KnownPorts::Https),
            554 => Ok(KnownPorts::Rtsp),
            631 => Ok(KnownPorts::Ipp),
            702 => Ok(KnownPorts::IrisBeep),
            2628 => Ok(KnownPorts::Dict),
            3478 => Ok(KnownPorts::Stun),
            3868 => Ok(KnownPorts::Diameter),
            4569 => Ok(KnownPorts::Iax),
            5060 => Ok(KnownPorts::Sip),
            5061 => Ok(KnownPorts::Sips),
            5500 => Ok(KnownPorts::Vnc),
            5683 => Ok(KnownPorts::Coap),
            5684 => Ok(KnownPorts::Coaps),
            _ => Err(()),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Host::IPV4(address) => write!(f, "{}", address),
            Host::IPV6(address) => write!(f, "[{}]", address),
            Host::IPVFuture(version, address) => write!(f, "[v{}.{}]", version, address),
            Host::Name(address) => write!(f, "{}", address),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Authority {
    fn default() -> Self {
        Self {
            host: None,
            user_name: None,
            password: None,
            port: None,
        }
    }
}

impl Display for Authority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.is_empty() {
            write!(f, "//")?;
            if let Some(user_name) = &self.user_name {
                write!(f, "{}", user_name)?;
                if let Some(password) = &self.password {
                    write!(f, ":{}", password)?;
                }
                write!(f, "@")?;
            }
            if let Some(host) = &self.host {
                write!(f, "{}", host)?;
            }
            if let Some(port) = &self.port {
                write!(f, ":{}", port)?;
            }
        }
        Ok(())
    }
}

impl FromStr for Authority {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_authority(s)
    }
}

impl Normalize for Authority {
    fn normalize(self) -> UriResult<Self> {
        unimplemented!()
    }
}

impl Authority {
    pub fn with_host(host: Host) -> Self {
        Self {
            host: Some(host),
            user_name: None,
            password: None,
            port: None,
        }
    }
    pub fn with_host_and_port(host: Host, port: Port) -> Self {
        Self {
            host: Some(host),
            user_name: None,
            password: None,
            port: Some(port),
        }
    }
    pub fn with_user_name(user_name: &str) -> Self {
        Self {
            host: None,
            user_name: Some(user_name.to_string()),
            password: None,
            port: None,
        }
    }
    pub fn with_user_name_and_password(user_name: &str, password: &str) -> Self {
        Self {
            host: None,
            user_name: Some(user_name.to_string()),
            password: Some(password.to_string()),
            port: None,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.user_name.is_none()
            && self.password.is_none()
            && self.host.is_none()
            && self.port.is_none()
    }
    pub fn host(&self) -> &Option<Host> {
        &self.host
    }
    pub fn port(&self) -> &Option<Port> {
        &self.port
    }
    pub fn user_name(&self) -> &Option<String> {
        &self.user_name
    }
    pub fn password(&self) -> &Option<String> {
        &self.password
    }

    pub fn set_host(&mut self, host: Host) {
        self.host = Some(host);
    }
    pub fn unset_host(&mut self) {
        self.host = None;
    }

    pub fn set_port(&mut self, port: Port) {
        self.port = Some(port);
    }
    pub fn unset_port(&mut self) {
        self.port = None;
    }

    pub fn set_user_name(&mut self, user_name: &str) -> UriResult<()> {
        if parse::is_iuserinfo(user_name) {
            self.user_name = Some(user_name.to_string());
            Ok(())
        } else {
            Err(ErrorKind::InvalidChar(Component::Authority).into())
        }
    }
    pub fn unset_user_name(&mut self) {
        self.user_name = None;
    }

    pub fn set_password(&mut self, password: &str) -> UriResult<()> {
        if parse::is_iuserinfo(password) {
            self.password = Some(password.to_string());
            Ok(())
        } else {
            Err(ErrorKind::InvalidChar(Component::Authority).into())
        }
    }
    pub fn unset_password(&mut self) {
        self.password = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse_authority(s: &str) -> UriResult<Authority> {
    let parts = s.split('@').collect::<Vec<&str>>();
    match parts.len() {
        1 => {
            if parse::is_ihost(s) {
                let (host, port) = parse_ihost(s)?;
                Ok(Authority {
                    host,
                    user_name: None,
                    password: None,
                    port,
                })
            } else {
                Err(ErrorKind::InvalidChar(Component::Path).into())
            }
        }
        2 => {
            if parse::is_iuserinfo(parts.get(0).unwrap()) && parse::is_ihost(parts.get(1).unwrap())
            {
                let (user_name, password) = parse_iuserinfo(s)?;
                let (host, port) = parse_ihost(s)?;
                Ok(Authority {
                    host,
                    user_name,
                    password,
                    port,
                })
            } else {
                Err(ErrorKind::InvalidChar(Component::Path).into())
            }
        }
        _ => Err(ErrorKind::InvalidChar(Component::Authority).into()),
    }
}

fn parse_iuserinfo(s: &str) -> UriResult<(Option<String>, Option<String>)> {
    let parts = s.split('@').collect::<Vec<&str>>();
    match parts.len() {
        1 => Ok((Some(s.to_string()), None)),
        2 => Ok((
            Some((*parts.get(0).unwrap()).to_string()),
            Some((*parts.get(0).unwrap()).to_string()),
        )),
        _ => Err(ErrorKind::InvalidChar(Component::Authority).into()),
    }
}

fn parse_ihost(s: &str) -> UriResult<(Option<Host>, Option<Port>)> {
    let ipv4 = Regex::new(r"^(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})(:(\d+))?$").unwrap();
    let ipvmore = Regex::new(r"^\[(v([0-9A-Fa-f]+)\.)?([0-9A-Fa-f:]+)](:(\d+))?$").unwrap();

    if let Some(captures) = ipv4.captures(s) {
        Ok((
            Some(Host::IPV4(captures.get(1).unwrap().as_str().to_string())),
            match captures.get(3) {
                None => None,
                Some(port) => Some(Port::from_str(port.as_str()).unwrap()),
            },
        ))
    } else if let Some(captures) = ipvmore.captures(s) {
        if captures.get(2).is_none() {
            Ok((
                Some(Host::IPV6(captures.get(3).unwrap().as_str().to_string())),
                match captures.get(5) {
                    None => None,
                    Some(port) => Some(Port::from_str(port.as_str()).unwrap()),
                },
            ))
        } else {
            Ok((
                Some(Host::IPVFuture(
                    captures.get(2).unwrap().as_str().to_string(),
                    captures.get(3).unwrap().as_str().to_string(),
                )),
                match captures.get(5) {
                    None => None,
                    Some(port) => Some(Port::from_str(port.as_str()).unwrap()),
                },
            ))
        }
    } else {
        Ok((Some(Host::Name(s.to_string())), None))
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
