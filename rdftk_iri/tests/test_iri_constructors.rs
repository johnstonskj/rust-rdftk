use rdftk_iri::{error::Result as IriResult, Fragment, Path, Query, IRI};
use std::convert::TryInto;
use std::path::PathBuf;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_default() {
    let blank_iri = IRI::default();

    assert!(!blank_iri.has_scheme());
    assert!(!blank_iri.has_authority());
    assert!(!blank_iri.has_path());
    assert!(!blank_iri.has_query());
    assert!(!blank_iri.has_fragment());
}

#[test]
fn test_from_absolute_path() {
    let path = Path::from_str("/Users/rust/readme").unwrap();
    let iri: IRI = path.into();

    assert!(!iri.has_scheme());
    assert!(!iri.has_authority());
    assert_eq!(iri.path().value(), "/Users/rust/readme");
    assert!(!iri.has_query());
    assert!(!iri.has_fragment());
}

#[test]
fn test_from_path() {
    let path = Path::from_str("Users/rust/readme").unwrap();
    let iri: IRI = path.into();

    assert!(!iri.has_scheme());
    assert!(!iri.has_authority());
    assert_eq!(iri.path().value(), "Users/rust/readme");
    assert!(!iri.has_query());
    assert!(!iri.has_fragment());
}

#[cfg(feature = "path_iri")]
#[test]
fn test_from_pathbuf() {
    let file_path = PathBuf::from("/Users/rust/readme");
    let result: IriResult<IRI> = file_path.try_into();
    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(iri.scheme().as_ref().unwrap().value(), "file");
    assert!(!iri.has_authority());
    assert_eq!(iri.path().value(), "/Users/rust/readme");
    assert!(!iri.has_query());
    assert!(!iri.has_fragment());
}

#[cfg(feature = "uuid_iri")]
#[test]
fn test_from_uuid() {
    let uuid = uuid::Uuid::new_v4();
    let result: IriResult<IRI> = uuid.try_into();
    assert!(result.is_ok());
    let iri = result.unwrap();

    assert_eq!(iri.scheme().as_ref().unwrap().value(), "urn");
    assert!(!iri.has_authority());
    assert_eq!(
        iri.path().value(),
        &format!("uuid:{}", uuid.to_hyphenated().to_string())
    );
    assert!(!iri.has_query());
    assert!(!iri.has_fragment());
}

#[test]
fn test_with_new_path() {
    let iri = IRI::from_str("http://joe.doe@example.com/foo/bar?query#fragment").unwrap();
    assert_eq!(iri.path().value(), "/foo/bar");

    let iri = iri.with_new_path(Path::from_str("Bar/Foo/").unwrap());
    assert_eq!(iri.path().value(), "Bar/Foo/");
}

#[test]
fn test_without_path() {
    let iri = IRI::from_str("http://joe.doe@example.com/foo/bar?query#fragment").unwrap();
    assert_eq!(iri.path().value(), "/foo/bar");

    let iri = iri.without_path();
    assert!(!iri.has_path());
}

#[test]
fn test_with_new_query() {
    let iri = IRI::from_str("http://joe.doe@example.com/foo/bar?query#fragment").unwrap();
    assert_eq!(iri.query().as_ref().unwrap().value(), "query");

    let iri = iri.with_new_query(Query::from_str("q=kew").unwrap());
    assert_eq!(iri.query().as_ref().unwrap().value(), "q=kew");
}

#[test]
fn test_without_query() {
    let iri = IRI::from_str("http://joe.doe@example.com/foo/bar?query#fragment").unwrap();
    assert_eq!(iri.query().as_ref().unwrap().value(), "query");

    let iri = iri.without_query();
    assert!(!iri.has_query());
}

#[test]
fn test_with_new_fragment() {
    let iri = IRI::from_str("http://joe.doe@example.com/foo/bar?query#fragment").unwrap();
    assert_eq!(iri.fragment().as_ref().unwrap().value(), "fragment");

    let iri = iri.with_new_fragment(Fragment::from_str("top").unwrap());
    assert_eq!(iri.fragment().as_ref().unwrap().value(), "top");
}

#[test]
fn test_without_fragment() {
    let iri = IRI::from_str("http://joe.doe@example.com/foo/bar?query#fragment").unwrap();
    assert_eq!(iri.fragment().as_ref().unwrap().value(), "fragment");

    let iri = iri.without_fragment();
    assert!(!iri.has_fragment());
}
