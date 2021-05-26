use rdftk_iri::IRI;
use std::str::FromStr;

#[test]
fn is_well_known() {
    let iri =
        IRI::from_str("http://example.com/.well-known/genid/d26a2d0e98334696f4ad70a677abc1f6")
            .unwrap();
    assert!(iri.is_well_known());
}

#[test]
fn is_not_well_known() {
    let iri = IRI::from_str("http://example.com/.well-nown/genid/d26a2d0e98334696f4ad70a677abc1f6")
        .unwrap();
    assert!(!iri.is_well_known());
}
