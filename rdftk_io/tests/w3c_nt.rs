use objio::ObjectReader;
use rdftk_core::error::eprint_trace;
use rdftk_core::model::graph::GraphRef;
use rdftk_io::nt::NTripleReader;
use std::path::PathBuf;

macro_rules! positive_test {
    ($name:ident, $comment:expr, $file:expr) => {
        #[test]
        fn $name() {
            println!($comment);
            match read_test_file($file) {
                Ok(_) => {}
                Err(e) => {
                    eprint_trace(&e);
                    panic!("{}", e);
                }
            }
        }
    };
}

macro_rules! negative_test {
    ($name:ident, $comment:expr, $file:expr) => {
        #[test]
        fn $name() {
            println!($comment);
            assert!(read_test_file($file).is_err());
        }
    };
}

fn read_test_file(file_name: &str) -> Result<GraphRef, rdftk_core::error::Error> {
    let file_path = PathBuf::from(format!("tests/w3c/nt/{}.nt", file_name));
    let reader = NTripleReader::default();
    reader.read_from_file(file_path)
}

positive_test!(nt_syntax_file_01, "Empty file", "nt-syntax-file-01");

positive_test!(nt_syntax_file_02, "Only comment", "nt-syntax-file-02");

positive_test!(
    nt_syntax_file_03,
    "One comment, one empty line",
    "nt-syntax-file-03"
);

positive_test!(nt_syntax_uri_01, "Only IRIs", "nt-syntax-uri-01");

positive_test!(
    nt_syntax_uri_02,
    "IRIs with Unicode escape",
    "nt-syntax-uri-02"
);

positive_test!(
    nt_syntax_uri_03,
    "IRIs with long Unicode escape",
    "nt-syntax-uri-03"
);

positive_test!(nt_syntax_uri_04, "Legal IRIs", "nt-syntax-uri-04");

positive_test!(nt_syntax_string_01, "string literal", "nt-syntax-string-01");

positive_test!(
    nt_syntax_string_02,
    "langString literal",
    "nt-syntax-string-02"
);

positive_test!(
    nt_syntax_string_03,
    "langString literal with region",
    "nt-syntax-string-03"
);

positive_test!(
    nt_syntax_str_esc_01,
    "string literal with escaped newline",
    "nt-syntax-str-esc-01"
);

positive_test!(
    nt_syntax_str_esc_02,
    "string literal with Unicode escape",
    "nt-syntax-str-esc-02"
);

positive_test!(
    nt_syntax_str_esc_03,
    "string literal with long Unicode escape",
    "nt-syntax-str-esc-03"
);

positive_test!(nt_syntax_bnode_01, "bnode subject", "nt-syntax-bnode-01");

positive_test!(nt_syntax_bnode_02, "bnode object", "nt-syntax-bnode-02");

positive_test!(
    nt_syntax_bnode_03,
    "Blank node labels may start with a digit",
    "nt-syntax-bnode-03"
);

positive_test!(
    nt_syntax_datatypes_01,
    "xsd:byte literal",
    "nt-syntax-datatypes-01"
);

positive_test!(
    nt_syntax_datatypes_02,
    "integer as xsd:string",
    "nt-syntax-datatypes-02"
);

negative_test!(
    nt_syntax_bad_uri_01,
    "Bad IRI : space (negative test)",
    "nt-syntax-bad-uri-01"
);

negative_test!(
    nt_syntax_bad_uri_02,
    "Bad IRI : bad escape (negative test)",
    "nt-syntax-bad-uri-02"
);

negative_test!(
    nt_syntax_bad_uri_03,
    "Bad IRI : bad long escape (negative test)",
    "nt-syntax-bad-uri-03"
);

negative_test!(
    nt_syntax_bad_uri_04,
    "Bad IRI : character escapes not allowed (negative test)",
    "nt-syntax-bad-uri-04"
);

negative_test!(
    nt_syntax_bad_uri_05,
    "Bad IRI : character escapes not allowed (2) (negative test)",
    "nt-syntax-bad-uri-05"
);

negative_test!(
    nt_syntax_bad_uri_06,
    "Bad IRI : relative IRI not allowed in subject (negative test)",
    "nt-syntax-bad-uri-06"
);

negative_test!(
    nt_syntax_bad_uri_07,
    "Bad IRI : relative IRI not allowed in predicate (negative test)",
    "nt-syntax-bad-uri-07"
);

negative_test!(
    nt_syntax_bad_uri_08,
    "Bad IRI : relative IRI not allowed in object (negative test)",
    "nt-syntax-bad-uri-08"
);

negative_test!(
    nt_syntax_bad_uri_09,
    "Bad IRI : relative IRI not allowed in datatype (negative test)",
    "nt-syntax-bad-uri-09"
);

negative_test!(
    nt_syntax_bad_prefix_01,
    "@prefix not allowed in n-triples (negative test)",
    "nt-syntax-bad-prefix-01"
);

negative_test!(
    nt_syntax_bad_base_01,
    "@base not allowed in n-triples (negative test)",
    "nt-syntax-bad-base-01"
);

negative_test!(
    nt_syntax_bad_struct_01,
    "N-Triples does not have objectList (negative test)",
    "nt-syntax-bad-struct-01"
);

negative_test!(
    nt_syntax_bad_struct_02,
    "N-Triples does not have predicateObjectList (negative test)",
    "nt-syntax-bad-struct-02"
);

negative_test!(
    nt_syntax_bad_lang_01,
    "langString with bad lang (negative test)",
    "nt-syntax-bad-lang-01"
);

negative_test!(
    nt_syntax_bad_esc_01,
    "Bad string escape (negative test)",
    "nt-syntax-bad-esc-01"
);

negative_test!(
    nt_syntax_bad_esc_02,
    "Bad string escape (negative test)",
    "nt-syntax-bad-esc-02"
);

negative_test!(
    nt_syntax_bad_esc_03,
    "Bad string escape (negative test)",
    "nt-syntax-bad-esc-03"
);

negative_test!(
    nt_syntax_bad_string_01,
    "mismatching string literal open/close (negative test)",
    "nt-syntax-bad-string-01"
);

negative_test!(
    nt_syntax_bad_string_02,
    "mismatching string literal open/close (negative test)",
    "nt-syntax-bad-string-02"
);

negative_test!(
    nt_syntax_bad_string_03,
    "single quotes (negative test)",
    "nt-syntax-bad-string-03"
);

negative_test!(
    nt_syntax_bad_string_04,
    "long single string literal (negative test)",
    "nt-syntax-bad-string-04"
);

negative_test!(
    nt_syntax_bad_string_05,
    "long double string literal (negative test)",
    "nt-syntax-bad-string-05"
);

negative_test!(
    nt_syntax_bad_string_06,
    "string literal with no end (negative test)",
    "nt-syntax-bad-string-06"
);

negative_test!(
    nt_syntax_bad_string_07,
    "string literal with no start (negative test)",
    "nt-syntax-bad-string-07"
);

negative_test!(
    nt_syntax_bad_num_01,
    "no numbers in N-Triples (integer) (negative test)",
    "nt-syntax-bad-num-01"
);

negative_test!(
    nt_syntax_bad_num_02,
    "no numbers in N-Triples (decimal) (negative test)",
    "nt-syntax-bad-num-02"
);

negative_test!(
    nt_syntax_bad_num_03,
    "no numbers in N-Triples (float) (negative test)",
    "nt-syntax-bad-num-03"
);

positive_test!(
    nt_syntax_subm_01,
    "Submission test from Original RDF Test Cases",
    "nt-syntax-subm-01"
);

positive_test!(
    comment_following_triple,
    "Tests comments after a triple",
    "comment_following_triple"
);

positive_test!(
    literal_ascii_boundaries,
    "literal_ascii_boundaries '\\x00\\x26\\x28...'",
    "literal_ascii_boundaries"
);

positive_test!(
    literal_with_utf8_boundaries,
    "literal_with_UTF8_boundaries '\\x80\\x7ff\\x800\\xfff...'",
    "literal_with_UTF8_boundaries"
);

positive_test!(
    literal_all_controls,
    "literal_all_controls '\\x00\\x01\\x02\\x03\\x04...'",
    "literal_all_controls"
);

positive_test!(
    literal_with_squote,
    "literal with squote \"x'y\"",
    "literal_with_squote"
);

positive_test!(
    literal_with_2_squotes,
    "literal with 2 squotes \"x''y\"",
    "literal_with_2_squotes"
);

positive_test!(literal, "literal \"\"\"x\"\"\"", "literal");

positive_test!(
    literal_with_dquote,
    r##"literal with dquote "x\"y""##,
    "literal_with_dquote"
);

positive_test!(
    literal_with_reverse_solidus2,
    r##"literal with 2 squotes \"\"\"a\"\"b\"\"\""##,
    "literal_with_REVERSE_SOLIDUS2"
);

positive_test!(
    literal_with_character_tabulation,
    "literal with CHARACTER TABULATION",
    "literal_with_CHARACTER_TABULATION"
);

positive_test!(
    literal_with_backspace,
    "literal with BACKSPACE",
    "literal_with_BACKSPACE"
);

positive_test!(
    literal_with_line_feed,
    "literal with LINE FEED",
    "literal_with_LINE_FEED"
);

positive_test!(
    literal_with_carriage_return,
    "literal with CARRIAGE RETURN",
    "literal_with_CARRIAGE_RETURN"
);

positive_test!(
    literal_with_form_feed,
    "literal with FORM FEED",
    "literal_with_FORM_FEED"
);

positive_test!(
    literal_with_reverse_solidus,
    "literal with REVERSE SOLIDUS",
    "literal_with_REVERSE_SOLIDUS"
);

positive_test!(
    literal_with_numeric_escape4,
    "literal with numeric escape4 \\u",
    "literal_with_numeric_escape4"
);

positive_test!(
    literal_with_numeric_escape8,
    "literal with numeric escape8 \\U",
    "literal_with_numeric_escape8"
);

positive_test!(
    langtagged_string,
    "langtagged string \"x\"@en",
    "langtagged_string"
);

positive_test!(
    lantag_with_subtag,
    "lantag with subtag \"x\"@en-us",
    "lantag_with_subtag"
);

positive_test!(
    minimal_whitespace,
    "tests absense of whitespace between subject, predicate, object and end-of-statement",
    "minimal_whitespace"
);
