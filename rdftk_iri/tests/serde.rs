//!
//! Serialization/deserialization roundtrip tests for all types that carry
//! `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`.
//!
//! The entire file is compiled only when the `serde` feature is active.
//!

#![cfg(feature = "serde")]

use rdftk_iri::{Iri, IriPrefixMap, IriRef, LocalName, Name, Namespace, PrefixedName};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Helper
// ------------------------------------------------------------------------------------------------

fn roundtrip<T>(value: &T) -> T
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    let json = serde_json::to_string(value).expect("serialization failed");
    serde_json::from_str(&json).expect("deserialization failed")
}

// ------------------------------------------------------------------------------------------------
// Iri
// ------------------------------------------------------------------------------------------------

#[test]
fn iri_roundtrip() {
    let iri = Iri::from_str("https://example.org/ns#Thing").unwrap();
    assert_eq!(iri, roundtrip(&iri));
}

#[test]
fn iri_roundtrip_with_path_slash() {
    let iri = Iri::from_str("https://example.org/ns/").unwrap();
    assert_eq!(iri, roundtrip(&iri));
}

#[test]
fn iri_roundtrip_no_fragment() {
    let iri = Iri::from_str("https://example.org").unwrap();
    assert_eq!(iri, roundtrip(&iri));
}

#[test]
fn iri_serializes_as_string() {
    let iri = Iri::from_str("https://example.org/").unwrap();
    let json = serde_json::to_string(&iri).unwrap();
    assert_eq!(json, r#""https://example.org/""#);
}

// ------------------------------------------------------------------------------------------------
// Name
// ------------------------------------------------------------------------------------------------

#[test]
fn name_roundtrip() {
    let name = Name::from_str("subClassOf").unwrap();
    assert_eq!(name, roundtrip(&name));
}

#[test]
fn name_roundtrip_underscore_prefix() {
    let name = Name::from_str("_thing").unwrap();
    assert_eq!(name, roundtrip(&name));
}

#[test]
fn name_serializes_as_string() {
    let name = Name::from_str("type").unwrap();
    let json = serde_json::to_string(&name).unwrap();
    assert_eq!(json, r#""type""#);
}

// ------------------------------------------------------------------------------------------------
// Namespace
// ------------------------------------------------------------------------------------------------

#[test]
fn namespace_roundtrip_named() {
    let ns = Namespace::from_str("rdf:").unwrap();
    assert_eq!(ns, roundtrip(&ns));
}

#[test]
fn namespace_roundtrip_default() {
    let ns = Namespace::new_default();
    assert_eq!(ns, roundtrip(&ns));
}

#[test]
fn namespace_serializes_as_string() {
    let ns = Namespace::from_str("owl:").unwrap();
    let json = serde_json::to_string(&ns).unwrap();
    assert_eq!(json, r#""owl:""#);
}

// ------------------------------------------------------------------------------------------------
// LocalName
// ------------------------------------------------------------------------------------------------

#[test]
fn local_name_roundtrip_prefixed() {
    let local = LocalName::from_str("rdf:type").unwrap();
    assert_eq!(local, roundtrip(&local));
}

#[test]
fn local_name_roundtrip_default_namespace() {
    let local = LocalName::new_in_default(Name::from_str("Thing").unwrap());
    assert_eq!(local, roundtrip(&local));
}

#[test]
fn local_name_roundtrip_constructed() {
    let local = LocalName::new(
        Namespace::from_str("rdfs:").unwrap(),
        Name::from_str("subClassOf").unwrap(),
    );
    assert_eq!(local, roundtrip(&local));
}

// ------------------------------------------------------------------------------------------------
// PrefixedName
// ------------------------------------------------------------------------------------------------

#[test]
fn prefixed_name_namespace_variant_roundtrip() {
    let pname = PrefixedName::Namespace(Namespace::from_str("owl:").unwrap());
    assert_eq!(pname, roundtrip(&pname));
}

#[test]
fn prefixed_name_local_variant_roundtrip() {
    let pname = PrefixedName::Local(LocalName::from_str("rdfs:subClassOf").unwrap());
    assert_eq!(pname, roundtrip(&pname));
}

#[test]
fn prefixed_name_local_default_ns_roundtrip() {
    let pname = PrefixedName::Local(LocalName::new_in_default(Name::from_str("Thing").unwrap()));
    assert_eq!(pname, roundtrip(&pname));
}

// ------------------------------------------------------------------------------------------------
// IriRef
// ------------------------------------------------------------------------------------------------

#[test]
fn iri_ref_iri_variant_roundtrip() {
    let iri_ref = IriRef::Iri(Iri::from_str("https://example.org/").unwrap());
    assert_eq!(iri_ref, roundtrip(&iri_ref));
}

#[test]
fn iri_ref_prefixed_name_namespace_variant_roundtrip() {
    let iri_ref = IriRef::PrefixedName(PrefixedName::Namespace(
        Namespace::from_str("rdf:").unwrap(),
    ));
    assert_eq!(iri_ref, roundtrip(&iri_ref));
}

#[test]
fn iri_ref_prefixed_name_local_variant_roundtrip() {
    let iri_ref = IriRef::PrefixedName(PrefixedName::Local(
        LocalName::from_str("rdf:type").unwrap(),
    ));
    assert_eq!(iri_ref, roundtrip(&iri_ref));
}

// ------------------------------------------------------------------------------------------------
// IriPrefixMap
// ------------------------------------------------------------------------------------------------

#[test]
fn iri_prefix_map_empty_roundtrip() {
    let map = IriPrefixMap::default();
    assert_eq!(map, roundtrip(&map));
}

#[test]
fn iri_prefix_map_common_roundtrip() {
    let map = IriPrefixMap::common();
    assert_eq!(map, roundtrip(&map));
}

#[test]
fn iri_prefix_map_with_default_ns_roundtrip() {
    let map =
        IriPrefixMap::default().with_default(Iri::from_str("https://example.org/ns#").unwrap());
    assert_eq!(map, roundtrip(&map));
}

#[test]
fn iri_prefix_map_custom_roundtrip() {
    let map = IriPrefixMap::default()
        .with(
            Namespace::from_str("ex:").unwrap(),
            Iri::from_str("https://example.org/ns#").unwrap(),
        )
        .with(
            Namespace::from_str("rdf:").unwrap(),
            Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap(),
        );
    assert_eq!(map, roundtrip(&map));
}
