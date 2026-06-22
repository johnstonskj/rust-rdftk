//!
//! Property tests for the `FromStr` parsers of `Namespace`, `Name`, and `LocalName`.
//!
//! Each strategy mirrors one of the SPARQL grammar productions used by the
//! crate-level documentation:
//!
//! ```text
//! PN_CHARS_BASE ::= [A-Z] | [a-z] | <high-codepoint ranges>
//! PN_CHARS_U    ::= PN_CHARS_BASE | '_'
//! PN_CHARS      ::= PN_CHARS_U | '-' | [0-9] | #x00B7 | ...
//! PN_PREFIX     ::= PN_CHARS_BASE ((PN_CHARS|'.')* PN_CHARS)?
//! PN_LOCAL      ::= (PN_CHARS_U | [0-9]) ((PN_CHARS|'.')* PN_CHARS)?
//! ```
//!

use proptest::prelude::*;
use rdftk_iri::{LocalName, Name, Namespace};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Character-class strategies
// ------------------------------------------------------------------------------------------------

fn pn_chars_base() -> impl Strategy<Value = char> {
    prop_oneof![
        prop::char::range('A', 'Z'),
        prop::char::range('a', 'z'),
        prop::char::range('\u{00C0}', '\u{00D6}'),
        prop::char::range('\u{00D8}', '\u{00F6}'),
        prop::char::range('\u{00F8}', '\u{02FF}'),
        prop::char::range('\u{0370}', '\u{037D}'),
        prop::char::range('\u{037F}', '\u{1FFF}'),
        prop::char::range('\u{200C}', '\u{200D}'),
        prop::char::range('\u{2070}', '\u{218F}'),
        prop::char::range('\u{2C00}', '\u{2FEF}'),
        prop::char::range('\u{3001}', '\u{D7FF}'),
        prop::char::range('\u{F900}', '\u{FDCF}'),
        prop::char::range('\u{FDF0}', '\u{FFFD}'),
        prop::char::range('\u{10000}', '\u{EFFFF}'),
    ]
}

fn pn_chars_u() -> impl Strategy<Value = char> {
    prop_oneof![pn_chars_base(), Just('_')]
}

fn pn_chars() -> impl Strategy<Value = char> {
    prop_oneof![
        pn_chars_u(),
        Just('-'),
        prop::char::range('0', '9'),
        Just('\u{00B7}'),
        prop::char::range('\u{0300}', '\u{036F}'),
        prop::char::range('\u{203F}', '\u{2040}'),
    ]
}

fn pn_local_first() -> impl Strategy<Value = char> {
    prop_oneof![pn_chars_u(), prop::char::range('0', '9')]
}

/// Build a grammar-valid `PN_PREFIX`: a leading `PN_CHARS_BASE` followed by an
/// optional body that ends with `PN_CHARS` (so it cannot end with `.`).
fn pn_prefix_strategy() -> impl Strategy<Value = String> {
    (
        pn_chars_base(),
        prop::collection::vec(prop_oneof![pn_chars(), Just('.')], 0..10),
        prop::option::of(pn_chars()),
    )
        .prop_map(|(first, middle, tail)| {
            let mut s = String::new();
            s.push(first);
            if let Some(end) = tail {
                for c in middle {
                    s.push(c);
                }
                s.push(end);
            }
            s
        })
}

/// Build a grammar-valid `PN_LOCAL` (a `Name`).
fn pn_local_strategy() -> impl Strategy<Value = String> {
    (
        pn_local_first(),
        prop::collection::vec(prop_oneof![pn_chars(), Just('.')], 0..10),
        prop::option::of(pn_chars()),
    )
        .prop_map(|(first, middle, tail)| {
            let mut s = String::new();
            s.push(first);
            if let Some(end) = tail {
                for c in middle {
                    s.push(c);
                }
                s.push(end);
            }
            s
        })
}

// ------------------------------------------------------------------------------------------------
// Namespace properties
// ------------------------------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        .. ProptestConfig::default()
    })]

    /// Any grammar-valid prefix followed by ':' must parse as a Namespace.
    #[test]
    fn namespace_accepts_grammar_valid(prefix in pn_prefix_strategy()) {
        let input = format!("{prefix}:");
        let parsed = Namespace::from_str(&input);
        prop_assert!(parsed.is_ok(), "expected {input:?} to parse, got {parsed:?}");
    }

    /// Namespace round-trips through `to_string`.
    #[test]
    fn namespace_round_trip(prefix in pn_prefix_strategy()) {
        let input = format!("{prefix}:");
        let ns = Namespace::from_str(&input).unwrap();
        prop_assert_eq!(ns.to_string(), input);
    }

    /// Namespace without a trailing ':' must always fail.
    #[test]
    fn namespace_rejects_no_separator(prefix in pn_prefix_strategy()) {
        prop_assert!(Namespace::from_str(&prefix).is_err());
    }

    /// Namespace with more than one ':' must fail.
    #[test]
    fn namespace_rejects_extra_separator(a in pn_prefix_strategy(), b in pn_prefix_strategy()) {
        let input = format!("{a}:{b}:");
        prop_assert!(Namespace::from_str(&input).is_err());
    }

    /// new_unchecked agrees with from_str on grammar-valid input.
    #[test]
    fn namespace_new_unchecked_matches(prefix in pn_prefix_strategy()) {
        let input = format!("{prefix}:");
        let parsed = Namespace::from_str(&input).unwrap();
        prop_assert_eq!(parsed, Namespace::new_unchecked(prefix));
    }
}

// ------------------------------------------------------------------------------------------------
// Name properties
// ------------------------------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        .. ProptestConfig::default()
    })]

    #[test]
    fn name_accepts_grammar_valid(s in pn_local_strategy()) {
        let parsed = Name::from_str(&s);
        prop_assert!(parsed.is_ok(), "expected {s:?} to parse, got {parsed:?}");
    }

    #[test]
    fn name_round_trip(s in pn_local_strategy()) {
        let name = Name::from_str(&s).unwrap();
        prop_assert_eq!(name.to_string(), s);
    }

    /// Names ending in '.' are explicitly rejected.
    #[test]
    fn name_rejects_trailing_dot(s in pn_local_strategy()) {
        let input = format!("{s}.");
        prop_assert!(Name::from_str(&input).is_err());
    }

    /// A `Name` containing a top-level ':' is not a `Name`.
    #[test]
    fn name_rejects_colon(a in pn_local_strategy(), b in pn_local_strategy()) {
        let input = format!("{a}:{b}");
        prop_assert!(Name::from_str(&input).is_err());
    }
}

// ------------------------------------------------------------------------------------------------
// LocalName properties
// ------------------------------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        .. ProptestConfig::default()
    })]

    /// LocalName decomposes into the same parts as the individual parsers produce.
    #[test]
    fn local_name_decomposes(prefix in pn_prefix_strategy(), local in pn_local_strategy()) {
        let input = format!("{prefix}:{local}");
        let parsed = LocalName::from_str(&input).unwrap();
        prop_assert_eq!(parsed.namespace().to_string(), format!("{prefix}:"));
        prop_assert_eq!(parsed.name().to_string(), local);
    }

    /// LocalName round-trips through `to_string`.
    #[test]
    fn local_name_round_trip(prefix in pn_prefix_strategy(), local in pn_local_strategy()) {
        let input = format!("{prefix}:{local}");
        let parsed = LocalName::from_str(&input).unwrap();
        prop_assert_eq!(parsed.to_string(), input);
    }

    /// LocalName with empty prefix == default namespace.
    #[test]
    fn local_name_default_namespace(local in pn_local_strategy()) {
        let input = format!(":{local}");
        let parsed = LocalName::from_str(&input).unwrap();
        prop_assert!(parsed.is_namespace_default());
    }
}

// ------------------------------------------------------------------------------------------------
// Explicit boundary cases (regression of the `'Z'`/`'z'` exclusive-range bug).
// ------------------------------------------------------------------------------------------------

#[test]
fn ascii_boundaries_accepted() {
    for ch in ['A', 'Z', 'a', 'z'] {
        let ns = format!("{ch}:");
        assert!(
            Namespace::from_str(&ns).is_ok(),
            "{ns} should parse as Namespace",
        );
        let name = format!("{ch}");
        assert!(
            Name::from_str(&name).is_ok(),
            "{name} should parse as Name",
        );
    }

    for prefix in ["Zoo", "zoo", "Apple", "apple"] {
        assert!(Namespace::from_str(&format!("{prefix}:")).is_ok());
        assert!(Name::from_str(prefix).is_ok());
        assert!(LocalName::from_str(&format!("{prefix}:bar")).is_ok());
    }
}

#[test]
fn name_allows_leading_digit() {
    // SPARQL PN_LOCAL allows a leading digit; XML local names do not.
    assert!(Name::from_str("9foo").is_ok());
    assert!(Name::from_str("0").is_ok());
}

#[test]
fn names_reject_empty_and_obvious_garbage() {
    assert!(Namespace::from_str("").is_err());
    assert!(Name::from_str("").is_err());
    assert!(LocalName::from_str("").is_err());
    assert!(Name::from_str(" ").is_err());
    assert!(Name::from_str("-leading-dash").is_err());
    assert!(Name::from_str("trailing.").is_err());
}
