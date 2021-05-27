/*!
Internal parsing functions.
*/

use regex::Regex;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref RE_SCHEME: Regex = Regex::new("^[[:alpha:]][[[:alnum:]]+-\\.]*$").unwrap();
}

pub(crate) fn is_scheme(s: &str) -> bool {
    // From RFC-2396, appendix A. _Collected BNF for URI_:
    // scheme = alpha *( alpha | digit | '+' | '-' | '.' )
    RE_SCHEME.is_match(s)
}

pub(crate) fn is_ihost(s: &str) -> bool {
    // ihost          = IP-literal / IPv4address / ireg-name
    // IPv4address    = dec-octet "." dec-octet "." dec-octet "." dec-octet
    // IP-literal     = "[" ( IPv6address / IPvFuture  ) "]"
    // IPvFuture      = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
    // IPv6address    = /* +HEXDIG / ":" */
    // ireg-name      = *( iunreserved / pct-encoded / sub-delims )
    s.is_empty()
        || (s.chars().all(|c| {
            is_iunreserved(c) || is_sub_delims(c) || c == '[' || c == ']' || c == ':' || c == '.'
        }) && is_correctly_escaped(s))
}

pub(crate) fn is_ireg_name(s: &str) -> bool {
    // ireg-name      = *( iunreserved / pct-encoded / sub-delims )
    s.is_empty()
        || (s
            .chars()
            .all(|c| is_iunreserved(c) || is_sub_delims(c) || c == '.')
            && is_correctly_escaped(s))
}

pub(crate) fn is_iuserinfo(s: &str) -> bool {
    s.is_empty()
        || (s.chars().all(|c| is_iunreserved(c) || is_sub_delims(c)) && is_correctly_escaped(s))
}

// pub(crate) fn is_port(s: &str) -> bool {
//     s.chars().all(|c| c.is_ascii_digit())
// }

pub(crate) fn is_path(s: &str) -> bool {
    s.split('/').all(is_path_segment)
}

pub(crate) fn is_path_segment(s: &str) -> bool {
    let parts = s.split(';').collect::<Vec<&str>>();
    if parts.len() > 2 {
        false
    } else {
        parts
            .iter()
            .all(|s| s.is_empty() || s.chars().all(is_ipchar) && is_correctly_escaped(s))
    }
}

pub(crate) fn is_iquery(s: &str) -> bool {
    s.is_empty()
        || (s
            .chars()
            .all(|c| is_ipchar(c) || is_iprivate(c) || c == '/' || c == '?')
            && is_correctly_escaped(s))
}

pub(crate) fn is_ifragment(s: &str) -> bool {
    s.is_empty()
        || (s.chars().all(|c| is_ipchar(c) || c == '/' || c == '?') && is_correctly_escaped(s))
}

// ------------------------------------------------------------------------------------------------

pub(crate) fn is_correctly_escaped(s: &str) -> bool {
    let chars = &s.chars().collect::<Vec<char>>();
    for window in chars.windows(3) {
        if window[0] == '%' && !(window[1].is_ascii_hexdigit() && window[2].is_ascii_hexdigit()) {
            return false;
        }
    }
    true
}

#[inline]
pub(crate) fn is_ipchar(c: char) -> bool {
    // Have to include '%' here as the is_correctly_escaped rule is run after this check.
    is_iunreserved(c) || is_sub_delims(c) || c == ':' || c == '@' || c == '%'
}

// #[inline]
// pub(crate) fn is_uric(c: char) -> bool {
//     is_reserved(c) || is_unreserved(c) // || is_escaped
// }

// #[inline]
// pub(crate) fn is_reserved(c: char) -> bool {
//     is_gen_delims(c) || is_sub_delims(c)
// }

// #[inline]
// pub(crate) fn is_gen_delims(c: char) -> bool {
//     c == ':' || c == '/' || c == '?' || c == '#' || c == '[' || c == ']' || c == '@'
// }

#[inline]
pub(crate) fn is_sub_delims(c: char) -> bool {
    c == '!'
        || c == '$'
        || c == '&'
        || c == '\''
        || c == '('
        || c == ')'
        || c == '*'
        || c == '+'
        || c == ','
        || c == ';'
        || c == '='
}

#[inline]
pub(crate) fn is_iunreserved(c: char) -> bool {
    is_unreserved(c) || is_ucschar(c)
}

#[inline]
pub(crate) fn is_unreserved(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_' || c == '~'
}

#[inline]
pub(crate) fn is_ucschar(c: char) -> bool {
    ('\u{00A0}'..='\u{D7FF}').contains(&c)
        || ('\u{F900}'..='\u{FDCF}').contains(&c)
        || ('\u{FDF0}'..='\u{FFEF}').contains(&c)
        || ('\u{10000}'..='\u{1FFFD}').contains(&c)
        || ('\u{20000}'..='\u{2FFFD}').contains(&c)
        || ('\u{30000}'..='\u{3FFFD}').contains(&c)
        || ('\u{40000}'..='\u{4FFFD}').contains(&c)
        || ('\u{50000}'..='\u{5FFFD}').contains(&c)
        || ('\u{60000}'..='\u{6FFFD}').contains(&c)
        || ('\u{70000}'..='\u{7FFFD}').contains(&c)
        || ('\u{80000}'..='\u{8FFFD}').contains(&c)
        || ('\u{90000}'..='\u{9FFFD}').contains(&c)
        || ('\u{A0000}'..='\u{AFFFD}').contains(&c)
        || ('\u{B0000}'..='\u{BFFFD}').contains(&c)
        || ('\u{C0000}'..='\u{CFFFD}').contains(&c)
        || ('\u{D0000}'..='\u{DFFFD}').contains(&c)
        || ('\u{E0000}'..='\u{EFFFD}').contains(&c)
}

#[inline]
pub(crate) fn is_iprivate(c: char) -> bool {
    ('\u{E000}'..='\u{F8FF}').contains(&c)
        || ('\u{F0000}'..='\u{FFFFD}').contains(&c)
        || ('\u{100000}'..='\u{10FFFD}').contains(&c)
}
