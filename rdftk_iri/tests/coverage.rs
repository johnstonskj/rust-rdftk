//!
//! Tests targeting the trivial-accessor and error-branch lines that the
//! existing test set never touches. Their primary purpose is to lift the
//! crate above the 80% line-coverage threshold; they are still useful
//! regression guards.
//!

use rdftk_iri::{
    Error, Iri, IriPrefixMap, IriRef, LocalName, Name, Namespace, PrefixedName,
    error::NameParseError,
    vocab::{
        VOCABULARY_CAL, VOCABULARY_DBPEDIA, VOCABULARY_DC_ELEMENTS, VOCABULARY_DC_TERMS,
        VOCABULARY_DOAP, VOCABULARY_FOAF, VOCABULARY_GEO_NAMES, VOCABULARY_ISO_SKOS,
        VOCABULARY_OPEN_GIS, VOCABULARY_ORG, VOCABULARY_OWL, VOCABULARY_RDF, VOCABULARY_RDF_SCHEMA,
        VOCABULARY_SIOC, VOCABULARY_SKOS, VOCABULARY_SKOS_XL, VOCABULARY_VANN, VOCABULARY_VOID,
        VOCABULARY_WOT, VOCABULARY_XML, VOCABULARY_XML_SCHEMA, Vocabulary,
    },
};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Namespace accessors and error branches
// ------------------------------------------------------------------------------------------------

#[test]
fn namespace_helpers() {
    assert!(Namespace::is_valid_str("rdf:"));
    assert!(!Namespace::is_valid_str("rdf:foo"));

    let default = Namespace::new_default();
    assert!(default.is_default());
    assert_eq!(default.name_string(), None);

    let named = Namespace::new_named("rdf").unwrap();
    assert_eq!(named.to_string(), "rdf:");
    assert_eq!(named.name_string(), Some("rdf"));
    assert!(!named.is_default());

    // String/AsRef conversions.
    assert_eq!(named.as_ref(), "rdf:");
    assert_eq!(String::from(named.clone()), "rdf:");
    assert_eq!(String::from(&named), "rdf:");
}

#[test]
fn namespace_from_str_error_branches() {
    assert!(matches!(
        Namespace::from_str(""),
        Err(NameParseError::EmptyString)
    ));
    assert!(matches!(
        Namespace::from_str("rdf"),
        Err(NameParseError::MissingSeparator(_))
    ));
    assert!(matches!(
        Namespace::from_str("rdf:foo:"),
        Err(NameParseError::TooManySeparators(_))
    ));
    assert!(matches!(
        Namespace::from_str("0bad:"),
        Err(NameParseError::InvalidCharacter(_))
    ));
}

#[test]
fn namespace_new_unchecked_appends_colon() {
    let ns = Namespace::new_unchecked("foo");
    assert_eq!(ns.to_string(), "foo:");
    let ns = Namespace::new_unchecked("foo:");
    assert_eq!(ns.to_string(), "foo:");
}

#[test]
fn namespace_qualify_round_trip() {
    let ns = Namespace::from_str("rdf:").unwrap();
    let local = ns.qualify(&Name::from_str("type").unwrap());
    assert_eq!(local.to_string(), "rdf:type");
}

// ------------------------------------------------------------------------------------------------
// Name accessors and error branches
// ------------------------------------------------------------------------------------------------

#[test]
fn name_helpers() {
    assert!(Name::is_valid_str("type"));
    assert!(!Name::is_valid_str("rdf:type"));

    let name = Name::from_str("type").unwrap();
    assert_eq!(name.as_ref(), "type");
    assert_eq!(String::from(name.clone()), "type");
    assert_eq!(String::from(&name), "type");

    let qualified = name.qualify(&Namespace::from_str("rdf:").unwrap());
    assert_eq!(qualified.to_string(), "rdf:type");
}

#[test]
fn name_from_str_error_branches() {
    assert!(matches!(
        Name::from_str(""),
        Err(NameParseError::EmptyString)
    ));
    assert!(matches!(
        Name::from_str("trailing."),
        Err(NameParseError::InvalidCharacter(_))
    ));
    assert!(matches!(
        Name::from_str("bad space"),
        Err(NameParseError::InvalidCharacter(_))
    ));
}

#[test]
fn name_new_unchecked_passes_through() {
    let name = Name::new_unchecked("anything-goes");
    assert_eq!(name.to_string(), "anything-goes");
}

// ------------------------------------------------------------------------------------------------
// LocalName accessors
// ------------------------------------------------------------------------------------------------

#[test]
fn local_name_conversions() {
    let local = LocalName::from_str("rdf:type").unwrap();
    assert!(!local.is_namespace_default());
    assert_eq!(String::from(local.clone()), "rdf:type");
    assert_eq!(String::from(&local), "rdf:type");
    assert_eq!(local.as_curie(), "[rdf:type]");
}

// ------------------------------------------------------------------------------------------------
// PrefixedName Display
// ------------------------------------------------------------------------------------------------

#[test]
fn prefixed_name_display() {
    let ns = PrefixedName::Namespace(Namespace::from_str("rdf:").unwrap());
    assert_eq!(ns.to_string(), "rdf:");
    let local = PrefixedName::Local(LocalName::from_str("rdf:type").unwrap());
    assert_eq!(local.to_string(), "rdf:type");
}

// ------------------------------------------------------------------------------------------------
// IriRef From impls
// ------------------------------------------------------------------------------------------------

#[test]
fn iri_ref_from_variants() {
    let iri = Iri::from_str("<https://example.org/>").unwrap();
    let r1: IriRef = iri.clone().into();
    let r2: IriRef = (&iri).into();
    assert_eq!(r1, r2);
    assert!(r1.is_iri());

    let pname = PrefixedName::Namespace(Namespace::from_str("rdf:").unwrap());
    let r3: IriRef = pname.clone().into();
    let r4: IriRef = (&pname).into();
    assert_eq!(r3, r4);
    assert!(r3.is_prefixed_name());
}

#[test]
fn iri_extra_fragment_helpers() {
    let iri = Iri::from_str("<https://example.org/ns#name>").unwrap();
    assert_eq!(
        iri.with_no_fragment().to_string(),
        "<https://example.org/ns>",
    );
    assert_eq!(
        iri.with_empty_fragment().to_string(),
        "<https://example.org/ns#>",
    );
}

#[test]
fn iri_extra_with_new_path() {
    let iri = Iri::from_str("<https://example.org/old>").unwrap();
    let updated = iri.with_new_path("/new");
    assert_eq!(updated.to_string(), "<https://example.org/new>");
}

#[test]
fn iri_extra_split_and_make_name_negative() {
    // No fragment, no usable path tail, and a query => split is None.
    let iri = Iri::from_str("<https://example.org/?q=1>").unwrap();
    assert_eq!(iri.split(), None);
    assert_eq!(iri.namespace(), None);
    assert_eq!(iri.name(), None);

    // make_name on a path without trailing '/' must yield None.
    let iri = Iri::from_str("<https://example.org/ns>").unwrap();
    assert_eq!(iri.make_name(Name::from_str("Name").unwrap()), None);

    // make_name on a path with trailing '/' but a query must yield None.
    let iri = Iri::from_str("<https://example.org/ns/?x=1>").unwrap();
    assert_eq!(iri.make_name(Name::from_str("Name").unwrap()), None);
}

#[test]
#[cfg(feature = "genid")]
fn iri_extra_genid_format() {
    let base = Iri::from_str("<https://example.org/>").unwrap();
    let id = base.genid().unwrap();
    let s = id.to_string();
    assert!(s.starts_with("<https://example.org/.well-known/genid/"));
    // 32 hex chars after the path.
    let tail = s
        .strip_prefix("<https://example.org/.well-known/genid/")
        .unwrap();
    assert_eq!(tail.len(), 33); // 32 character ID + '>'
}

// ------------------------------------------------------------------------------------------------
// IriPrefixMap iterators and mutation
// ------------------------------------------------------------------------------------------------

#[test]
fn prefix_map_empty_and_iterators() {
    let mut map = IriPrefixMap::default();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert_eq!(map.mappings().count(), 0);
    assert_eq!(map.prefixes().count(), 0);
    assert_eq!(map.iris().count(), 0);

    map.set_default_namespace(Iri::from_str("<https://example.org/>").unwrap());
    assert!(!map.is_empty());
    assert_eq!(map.prefixes().count(), 1);
    assert_eq!(map.iris().count(), 1);

    map.remove_default_namespace();
    assert!(map.is_empty());
}

#[test]
fn prefix_map_insert_vocabulary_and_remove_clear() {
    let mut map = IriPrefixMap::default();
    map.insert_vocabulary(&VOCABULARY_RDF);
    assert_eq!(map.len(), 1);

    map.remove(&Namespace::from_str("rdf:").unwrap());
    assert_eq!(map.len(), 0);

    map.insert_vocabulary(&VOCABULARY_RDF);
    map.insert_vocabulary(&VOCABULARY_OWL);
    assert_eq!(map.len(), 2);
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn prefix_map_display_contains_mapping() {
    let map = IriPrefixMap::default().with(
        Namespace::from_str("rdf:").unwrap(),
        Iri::from_str("<http://www.w3.org/1999/02/22-rdf-syntax-ns#>").unwrap(),
    );
    let s = map.to_string();
    assert!(s.contains("rdf:"));
    assert!(s.contains("rdf-syntax-ns#"));
}

#[test]
fn prefix_map_compress_returns_none_for_non_namespace_iri() {
    let map = IriPrefixMap::common();
    let iri = Iri::from_str("<https://example.org>").unwrap();
    assert_eq!(map.compress(&iri), None);
}

// ------------------------------------------------------------------------------------------------
// Vocabulary accessors
// ------------------------------------------------------------------------------------------------

#[test]
fn vocabulary_accessors() {
    let v = VOCABULARY_OWL;
    assert_eq!(v.prefix(), "owl");
    assert_eq!(v.iri(), "http://www.w3.org/2002/07/owl#");
    assert_eq!(v.description(), None);

    let v = VOCABULARY_RDF;
    assert!(v.description().is_some());

    let custom = Vocabulary::new("ex", "https://example.org/ns#").with_description("Example");
    assert_eq!(custom.prefix(), "ex");
    assert_eq!(custom.iri(), "https://example.org/ns#");
    assert_eq!(custom.description(), Some(&"Example"));
}

#[test]
fn vocabulary_constants_are_well_formed() {
    let all = [
        VOCABULARY_CAL,
        VOCABULARY_DBPEDIA,
        VOCABULARY_DC_ELEMENTS,
        VOCABULARY_DC_TERMS,
        VOCABULARY_DOAP,
        VOCABULARY_FOAF,
        VOCABULARY_GEO_NAMES,
        VOCABULARY_ISO_SKOS,
        VOCABULARY_OPEN_GIS,
        VOCABULARY_ORG,
        VOCABULARY_OWL,
        VOCABULARY_RDF,
        VOCABULARY_RDF_SCHEMA,
        VOCABULARY_SIOC,
        VOCABULARY_SKOS,
        VOCABULARY_SKOS_XL,
        VOCABULARY_VANN,
        VOCABULARY_VOID,
        VOCABULARY_WOT,
        VOCABULARY_XML,
        VOCABULARY_XML_SCHEMA,
    ];
    for v in all {
        // These panic if the bundled string is malformed.
        let _ = v.prefix_as_namespace();
        let _ = v.iri_as_iri();
    }
}

// ------------------------------------------------------------------------------------------------
// Error From conversions
// ------------------------------------------------------------------------------------------------

#[test]
fn error_from_conversions() {
    let e: Error = NameParseError::EmptyString.into();
    assert!(e.is_name());
    let bad = Iri::from_str("");
    let e: Error = bad.unwrap_err().into();
    assert!(e.is_url());
}
