#![allow(clippy::module_name_repetitions)]

use crate::error::{Error as IriError, ErrorKind, Result as IriResult};
use crate::Normalize;
use crate::{parse, ValidateStr};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Provides the `Scheme` component of an `IRI` as well as a set of known schemes.
///
/// The scheme for an IRI (URI, or URL) determines the syntax and meaning of the following
/// components. It is a single string value.
///  
/// # Example
///
/// Any valid scheme string can be parsed into a `Scheme` instance.
///
/// ```rust
/// use rdftk_iri::Scheme;
/// use std::str::FromStr;
///
/// let http_scheme = Scheme::from_str("http");
/// ```
///
/// A number of well-known schemes are also provided as associated functions on the `Scheme`
/// implementation for convenience.
///
/// ```rust
/// use rdftk_iri::Scheme;
///
/// let http_scheme = Scheme::http();
/// ```
///
#[derive(Clone, Debug, Eq)]
pub struct Scheme(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl PartialEq for Scheme {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Hash for Scheme {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state);
    }
}

impl Display for Scheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.0)
    }
}

impl FromStr for Scheme {
    type Err = IriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(ErrorKind::ParseSchemeError(s.to_string()).into())
        }
    }
}

impl ValidateStr for Scheme {
    fn is_valid(s: &str) -> bool {
        parse::is_scheme(s)
    }
}

impl Normalize for Scheme {
    fn normalize(self) -> IriResult<Self> {
        Ok(Self(self.0.to_lowercase()))
    }
}

impl Scheme {
    // The following schemes are taken from https://en.wikipedia.org/wiki/List_of_URI_schemes
    // where status = 'permanent'
    //   and defined_by = known org

    /// Known Scheme for Diameter Protocol, RFC 3588,6733
    pub fn aaa() -> Self {
        "aaa".parse().unwrap()
    }

    /// Known Scheme for Diameter Protocol, RFC 3588,6733
    pub fn aaas() -> Self {
        "aaas".parse().unwrap()
    }

    /// Known Scheme for product information, RFC 6694
    pub fn about() -> Self {
        "about".parse().unwrap()
    }

    /// Known Scheme for Application Configuration Access Protocol, RFC 2244
    pub fn acap() -> Self {
        "acap".parse().unwrap()
    }

    /// Known Scheme for identifying user account, RFC 7565
    pub fn acct() -> Self {
        "acct".parse().unwrap()
    }

    /// Known Scheme for binary data access in browsers (http://www.w3.org/TR/FileAPI/#url)
    pub fn blob() -> Self {
        "blob".parse().unwrap()
    }

    /// Known Scheme for Calendar Access Protocol, RFC 4324
    pub fn cap() -> Self {
        "cap".parse().unwrap()
    }

    /// Known Scheme for referencing individual parts of an SMTP/MIME message, RFC 2111,2392
    pub fn cid() -> Self {
        "cid".parse().unwrap()
    }

    /// Known Scheme for Constrained Application Protocol, RFC 7252
    pub fn coap() -> Self {
        "coap".parse().unwrap()
    }

    /// Known Scheme for Constrained Application Protocol, RFC 7252
    pub fn coaps() -> Self {
        "coaps".parse().unwrap()
    }

    /// Known Scheme for TV-Anytime Content Reference Identifier, RFC 4078
    pub fn crid() -> Self {
        "crid".parse().unwrap()
    }

    /// Known Scheme for inclusion of small data items inline, RFC 2397
    pub fn data() -> Self {
        "data".parse().unwrap()
    }

    /// Known Scheme for HTTP Extensions for Distributed Authoring (WebDAV), RFC 2518,4918
    pub fn dav() -> Self {
        "dav".parse().unwrap()
    }

    /// Known Scheme for Dictionary Service Protocol, RFC 2229
    pub fn dict() -> Self {
        "dict".parse().unwrap()
    }

    /// Known Scheme for Domain Name System, RFC 4501
    pub fn dns() -> Self {
        "dns".parse().unwrap()
    }

    /// Known Scheme for examples, RFC 7595
    pub fn example() -> Self {
        "example".parse().unwrap()
    }

    /// Known Scheme for addressing files on local or network file systems, RFC 1738,3986
    pub fn file() -> Self {
        "file".parse().unwrap()
    }

    /// Known Scheme for File Transfer Protocol, RFC 1738
    pub fn ftp() -> Self {
        "ftp".parse().unwrap()
    }

    /// Known Scheme for Geographic Locations, RFC 5870"
    pub fn geo() -> Self {
        "geo".parse().unwrap()
    }

    /// Known Scheme for Common Name Resolution Protocol, RFC 3368
    pub fn go() -> Self {
        "go".parse().unwrap()
    }

    /// Known Scheme for Gopher Protocol, RFC 4266
    pub fn gopher() -> Self {
        "gopher".parse().unwrap()
    }

    /// Known Scheme for HTTP resources, RFC 2817,7230
    pub fn http() -> Self {
        "http".parse().unwrap()
    }

    /// Known Scheme for HTTP resources secured using SSL/TLS, RFC 2817,7230
    pub fn https() -> Self {
        "https".parse().unwrap()
    }

    /// Known Scheme for Inter-Asterisk eXchange protocol, RFC 5456
    pub fn iax() -> Self {
        "iax".parse().unwrap()
    }

    /// Known Scheme for Internet Content Adaptation Protocol, RFC 3507
    pub fn icap() -> Self {
        "icap".parse().unwrap()
    }

    /// Known Scheme for Instant Messaging Protocol, RFC 3860
    pub fn im() -> Self {
        "im".parse().unwrap()
    }

    /// Known Scheme for accessing e-mail resources through IMAP, RFC 2192,5092
    pub fn imap() -> Self {
        "imap".parse().unwrap()
    }

    /// Known Scheme for Information Assets with Identifiers in Public Namespaces, RFC 4452
    pub fn info() -> Self {
        "info".parse().unwrap()
    }

    /// Known Scheme for Internet Printing Protocol, RFC 3510
    pub fn ipp() -> Self {
        "ipp".parse().unwrap()
    }

    /// Known Scheme for Internet Printing Protocol over HTTPS, RFC 7472
    pub fn ipps() -> Self {
        "ipps".parse().unwrap()
    }

    /// Known Scheme for Internet Registry Information Service, RFC 3981
    pub fn iris() -> Self {
        "iris".parse().unwrap()
    }

    /// Known Scheme for Internet Registry Information Service, RFC 3983
    pub fn iris_beep() -> Self {
        "iris.beep".parse().unwrap()
    }

    /// Known Scheme for Internet Registry Information Service, RFC 4992
    pub fn iris_xpc() -> Self {
        "iris.xpc".parse().unwrap()
    }

    /// Known Scheme for Internet Registry Information Service, RFC 4992
    pub fn iris_xpcs() -> Self {
        "iris.xpcs".parse().unwrap()
    }

    /// Known Scheme for Internet Registry Information Service, RFC 4993
    pub fn iris_lws() -> Self {
        "iris.lws".parse().unwrap()
    }

    /// Known Scheme for LDAP directory request, RFC 2255,4516
    pub fn ldap() -> Self {
        "ldap".parse().unwrap()
    }

    /// Known Scheme for SMTP e-mail addresses and default content, RFC 6068
    pub fn mailto() -> Self {
        "mailto".parse().unwrap()
    }

    /// Known Scheme for referencing SMTP/MIME messages, or parts of messages, RFC 2111,2392
    pub fn mid() -> Self {
        "mid".parse().unwrap()
    }

    /// Known Scheme for Message Session Relay Protocol, RFC 4975
    pub fn msrp() -> Self {
        "msrp".parse().unwrap()
    }

    /// Known Scheme for Message Session Relay Protocol, RFC 4975
    pub fn msrps() -> Self {
        "msrps".parse().unwrap()
    }

    /// Known Scheme for Message Tracking Query Protocol, RFC 3887
    pub fn mtqp() -> Self {
        "mtqp".parse().unwrap()
    }

    /// Known Scheme for Mailbox Update Protocol, RFC 3656
    pub fn mupdate() -> Self {
        "mupdate".parse().unwrap()
    }

    /// Known Scheme for (Usenet) newsgroups and postings, RFC 1738,5538
    pub fn news() -> Self {
        "news".parse().unwrap()
    }

    /// Known Scheme for Network File System resources, RFC 2224
    pub fn nfs() -> Self {
        "nfs".parse().unwrap()
    }

    /// Known Scheme for Named Information, RFC 6920
    pub fn ni() -> Self {
        "ni".parse().unwrap()
    }

    /// Known Scheme for Named Information for Humans, RFC 6920
    pub fn nih() -> Self {
        "nih".parse().unwrap()
    }

    /// Known Scheme for Usenet NNTP, RFC 1738,5538
    pub fn nntp() -> Self {
        "nntp".parse().unwrap()
    }

    /// Known Scheme for WebDAV lock token, RFC 2518,4918
    pub fn opaque_lock_token() -> Self {
        "opaquelocktoken".parse().unwrap()
    }

    /// Known Scheme for PKCS #11, RFC 7512
    pub fn pkcs11() -> Self {
        "pkcs11".parse().unwrap()
    }

    /// Known Scheme for Accessing mailbox through POP3, RFC 2358
    pub fn pop() -> Self {
        "pop".parse().unwrap()
    }

    /// Known Scheme for Used in Common Profile for Presence (CPP) to identify presence, RFC 3859
    pub fn pres() -> Self {
        "pres".parse().unwrap()
    }

    /// Known Scheme for REsource LOcation And Discovery Protocol, RFC 6940
    pub fn reload() -> Self {
        "reload".parse().unwrap()
    }

    /// Known Scheme for Real Time Streaming Protocol, RFC 2326
    pub fn rtsp() -> Self {
        "rtsp".parse().unwrap()
    }

    /// Known Scheme for Used in Service Location Protocol, RFC 2609
    pub fn service() -> Self {
        "service".parse().unwrap()
    }

    /// Known Scheme for Media Resource Control Protocol, RFC 4463,6787
    pub fn session() -> Self {
        "session".parse().unwrap()
    }

    /// Known Scheme for Secure HTTP, RFC 2660
    pub fn shttp() -> Self {
        "shttp".parse().unwrap()
    }

    /// Known Scheme for ManageSieve protocol, RFC 5804
    pub fn sieve() -> Self {
        "sieve".parse().unwrap()
    }

    /// Known Scheme for Used with Session Initiation Protocol (SIP), RFC 2543,3969,3261
    pub fn sip() -> Self {
        "sip".parse().unwrap()
    }

    /// Known Scheme for Secure equivalent of SIP, RFC 3969,3261
    pub fn sips() -> Self {
        "sips".parse().unwrap()
    }

    /// Known Scheme for Interact with SMS capable devices for messaging, RFC 5724
    pub fn sms() -> Self {
        "sms".parse().unwrap()
    }

    /// Known Scheme for Simple Network Management Protocol, RFC 4088
    pub fn snmp() -> Self {
        "snmp".parse().unwrap()
    }

    /// Known Scheme for SOAP binding to BEEP, RFC 3288,4227
    pub fn soap_beep() -> Self {
        "soap.beep".parse().unwrap()
    }

    /// Known Scheme for SOAP binding to BEEP, RFC 3288,4227
    pub fn soap_beeps() -> Self {
        "soap.beeps".parse().unwrap()
    }

    /// Known Scheme for ion Traversal Utilities for NAT (STUN), RFC 7064
    pub fn stun() -> Self {
        "stun".parse().unwrap()
    }

    /// Known Scheme for ion Traversal Utilities for NAT (STUN), RFC 7064
    pub fn stuns() -> Self {
        "stuns".parse().unwrap()
    }

    /// Known Scheme for The Tag URI, RFC 4151
    pub fn tag() -> Self {
        "tag".parse().unwrap()
    }

    /// Known Scheme for Used for telephone numbers, RFC 5341,3966,2806
    pub fn tel() -> Self {
        "tel".parse().unwrap()
    }

    /// Known Scheme for Used with telnet, RFC 1738,4248
    pub fn telnet() -> Self {
        "telnet".parse().unwrap()
    }

    /// Known Scheme for Trivial File Transfer Protocol, RFC 3627
    pub fn tftp() -> Self {
        "tftp".parse().unwrap()
    }

    /// Known Scheme for multipart/related relative reference resolution, RFC 2557
    pub fn this_message() -> Self {
        "thismessage".parse().unwrap()
    }

    /// Known Scheme for Interactive 3270 emulation sessions, RFC 6270
    pub fn tn3270() -> Self {
        "tn3270".parse().unwrap()
    }

    /// Known Scheme for Transaction Internet Protocol, RFC 2371
    pub fn tip() -> Self {
        "tip".parse().unwrap()
    }

    /// Known Scheme for Traversal Using Relays around NAT (TURN), RFC 7065
    pub fn turn() -> Self {
        "turn".parse().unwrap()
    }

    /// Known Scheme for Traversal Using Relays around NAT (TURN), RFC 7065
    pub fn turns() -> Self {
        "turns".parse().unwrap()
    }

    /// Known Scheme for TV Broadcasts, RFC 2838
    pub fn tv() -> Self {
        "tv".parse().unwrap()
    }

    /// Known Scheme for Uniform Resource Names, RFC 2141
    pub fn urn() -> Self {
        "urn".parse().unwrap()
    }

    /// Known Scheme for Versatile Multimedia Interface, RFC 2122
    pub fn vemmi() -> Self {
        "vemmi".parse().unwrap()
    }

    /// Known Scheme for Virtual Network Computing, RFC 7869
    pub fn vnc() -> Self {
        "vnc".parse().unwrap()
    }

    /// Known Scheme for WebSocket protocol, RFC 6455
    pub fn ws() -> Self {
        "ws".parse().unwrap()
    }

    /// Known Scheme for WebSocket protocol, RFC 6455
    pub fn wss() -> Self {
        "wss".parse().unwrap()
    }

    /// Known Scheme for Centralized Conferencing (XCON) over SIP, RFC 6501
    pub fn xcon() -> Self {
        "xcon".parse().unwrap()
    }

    /// Known Scheme for Centralized Conferencing (XCON) over SIP, RFC 6501
    pub fn xcon_user_id() -> Self {
        "xcon-userid".parse().unwrap()
    }

    /// Known Scheme for XML-RPC in BEEP, RFC 3529
    pub fn xml_rpc_beep() -> Self {
        "xmlrpc.beep".parse().unwrap()
    }

    /// Known Scheme for XML-RPC in BEEP, RFC 3529
    pub fn xml_rpc_beeps() -> Self {
        "xmlrpc.beeps".parse().unwrap()
    }

    /// Known Scheme for Extensible Messaging and Presence Protocol (XMPP), RFC 4622,5122
    pub fn xmpp() -> Self {
        "xmpp".parse().unwrap()
    }

    /// Known Scheme for Z39.50 Retrieval, RFC 2056
    pub fn z3950r() -> Self {
        "z39.50r".parse().unwrap()
    }

    /// Known Scheme for Z39.50 Session, RFC 2056
    pub fn z3950s() -> Self {
        "z39.50s".parse().unwrap()
    }

    /// Return the string value of this scheme.
    pub fn value(&self) -> &String {
        &self.0
    }
}
