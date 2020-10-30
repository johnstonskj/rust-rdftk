use rdftk_iri::{Fragment, Host, Path, Scheme, IRI};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_parse_iri_simple_url() {
    let result = IRI::from_str(
        "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top",
    );
    assert!(result.is_ok());
    println!("{:#?}", result);
    let result = result.unwrap();
    assert_eq!(result.scheme(), &Some(Scheme::https()));
    let authority = result.authority().as_ref().unwrap();
    assert_eq!(
        authority.host(),
        &Host::from_str("www.example.com").unwrap()
    );
    let user_info = authority.user_info().as_ref().unwrap();
    assert_eq!(user_info.user_name(), &"john.doe".to_string());
    assert_eq!(user_info.password(), &None);
    assert_eq!(authority.port(), &Some(123.into()));
    assert_eq!(result.path(), &Path::from_str("/forum/questions/").unwrap());
    assert_eq!(result.fragment(), &Some(Fragment::from_str("top").unwrap()));
}

#[test]
fn test_parse_ldap_iri() {
    let result = IRI::from_str("ldap://[2001:db8::7]/c=GB?objectClass?one");
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:#?}", result);
}

#[test]
fn test_parse_mailto_iri() {
    let result = IRI::from_str("mailto:John.Doe@example.com");
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:#?}", result);
    assert_eq!(result.scheme(), &Some(Scheme::mailto()));
    assert_eq!(result.authority(), &None);
    assert_eq!(
        result.path(),
        &Path::from_str("John.Doe@example.com").unwrap()
    );
    assert_eq!(result.fragment(), &None);
}

#[test]
fn test_parse_usenet_iri() {
    let result = IRI::from_str("news:comp.infosystems.www.servers.unix");
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:#?}", result);
    assert_eq!(result.scheme(), &Some(Scheme::news()));
    assert_eq!(result.authority(), &None);
    assert_eq!(
        result.path(),
        &Path::from_str("comp.infosystems.www.servers.unix").unwrap()
    );
    assert_eq!(result.fragment(), &None);
}

#[test]
fn test_parse_tel_iri() {
    let result = IRI::from_str("tel:+1-816-555-1212");
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:#?}", result);
    assert_eq!(result.scheme(), &Some(Scheme::tel()));
    assert_eq!(result.authority(), &None);
    assert_eq!(result.path(), &Path::from_str("+1-816-555-1212").unwrap());
    assert_eq!(result.fragment(), &None);
}

#[test]
fn test_parse_telnet_iri() {
    let result = IRI::from_str("telnet://192.0.2.16:80/");
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:#?}", result);
    assert_eq!(result.scheme(), &Some(Scheme::telnet()));
    let authority = result.authority().as_ref().unwrap();
    assert_eq!(authority.host(), &Host::from_str("192.0.2.16").unwrap());
    assert_eq!(authority.user_info(), &None);
    assert_eq!(authority.port(), &Some(80.into()));
    assert_eq!(result.path(), &Path::from_str("/").unwrap());
    assert_eq!(result.fragment(), &None);
}

#[test]
fn test_parse_urn_iri() {
    let result = IRI::from_str("urn:oasis:names:specification:docbook:dtd:xml:4.1.2");
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:#?}", result);
    assert_eq!(result.scheme(), &Some(Scheme::urn()));
    assert_eq!(result.authority(), &None);
    assert_eq!(
        result.path(),
        &Path::from_str("oasis:names:specification:docbook:dtd:xml:4.1.2").unwrap()
    );
    assert_eq!(result.fragment(), &None);
}

#[test]
fn test_parse_iri_i18n_path() {
    let result = IRI::from_str("https://en.wiktionary.org/wiki/Ῥόδος");
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:#?}", result);
    assert_eq!(result.scheme(), &Some(Scheme::https()));
    let authority = result.authority().as_ref().unwrap();
    assert_eq!(
        authority.host(),
        &Host::from_str("en.wiktionary.org").unwrap()
    );
    assert_eq!(authority.user_info(), &None);
    assert_eq!(authority.port(), &None);
    assert_eq!(result.path(), &Path::from_str("/wiki/Ῥόδος").unwrap());
    assert_eq!(result.fragment(), &None);
}

#[test]
fn test_parse_iri_i18n_host() {
    let result = IRI::from_str("http://www.myfictionαlbank.com/");
    assert!(result.is_ok());
    let result = result.unwrap();
    println!("{:#?}", result);
    assert_eq!(result.scheme(), &Some(Scheme::http()));
    let authority = result.authority().as_ref().unwrap();
    assert_eq!(
        authority.host(),
        &Host::from_str("www.myfictionαlbank.com").unwrap()
    );
    assert_eq!(authority.user_info(), &None);
    assert_eq!(authority.port(), &None);
    assert_eq!(result.path(), &Path::from_str("/").unwrap());
    assert_eq!(result.fragment(), &None);
}
