pub mod common;
pub use common::*;

#[test]
fn test_simple_success() {
    parse_success("");
    parse_success("http://www.example.com/foo/bar");
    parse_success("http://www.example.com/#𐌀ss");
    parse_success("http://www.пример.com/#𐌀ss");
}

#[test]
fn test_simple_failure() {
    parse_failure("http://www.example.com/#hello, world");
    parse_failure("http://www.example.com/#asdf#qwer");
    parse_failure("http://www.example.com/##asdf");
}
