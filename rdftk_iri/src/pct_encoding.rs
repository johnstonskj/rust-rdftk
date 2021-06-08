/*!
One-line description.

More detailed description, with

# Example

*/

use std::collections::BTreeMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn from_chars(chars: &[char]) -> BTreeMap<char, String> {
    let mut map: BTreeMap<char, String> = Default::default();

    for c in chars {
        let c = *c;
        let _ = map.insert(c, format!("%{:X}", c as u32));
    }
    map
}

pub(crate) fn code_point_map() -> BTreeMap<char, String> {
    let mut map: BTreeMap<char, String> = Default::default();

    let _ = map.insert('%', format!("%{:X}", '%' as u32));

    for i in 0..0x1F {
        let _ = map.insert(char::from_u32(i).unwrap(), format!("%{:X}", i));
    }
    map
}

pub(crate) fn user_info_map() -> BTreeMap<char, String> {
    let mut map: BTreeMap<char, String> = path_map();
    map.extend(from_chars(&[
        '/', ':', ';', '=', '@', '[', '\\', ']', '^', '|',
    ]));
    map
}

pub(crate) fn path_map() -> BTreeMap<char, String> {
    let mut map: BTreeMap<char, String> = query_map();
    map.extend(from_chars(&['?', '`', '{', '}']));
    map
}

pub(crate) fn query_map() -> BTreeMap<char, String> {
    let mut map: BTreeMap<char, String> = code_point_map();
    map.extend(from_chars(&[' ', '"', '<', '>', '#']));
    map
}

pub(crate) fn fragment_map() -> BTreeMap<char, String> {
    let mut map: BTreeMap<char, String> = code_point_map();
    map.extend(from_chars(&[' ', '"', '<', '>', '`']));
    map
}

pub(crate) fn pct_encode(s: &str, replacements: BTreeMap<char, String>, for_uri: bool) -> String {
    s.chars()
        .map(|c| {
            if replacements.contains_key(&c) {
                replacements.get(&c).unwrap().to_string()
            } else if for_uri && !c.is_ascii() {
                let mut buf = [0; 6];
                let bytes = c.encode_utf8(&mut buf);
                bytes
                    .as_bytes()
                    .iter()
                    .map(|b| format!("%{:X}", b))
                    .collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
