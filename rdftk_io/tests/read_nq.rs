use objio::ObjectReader;
use rdftk_io::nq::NQuadReader;

#[test]
fn parse_simple_1() {
    let reader = NQuadReader::default();
    assert!(reader.read_from_string(
            r###"
<http://one.example/subject1> <http://one.example/predicate1> <http://one.example/object1> <http://example.org/graph3> . # comments here
# or on a line by themselves
_:subject1 <http://an.example/predicate1> "object1" <http://example.org/graph1> .
_:subject2 <http://an.example/predicate2> "object2" <http://example.org/graph5> .
"###).is_ok());
}

#[test]
fn parse_simple_2() {
    let reader = NQuadReader::default();
    assert!(reader
        .read_from_string(
            r###"
_:alice <http://xmlns.com/foaf/0.1/knows> _:bob <http://example.org/graphs/john> .
_:bob <http://xmlns.com/foaf/0.1/knows> _:alice <http://example.org/graphs/james> .
"###,
        )
        .is_ok());
}
