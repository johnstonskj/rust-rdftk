use rdftk_iri::builder::IriBuilder;
use rdftk_iri::error::Result as IriResult;
use rdftk_iri::{Fragment, Host, Path, Query, Scheme, IRI};
use std::convert::TryInto;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_http_url_1() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme_str("https")
        .unwrap()
        .user_name("john.doe")
        .host_str("www.example.com")
        .unwrap()
        .port(123.into())
        .path_str("/forum/questions/")
        .unwrap()
        .query_str("tag=networking&order=newest")
        .unwrap()
        .fragment_str("top")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
            .to_string()
    );
}

#[test]
fn test_http_url_2() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::https())
        .user_name("john.doe")
        .host_str("www.example.com")
        .unwrap()
        .port(123.into())
        .path_str("/forum/questions/")
        .unwrap()
        .query_str("tag=networking&order=newest")
        .unwrap()
        .fragment_str("top")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
            .to_string()
    );
}

#[test]
fn test_http_url_3() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::https())
        .user_name("john.doe")
        .host(&Host::from_str("www.example.com").unwrap())
        .port(123.into())
        .path_str("/forum/questions/")
        .unwrap()
        .query_str("tag=networking&order=newest")
        .unwrap()
        .fragment_str("top")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
            .to_string()
    );
}

#[test]
fn test_http_url_4() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::https())
        .user_name("john.doe")
        .host_str("www.example.com")
        .unwrap()
        .port(123.into())
        .path(&Path::from_str("/forum/questions/").unwrap())
        .query_str("tag=networking&order=newest")
        .unwrap()
        .fragment_str("top")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
            .to_string()
    );
}

#[test]
fn test_http_url_5() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::https())
        .user_name("john.doe")
        .host_str("www.example.com")
        .unwrap()
        .port(123.into())
        .path_str("/forum/questions/")
        .unwrap()
        .query(&Query::from_str("tag=networking&order=newest").unwrap())
        .fragment_str("top")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
            .to_string()
    );
}

#[test]
fn test_http_url_6() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::https())
        .user_name("john.doe")
        .host_str("www.example.com")
        .unwrap()
        .port(123.into())
        .path_str("/forum/questions/")
        .unwrap()
        .query_str("tag=networking&order=newest")
        .unwrap()
        .fragment(&Fragment::from_str("top").unwrap())
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
            .to_string()
    );
}

#[test]
fn test_ldap_url() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::ldap())
        .host_str("[2001:db8::7]")
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
        .host_str("192.0.2.16")
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

#[test]
fn test_append_path_1() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .append_path_segment("foo")
        .unwrap()
        .append_path_segment("bar")
        .unwrap()
        .append_path_segment("baz")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(iri.to_string(), "foo/bar/baz".to_string());
}

#[test]
fn test_append_path_2() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .path_root()
        .append_path_segment("foo")
        .unwrap()
        .append_path_segment("bar")
        .unwrap()
        .append_path_segment("baz")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(iri.to_string(), "/foo/bar/baz".to_string());
}

#[test]
fn test_http_user_info_1() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::https())
        .user("john.doe", "passw0rd")
        .host_str("www.example.com")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe:passw0rd@www.example.com".to_string()
    );
}

#[test]
fn test_http_user_info_2() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::https())
        .user_name("john.doe")
        .password("passw0rd")
        .host_str("www.example.com")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe:passw0rd@www.example.com".to_string()
    );
}

#[test]
fn test_http_user_info_3() {
    let mut builder = IriBuilder::default();
    let result: IriResult<IRI> = builder
        .scheme(&Scheme::https())
        .user_name("john.doe")
        .host_str("www.example.com")
        .unwrap()
        .try_into();

    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(
        iri.to_string(),
        "https://john.doe@www.example.com".to_string()
    );
}
