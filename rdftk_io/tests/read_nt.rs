use objio::{ObjectReader, ObjectWriter};
use rdftk_core::{error::Error, model::graph::GraphRef};
use rdftk_io::nt::{NTripleReader, NTripleWriter};

fn write_graph(graph: &GraphRef) {
    let writer = NTripleWriter::default();
    let _ = writer.write(&mut std::io::stdout(), graph);
}

#[test]
fn parse_simple() {
    let reader = NTripleReader::default();
    let result: Result<GraphRef, Error> = reader.read_from_string(
            r###"
<http://example.org/show/218> <http://www.w3.org/2000/01/rdf-schema#label> "That Seventies Show"^^<http://www.w3.org/2001/XMLSchema#string> . # literal with XML Schema string datatype
<http://example.org/show/218> <http://www.w3.org/2000/01/rdf-schema#label> "That Seventies Show" . # same as above
<http://example.org/show/218> <http://example.org/show/localName> "That Seventies Show"@en . # literal with a language tag
<http://example.org/show/218> <http://example.org/show/localName> "Cette Série des Années Septante"@fr-be .  # literal outside of ASCII range with a region subtag
<http://example.org/#spiderman> <http://example.org/text> "This is a multi-line\nliteral with many quotes (\"\"\"\"\")\nand two apostrophes ('')." .
<http://en.wikipedia.org/wiki/Helium> <http://example.org/elements/atomicNumber> "2"^^<http://www.w3.org/2001/XMLSchema#integer> . # xsd:integer
<http://en.wikipedia.org/wiki/Helium> <http://example.org/elements/specificGravity> "1.663E-4"^^<http://www.w3.org/2001/XMLSchema#double> .     # xsd:double
"### );
    match result {
        Ok(g) => {
            println!("ok");
            write_graph(&g);
        }
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    }
}

#[test]
fn parse_simple_with_blanks() {
    let reader = NTripleReader::default();
    let result: Result<GraphRef, Error> = reader.read_from_string(
            r###"
<http://one.example/subject1> <http://one.example/predicate1> <http://one.example/object1> . # comments here
# or on a line by themselves
_:subject1 <http://an.example/predicate1> "object1" .
_:subject2 <http://an.example/predicate2> "object2" .
"### );
    match result {
        Ok(g) => {
            println!("ok");
            write_graph(&g);
        }
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    }
}
