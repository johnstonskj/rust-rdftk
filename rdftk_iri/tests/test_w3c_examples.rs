use rdftk_iri::IRI;
use std::str::FromStr;

#[test]
pub fn from_ntriple_suite() {
    let result = IRI::from_str("scheme:!$%25&'()*+,-./0123456789:/@ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz~?#");
    println!("{:?}", result);
}
