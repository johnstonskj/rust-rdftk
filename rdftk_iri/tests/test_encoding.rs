use rdftk_iri::{PercentEncoding, IRI};
use std::str::FromStr;

#[test]
fn test_percent_encoding() {
    let iri = IRI::from_str("http://example.org/translate/དལཻ་ལམ་").unwrap();
    assert_eq!(iri.to_string(), "http://example.org/translate/དལཻ་ལམ་");
    let uri = iri.encode(true);
    assert_eq!(uri.to_string(), "http://example.org/translate/%E0%BD%91%E0%BD%A3%E0%BD%BB%E0%BC%8B%E0%BD%A3%E0%BD%98%E0%BC%8B".to_string());
}
