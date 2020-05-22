/*!
One-line description.

More detailed description, with

# Example

*/

#![allow(clippy::module_name_repetitions)]

use crate::error::{Component, Error as UriError, ErrorKind, Result as UriResult};
use crate::parse;
use crate::Normalize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scheme {
    inner: String,
}

// From https://en.wikipedia.org/wiki/List_of_URI_schemes
// where status = permanent
//   and defined_by = known org
pub enum KnownSchemes {
    Aaa,
    Aaas,
    About,
    Acap,
    Acct,
    Cap,
    Cid,
    Coap,
    Coaps,
    Crid,
    Data,
    Dav,
    Dict,
    Dns,
    Example,
    File,
    Ftp,
    Geo,
    Go,
    Gopher,
    Http,
    Https,
    Iax,
    Icap,
    Im,
    Imap,
    Info,
    Ipp,
    Ipps,
    Iris,
    IrisBeep,
    IrisXpc,
    IrosXpcs,
    IrisLws,
    Ldap,
    Mailto,
    Mid,
    Msrp,
    Msrps,
    Mtqp,
    MUpdate,
    News,
    Nfs,
    Ni,
    Nih,
    Nntp,
    OpaqueLockToken,
    Pkcs11,
    Pop,
    Pres,
    Reload,
    Rtsp,
    Service,
    Session,
    Shttp,
    Sieve,
    Sip,
    Sips,
    Sms,
    Snmp,
    SoapBeep,
    SoapBeeps,
    Stun,
    Stuns,
    Tag,
    Tel,
    Telnet,
    Tftp,
    ThisMessage,
    Tn3270,
    Tip,
    Turn,
    Turns,
    Tv,
    Urn,
    Vemmi,
    Vnc,
    Ws,
    Wss,
    Xcon,
    XconUserID,
    XmlRpcBeep,
    XmlRpcBeeps,
    Xmpp,
    Z3950r,
    Z3950s,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Scheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.inner)
    }
}

impl FromStr for Scheme {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if parse::is_scheme(s) {
            Ok(Self {
                inner: s.to_string(),
            })
        } else {
            Err(ErrorKind::InvalidChar(Component::Scheme).into())
        }
    }
}

impl Normalize for Scheme {
    fn normalize(self) -> UriResult<Self> {
        Ok(Self {
            inner: self.inner.to_lowercase(),
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for KnownSchemes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KnownSchemes::Aaa => "aaa",
                KnownSchemes::Aaas => "aaas",
                KnownSchemes::About => "about",
                KnownSchemes::Acap => "acap",
                KnownSchemes::Acct => "acct",
                KnownSchemes::Cap => "cap",
                KnownSchemes::Cid => "cid",
                KnownSchemes::Coap => "coap",
                KnownSchemes::Coaps => "coaps",
                KnownSchemes::Crid => "crid",
                KnownSchemes::Data => "data",
                KnownSchemes::Dav => "dav",
                KnownSchemes::Dict => "dict",
                KnownSchemes::Dns => "dns",
                KnownSchemes::Example => "example",
                KnownSchemes::File => "file",
                KnownSchemes::Ftp => "ftp",
                KnownSchemes::Geo => "geo",
                KnownSchemes::Go => "go",
                KnownSchemes::Gopher => "gopher",
                KnownSchemes::Http => "http",
                KnownSchemes::Https => "https",
                KnownSchemes::Iax => "iax",
                KnownSchemes::Icap => "icap",
                KnownSchemes::Im => "im",
                KnownSchemes::Imap => "imap",
                KnownSchemes::Info => "info",
                KnownSchemes::Ipp => "ipp",
                KnownSchemes::Ipps => "ipps",
                KnownSchemes::Iris => "iris",
                KnownSchemes::IrisBeep => "iris.beep",
                KnownSchemes::IrisXpc => "iris.xpc",
                KnownSchemes::IrosXpcs => "iris.xpcs",
                KnownSchemes::IrisLws => "iris.lws",
                KnownSchemes::Ldap => "ldap",
                KnownSchemes::Mailto => "mailto",
                KnownSchemes::Mid => "mid",
                KnownSchemes::Msrp => "msrp",
                KnownSchemes::Msrps => "msrps",
                KnownSchemes::Mtqp => "mtqp",
                KnownSchemes::MUpdate => "mupdate",
                KnownSchemes::News => "news",
                KnownSchemes::Nfs => "nfs",
                KnownSchemes::Ni => "ni",
                KnownSchemes::Nih => "nih",
                KnownSchemes::Nntp => "nntp",
                KnownSchemes::OpaqueLockToken => "opaquelocktoken",
                KnownSchemes::Pkcs11 => "pkcs11",
                KnownSchemes::Pop => "pop",
                KnownSchemes::Pres => "pres",
                KnownSchemes::Reload => "reload",
                KnownSchemes::Rtsp => "rtsp",
                KnownSchemes::Service => "service",
                KnownSchemes::Session => "session",
                KnownSchemes::Shttp => "shhtp",
                KnownSchemes::Sieve => "sieve",
                KnownSchemes::Sip => "sip",
                KnownSchemes::Sips => "sips",
                KnownSchemes::Sms => "sms",
                KnownSchemes::Snmp => "snmp",
                KnownSchemes::SoapBeep => "soap.beep",
                KnownSchemes::SoapBeeps => "soap.beeps",
                KnownSchemes::Stun => "stun",
                KnownSchemes::Stuns => "stuns",
                KnownSchemes::Tag => "tag",
                KnownSchemes::Tel => "tel",
                KnownSchemes::Telnet => "telnet",
                KnownSchemes::Tftp => "tftp",
                KnownSchemes::ThisMessage => "thismessage",
                KnownSchemes::Tn3270 => "tn3270",
                KnownSchemes::Tip => "tip",
                KnownSchemes::Turn => "turn",
                KnownSchemes::Turns => "turns",
                KnownSchemes::Tv => "tv",
                KnownSchemes::Urn => "urn",
                KnownSchemes::Vemmi => "vemmi",
                KnownSchemes::Vnc => "vnc",
                KnownSchemes::Ws => "ws",
                KnownSchemes::Wss => "wss",
                KnownSchemes::Xcon => "xcon",
                KnownSchemes::XconUserID => "xcon-userid",
                KnownSchemes::XmlRpcBeep => "xmlrpc.beep",
                KnownSchemes::XmlRpcBeeps => "xmlrpc.beeps",
                KnownSchemes::Xmpp => "xmpp",
                KnownSchemes::Z3950r => "z39.50r",
                KnownSchemes::Z3950s => "z39.50s",
            }
        )
    }
}

impl FromStr for KnownSchemes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ftp" => Ok(KnownSchemes::Ftp),
            "http" => Ok(KnownSchemes::Http),
            "https" => Ok(KnownSchemes::Https),
            "mailto" => Ok(KnownSchemes::Mailto),
            "telnet" => Ok(KnownSchemes::Telnet),
            "urn" => Ok(KnownSchemes::Urn),
            _ => Err(()),
        }
    }
}

impl Into<Scheme> for KnownSchemes {
    fn into(self) -> Scheme {
        Scheme {
            inner: self.to_string(),
        }
    }
}

impl KnownSchemes {
    pub fn purpose(&self) -> String {
        match self {
            KnownSchemes::Aaa => "Diameter Protocol",
            KnownSchemes::Aaas => "Diameter Protocol",
            KnownSchemes::About => "Product Information",
            KnownSchemes::Acap => "Application Configuration Access Protocol",
            KnownSchemes::Acct => "Identifying user account",
            KnownSchemes::Cap => "Calendar Access Protocol",
            KnownSchemes::Cid => "Referencing individual parts of an SMTP/MIME message",
            KnownSchemes::Coap => "Constrained Application Protocol",
            KnownSchemes::Coaps => "Constrained Application Protocol",
            KnownSchemes::Crid => "TV-Anytime Content Reference Identifier",
            KnownSchemes::Data => "Inclusion of small data items inline",
            KnownSchemes::Dav => "HTTP Extensions for Distributed Authoring (WebDAV)",
            KnownSchemes::Dict => "Dictionary Service Protocol",
            KnownSchemes::Dns => "Domain Name System",
            KnownSchemes::Example => "For examples",
            KnownSchemes::File => "Addressing files on local or network file systems",
            KnownSchemes::Ftp => "File Transfer Protocol",
            KnownSchemes::Geo => "A Uniform Resource Identifier for Geographic Locations",
            KnownSchemes::Go => "Common Name Resolution Protocol",
            KnownSchemes::Gopher => "Used with Gopher protocol",
            KnownSchemes::Http => "HTTP resources",
            KnownSchemes::Https => "HTTP resources secured using SSL/TLS",
            KnownSchemes::Iax => "Inter-Asterisk eXchange protocol",
            KnownSchemes::Icap => "Internet Content Adaptation Protocol",
            KnownSchemes::Im => "Instant messaging protocol",
            KnownSchemes::Imap => "Accessing e-mail resources through IMAP",
            KnownSchemes::Info => "Information Assets with Identifiers in Public Namespaces",
            KnownSchemes::Ipp => "Internet Printing Protocol",
            KnownSchemes::Ipps => "Internet Printing Protocol over HTTPS",
            KnownSchemes::Iris => "Internet Registry Information Service",
            KnownSchemes::IrisBeep => "Internet Registry Information Service",
            KnownSchemes::IrisXpc => "Internet Registry Information Service",
            KnownSchemes::IrosXpcs => "Internet Registry Information Service",
            KnownSchemes::IrisLws => "Internet Registry Information Service",
            KnownSchemes::Ldap => "LDAP directory request",
            KnownSchemes::Mailto => "SMTP e-mail addresses and default content",
            KnownSchemes::Mid => "Referencing SMTP/MIME messages, or parts of messages",
            KnownSchemes::Msrp => "Message Session Relay Protocol",
            KnownSchemes::Msrps => "",
            KnownSchemes::Mtqp => "Message Tracking Query Protocol",
            KnownSchemes::MUpdate => "Mailbox Update Protocol",
            KnownSchemes::News => "(Usenet) newsgroups and postings",
            KnownSchemes::Nfs => "Network File System resources",
            KnownSchemes::Ni => "Named Information",
            KnownSchemes::Nih => "Named Information for Humans",
            KnownSchemes::Nntp => "Usenet NNTP",
            KnownSchemes::OpaqueLockToken => "WebDAV lock token",
            KnownSchemes::Pkcs11 => "PKCS #11",
            KnownSchemes::Pop => "Accessing mailbox through POP3",
            KnownSchemes::Pres => "Used in Common Profile for Presence (CPP) to identify presence",
            KnownSchemes::Reload => "REsource LOcation And Discovery Protocol",
            KnownSchemes::Rtsp => "Real Time Streaming Protocol",
            KnownSchemes::Service => "Used in Service Location Protocol",
            KnownSchemes::Session => "Media Resource Control Protocol",
            KnownSchemes::Shttp => "Secure HTTP",
            KnownSchemes::Sieve => "ManageSieve protocol",
            KnownSchemes::Sip => "Used with Session Initiation Protocol (SIP)",
            KnownSchemes::Sips => "Secure equivalent of sip",
            KnownSchemes::Sms => "Interact with SMS capable devices for messaging",
            KnownSchemes::Snmp => "Simple Network Management Protocol",
            KnownSchemes::SoapBeep => "SOAP binding to BEEP",
            KnownSchemes::SoapBeeps => "SOAP binding to BEEP",
            KnownSchemes::Stun => "Session Traversal Utilities for NAT (STUN)",
            KnownSchemes::Stuns => "Session Traversal Utilities for NAT (STUN)",
            KnownSchemes::Tag => "The Tag URI",
            KnownSchemes::Tel => "Used for telephone numbers",
            KnownSchemes::Telnet => "Used with telnet",
            KnownSchemes::Tftp => "Trivial File Transfer Protocol",
            KnownSchemes::ThisMessage => "multipart/related relative reference resolution",
            KnownSchemes::Tn3270 => "Interactive 3270 emulation sessions",
            KnownSchemes::Tip => "Transaction Internet Protocol",
            KnownSchemes::Turn => "Traversal Using Relays around NAT (TURN)	",
            KnownSchemes::Turns => "Traversal Using Relays around NAT (TURN)	",
            KnownSchemes::Tv => "TV Broadcasts",
            KnownSchemes::Urn => "	Uniform Resource Names",
            KnownSchemes::Vemmi => "Versatile Multimedia Interface",
            KnownSchemes::Vnc => "	Virtual Network Computing",
            KnownSchemes::Ws => "WebSocket protocol",
            KnownSchemes::Wss => "WebSocket protocol",
            KnownSchemes::Xcon => "Centralized Conferencing (XCON) over SIP",
            KnownSchemes::XconUserID => "Centralized Conferencing (XCON) over SIP",
            KnownSchemes::XmlRpcBeep => "XML-RPC in BEEP",
            KnownSchemes::XmlRpcBeeps => "XML-RPC in BEEP",
            KnownSchemes::Xmpp => "Extensible Messaging and Presence Protocol (XMPP)",
            KnownSchemes::Z3950r => "Z39.50 retrieval",
            KnownSchemes::Z3950s => "Z39.50 session",
        }
        .to_string()
    }

    pub fn rfc(&self) -> String {
        match self {
            KnownSchemes::Aaa => "3588,6733",
            KnownSchemes::Aaas => "3588,6733",
            KnownSchemes::About => "6694",
            KnownSchemes::Acap => "2244",
            KnownSchemes::Acct => "7565",
            KnownSchemes::Cap => "4324",
            KnownSchemes::Cid => "2111,2392",
            KnownSchemes::Coap => "7252",
            KnownSchemes::Coaps => "7252",
            KnownSchemes::Crid => "4078",
            KnownSchemes::Data => "2397",
            KnownSchemes::Dav => "2518,4918",
            KnownSchemes::Dict => "2229",
            KnownSchemes::Dns => "4501",
            KnownSchemes::Example => "7595",
            KnownSchemes::File => "1738,3986",
            KnownSchemes::Ftp => "1738",
            KnownSchemes::Geo => "5870",
            KnownSchemes::Go => "3368",
            KnownSchemes::Gopher => "4266",
            KnownSchemes::Http => "2817,7230",
            KnownSchemes::Https => "2817,7230",
            KnownSchemes::Iax => "5456",
            KnownSchemes::Icap => "3507",
            KnownSchemes::Im => "3860",
            KnownSchemes::Imap => "2192,5092",
            KnownSchemes::Info => "4452",
            KnownSchemes::Ipp => "3510",
            KnownSchemes::Ipps => "7472",
            KnownSchemes::Iris => "3981",
            KnownSchemes::IrisBeep => "3983",
            KnownSchemes::IrisXpc => "4992",
            KnownSchemes::IrosXpcs => "4992",
            KnownSchemes::IrisLws => "4993",
            KnownSchemes::Ldap => "2255,4516",
            KnownSchemes::Mailto => "6068",
            KnownSchemes::Mid => "2111,2392",
            KnownSchemes::Msrp => "4975",
            KnownSchemes::Msrps => "4975",
            KnownSchemes::Mtqp => "3887",
            KnownSchemes::MUpdate => "3656",
            KnownSchemes::News => "1738,5538",
            KnownSchemes::Nfs => "2224",
            KnownSchemes::Ni => "6920",
            KnownSchemes::Nih => "6920",
            KnownSchemes::Nntp => "1738,5538",
            KnownSchemes::OpaqueLockToken => "2518,4918",
            KnownSchemes::Pkcs11 => "7512",
            KnownSchemes::Pop => "2358",
            KnownSchemes::Pres => "3859",
            KnownSchemes::Reload => "6940",
            KnownSchemes::Rtsp => "2326",
            KnownSchemes::Service => "2609",
            KnownSchemes::Session => "4463,6787",
            KnownSchemes::Shttp => "2660",
            KnownSchemes::Sieve => "5804",
            KnownSchemes::Sip => "2543,3969,3261",
            KnownSchemes::Sips => "3969,3261",
            KnownSchemes::Sms => "5724",
            KnownSchemes::Snmp => "4088",
            KnownSchemes::SoapBeep => "3288,4227",
            KnownSchemes::SoapBeeps => "3288,4227",
            KnownSchemes::Stun => "7064",
            KnownSchemes::Stuns => "7064",
            KnownSchemes::Tag => "4151",
            KnownSchemes::Tel => "5341,3966,2806",
            KnownSchemes::Telnet => "1738,4248",
            KnownSchemes::Tftp => "3627",
            KnownSchemes::ThisMessage => "2557",
            KnownSchemes::Tn3270 => "6270",
            KnownSchemes::Tip => "2371",
            KnownSchemes::Turn => "7065",
            KnownSchemes::Turns => "7065",
            KnownSchemes::Tv => "2838",
            KnownSchemes::Urn => "2141",
            KnownSchemes::Vemmi => "2122",
            KnownSchemes::Vnc => "7869",
            KnownSchemes::Ws => "6455",
            KnownSchemes::Wss => "6455",
            KnownSchemes::Xcon => "6501",
            KnownSchemes::XconUserID => "6501",
            KnownSchemes::XmlRpcBeep => "3529",
            KnownSchemes::XmlRpcBeeps => "3529",
            KnownSchemes::Xmpp => "4622,5122",
            KnownSchemes::Z3950r => "2056",
            KnownSchemes::Z3950s => "2056",
        }
        .to_string()
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
