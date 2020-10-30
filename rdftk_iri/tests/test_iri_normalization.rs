pub mod common;
pub use common::*;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_case_conversion() {
    normalize_and_compare("hTTp://google.com", "http://google.com:80/");
    normalize_and_compare("http://GoOgLe.CoM", "http://google.com:80/");
    normalize_and_compare("http://ΓΟΟΓΛΕ.CoM", "http://γοογλε.com:80/");
}

#[test]
fn test_port_addition() {
    normalize_and_compare("ftp://example.com", "ftp://example.com:20/");
    normalize_and_compare("ssh://127.0.0.1", "ssh://127.0.0.1:22");
    normalize_and_compare("telnet://127.0.0.1", "telnet://127.0.0.1:23");
    normalize_and_compare("tftp://example.com", "tftp://example.com:69/");
    normalize_and_compare("gopher://example.com", "gopher://example.com:70");
    normalize_and_compare("http://example.com", "http://example.com:80/");
    normalize_and_compare("nntp://example.com", "nntp://example.com:119");
    normalize_and_compare("imap://example.com", "imap://example.com:143");
    normalize_and_compare("snmp://example.com", "snmp://example.com:161");
    normalize_and_compare("ldap://example.com", "ldap://example.com:389");
    normalize_and_compare("https://example.com", "https://example.com:443/");
    normalize_and_compare("rtsp://example.com", "rtsp://example.com:554");
    normalize_and_compare("ipp://example.com", "ipp://example.com:631");
    normalize_and_compare("iris.beep://example.com", "iris.beep://example.com:702");
    normalize_and_compare("dict://example.com", "dict://example.com:2628");
    normalize_and_compare("stun://example.com", "stun://example.com:3478");
    normalize_and_compare("aaa://example.com", "aaa://example.com:3868");
    normalize_and_compare("iax://example.com", "iax://example.com:4569");
    normalize_and_compare("sip://example.com", "sip://example.com:5060");
    normalize_and_compare("sips://example.com", "sips://example.com:5061");
    normalize_and_compare("vnc://example.com", "vnc://example.com:5500");
    normalize_and_compare("coap://example.com", "coap://example.com:5683");
    normalize_and_compare("coaps://example.com", "coaps://example.com:5684");
}

#[test]
fn test_no_port_additions() {
    normalize_and_compare("mailto:john.doe@example.com", "mailto:john.doe@example.com");
    normalize_and_compare(
        "spotify:track:2jCnn1QPQ3E8ExtLe6INsx",
        "spotify:track:2jCnn1QPQ3E8ExtLe6INsx",
    );
    normalize_and_compare("tel:555-555-5555", "tel:555-555-5555");
}

#[test]
fn test_no_port_changes() {
    normalize_and_compare("http://example.com:8080", "http://example.com:8080/");
    normalize_and_compare("https://example.com:4433", "https://example.com:4433/");
}

#[test]
fn test_path() {
    normalize_and_compare("http://example.com", "http://example.com:80/");
    normalize_and_compare("http://example.com/.", "http://example.com:80/");
    normalize_and_compare("http://example.com/..", "http://example.com:80/");
}
