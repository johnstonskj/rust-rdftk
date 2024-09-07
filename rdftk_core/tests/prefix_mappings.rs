use rdftk_core::model::graph::PrefixMappingRef;
use rdftk_core::model::qname::QName;
use rdftk_core::simple::mapping::common_mappings;
use rdftk_iri::{Iri, IriRef, Name};
use std::str::FromStr;

fn make_mappings() -> PrefixMappingRef {
    let mappings = common_mappings();
    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.set_default_namespace(IriRef::from(
            Iri::from_str("http://xmlns.com/foaf/0.1/").unwrap(),
        ));
    }
    mappings
}

#[test]
fn test_construct_mappings() {
    let mappings = make_mappings();
    let mappings = mappings.borrow();

    assert_eq!(mappings.len(), 5);

    assert!(mappings.get_default_namespace().is_some());

    assert!(mappings
        .get_namespace(&Name::new_unchecked("owl"))
        .is_some());
    assert!(mappings
        .get_namespace(&Name::new_unchecked("xsd"))
        .is_some());
    assert!(mappings
        .get_namespace(&Name::new_unchecked("rdf"))
        .is_some());
    assert!(mappings
        .get_namespace(&Name::new_unchecked("rdfs"))
        .is_some());
}

#[test]
fn test_mapping_expand() {
    let mappings = make_mappings();

    {
        let mut mut_mappings = mappings.borrow_mut();
        mut_mappings.insert(
            Name::new_unchecked("foo"),
            IriRef::from(Iri::from_str("http://example.com/schema/foo/1.0/").unwrap()),
        );
    }

    let mappings = mappings.borrow();
    assert_eq!(
        mappings.expand(&QName::new_unchecked(
            Some(Name::new_unchecked("rdf")),
            Name::new_unchecked("Bag")
        )),
        Some(IriRef::from(
            Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag").unwrap()
        ))
    );
    assert_eq!(
        mappings.expand(&QName::new_unchecked(None, Name::new_unchecked("knows"))),
        Some(IriRef::from(
            Iri::from_str("http://xmlns.com/foaf/0.1/knows").unwrap()
        ))
    );
    assert_eq!(
        mappings.expand(&QName::new_unchecked(
            Some(Name::new_unchecked("foo")),
            Name::new_unchecked("Bar")
        )),
        Some(IriRef::from(
            Iri::from_str("http://example.com/schema/foo/1.0/Bar").unwrap()
        ))
    );

    assert_eq!(
        mappings.expand(&QName::new_unchecked(
            Some(Name::new_unchecked("rdfx")),
            Name::new_unchecked("Bag")
        )),
        None
    );
}

#[test]
fn test_mapping_compress() {
    let mappings = make_mappings();
    let mappings = mappings.borrow();

    assert_eq!(
        mappings.compress(&IriRef::from(
            Iri::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag").unwrap()
        )),
        Some(QName::new_unchecked(
            Some(Name::new_unchecked("rdf")),
            Name::new_unchecked("Bag")
        ))
    );
    assert_eq!(
        mappings.compress(&IriRef::from(
            Iri::from_str("http://xmlns.com/foaf/0.1/knows").unwrap()
        )),
        Some(QName::new_unchecked(None, Name::new_unchecked("knows")))
    );
    assert_eq!(
        mappings.compress(&IriRef::from(
            Iri::from_str("http://www.w3.org/2003/01/geo/wgs84_pos#SpatialThing").unwrap()
        )),
        None
    );
}
