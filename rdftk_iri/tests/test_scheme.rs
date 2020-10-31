use proptest::prelude::*;
use rdftk_iri::{Normalize, Scheme, ValidateStr};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_parse_examples() {
    // simple string
    assert!(Scheme::from_str("aaa").is_ok());

    // string with numbers
    assert!(Scheme::from_str("tn3270").is_ok());

    // string with dots
    assert!(Scheme::from_str("iris.beep").is_ok());

    // string with hyphens
    assert!(Scheme::from_str("xcon-userid").is_ok());
}

#[test]
fn test_display_examples() {
    // simple string
    assert_eq!(
        Scheme::from_str("aaa").unwrap().to_string(),
        "aaa:".to_string()
    );

    // string with numbers
    assert_eq!(
        Scheme::from_str("tn3270").unwrap().to_string(),
        "tn3270:".to_string()
    );

    // string with dots
    assert_eq!(
        Scheme::from_str("iris.beep").unwrap().to_string(),
        "iris.beep:".to_string()
    );

    // string with hyphens
    assert_eq!(
        Scheme::from_str("xcon-userid").unwrap().to_string(),
        "xcon-userid:".to_string()
    );
}

#[test]
fn test_parse_illegal() {
    assert_eq!(Scheme::is_valid(""), false, "should not be empty");
    assert!(Scheme::from_str("").is_err());

    assert_eq!(Scheme::is_valid(" "), false, "should not accept spaces");
    assert!(Scheme::from_str("").is_err());

    assert_eq!(
        Scheme::is_valid(" aaa"),
        false,
        "should not leading accept spaces"
    );
    assert!(Scheme::from_str("").is_err());

    assert_eq!(
        Scheme::is_valid("aa a"),
        false,
        "should not embedded accept spaces"
    );
    assert!(Scheme::from_str("").is_err());

    assert_eq!(
        Scheme::is_valid("123"),
        false,
        "should not accept all numbers"
    );
    assert!(Scheme::from_str("123").is_err());

    assert_eq!(
        Scheme::is_valid("123ab"),
        false,
        "should not accept leading numbers"
    );
    assert!(Scheme::from_str("123ab").is_err());

    assert_eq!(Scheme::is_valid("a!"), false, "should not accept !");
    assert!(Scheme::from_str("a!").is_err());
}

#[test]
fn test_normalize_examples() {
    // simple string
    assert_eq!(
        Scheme::from_str("aaa").unwrap().normalize().unwrap(),
        Scheme::from_str("aaa").unwrap()
    );
    assert_eq!(
        Scheme::from_str("AAA").unwrap().normalize().unwrap(),
        Scheme::from_str("aaa").unwrap()
    );
    assert_eq!(
        Scheme::from_str("aAa").unwrap().normalize().unwrap(),
        Scheme::from_str("aaa").unwrap()
    );
}

#[test]
fn test_equality() {
    // simple string
    assert_eq!(
        Scheme::from_str("aaa").unwrap(),
        Scheme::from_str("AAA").unwrap()
    );
}

#[test]
fn test_known_schemes() {
    #[allow(clippy::type_complexity)]
    let tests: Vec<(fn() -> rdftk_iri::Scheme, &str)> = vec![
        (Scheme::aaa, "aaa"),
        (Scheme::aaas, "aaas"),
        (Scheme::about, "about"),
        (Scheme::acap, "acap"),
        (Scheme::acct, "acct"),
        (Scheme::blob, "blob"),
        (Scheme::cap, "cap"),
        (Scheme::cid, "cid"),
        (Scheme::coap, "coap"),
        (Scheme::coaps, "coaps"),
        (Scheme::crid, "crid"),
        (Scheme::data, "data"),
        (Scheme::dav, "dav"),
        (Scheme::dict, "dict"),
        (Scheme::dns, "dns"),
        (Scheme::example, "example"),
        (Scheme::file, "file"),
        (Scheme::ftp, "ftp"),
        (Scheme::geo, "geo"),
        (Scheme::go, "go"),
        (Scheme::gopher, "gopher"),
        (Scheme::http, "http"),
        (Scheme::https, "https"),
        (Scheme::iax, "iax"),
        (Scheme::icap, "icap"),
        (Scheme::im, "im"),
        (Scheme::imap, "imap"),
        (Scheme::info, "info"),
        (Scheme::ipp, "ipp"),
        (Scheme::ipps, "ipps"),
        (Scheme::iris, "iris"),
        (Scheme::iris_beep, "iris.beep"),
        (Scheme::iris_xpc, "iris.xpc"),
        (Scheme::iris_xpcs, "iris.xpcs"),
        (Scheme::iris_lws, "iris.lws"),
        (Scheme::ldap, "ldap"),
        (Scheme::mailto, "mailto"),
        (Scheme::mid, "mid"),
        (Scheme::msrp, "msrp"),
        (Scheme::msrps, "msrps"),
        (Scheme::mtqp, "mtqp"),
        (Scheme::mupdate, "mupdate"),
        (Scheme::news, "news"),
        (Scheme::nfs, "nfs"),
        (Scheme::ni, "ni"),
        (Scheme::nih, "nih"),
        (Scheme::nntp, "nntp"),
        (Scheme::opaque_lock_token, "opaquelocktoken"),
        (Scheme::pkcs11, "pkcs11"),
        (Scheme::pop, "pop"),
        (Scheme::pres, "pres"),
        (Scheme::reload, "reload"),
        (Scheme::rtsp, "rtsp"),
        (Scheme::service, "service"),
        (Scheme::session, "session"),
        (Scheme::shttp, "shttp"),
        (Scheme::sieve, "sieve"),
        (Scheme::sip, "sip"),
        (Scheme::sips, "sips"),
        (Scheme::sms, "sms"),
        (Scheme::snmp, "snmp"),
        (Scheme::soap_beep, "soap.beep"),
        (Scheme::soap_beeps, "soap.beeps"),
        (Scheme::stun, "stun"),
        (Scheme::stuns, "stuns"),
        (Scheme::tag, "tag"),
        (Scheme::tel, "tel"),
        (Scheme::telnet, "telnet"),
        (Scheme::tftp, "tftp"),
        (Scheme::this_message, "thismessage"),
        (Scheme::tn3270, "tn3270"),
        (Scheme::tip, "tip"),
        (Scheme::turn, "turn"),
        (Scheme::turns, "turns"),
        (Scheme::tv, "tv"),
        (Scheme::urn, "urn"),
        (Scheme::vemmi, "vemmi"),
        (Scheme::ws, "ws"),
        (Scheme::wss, "wss"),
        (Scheme::xcon, "xcon"),
        (Scheme::xcon_user_id, "xcon-userid"),
        (Scheme::xml_rpc_beep, "xmlrpc.beep"),
        (Scheme::xml_rpc_beeps, "xmlrpc.beeps"),
        (Scheme::xmpp, "xmpp"),
        (Scheme::z3950r, "z39.50r"),
        (Scheme::z3950s, "z39.50s"),
    ];
    for (fun, value) in tests {
        let scheme = fun();
        assert_eq!(scheme.value(), value);
    }
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        let _ = Scheme::from_str(&s);
    }

    #[test]
    fn valid_values(s in "[[:alpha:]][[[:alnum:]]+-\\.]*") {
        println!("valid_values {:?}", s);
        assert!(Scheme::is_valid(&s));
        assert!(Scheme::from_str(&s).is_ok());
    }
}

// ------------------------------------------------------------------------------------------------
// Regression Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_unicode_gibberish() {
    assert!(Scheme::from_str("º").is_err());
}
