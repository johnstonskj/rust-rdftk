use rdftk_core::model::literal::LanguageTag;
use std::str::FromStr;

// See: https://schneegans.de/lv/?tags=de
// Input: de
// Canonical: de
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: de – German
#[test]
fn bcp_47_validator_01() {
    let result = LanguageTag::from_str("de");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"de".to_string());
        assert!(tag.script().is_none());
        assert!(tag.region().is_none());
        assert_eq!(tag.variants().count(), 0);
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=de-CH
// Input: de-CH
// Canonical: de-CH
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: de – German
// Region subtag: CH – Switzerland
#[test]
fn bcp_47_validator_02() {
    let result = LanguageTag::from_str("de-CH");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"de".to_string());
        assert!(tag.script().is_none());
        assert_eq!(tag.region().unwrap(), &"CH".to_string());
        assert_eq!(tag.variants().count(), 0);
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=de-DE-1901
// Input: de-DE-1901
// Canonical: de-DE-1901
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: de – German
// Region subtag: DE – Germany
// Variant subtags: 1901 – Traditional German orthography
#[test]
fn bcp_47_validator_03() {
    let result = LanguageTag::from_str("de-DE-1901");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"de".to_string());
        assert!(tag.script().is_none());
        assert_eq!(tag.region().unwrap(), &"DE".to_string());
        assert_eq!(
            tag.variants().cloned().collect::<Vec<String>>(),
            vec!["1901"]
        );
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=es-419
// Input: es-419
// Canonical: es-419
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: es – Spanish; Castilian
// Region subtag: 419 – Latin America and the Caribbean
#[test]
fn bcp_47_validator_04() {
    let result = LanguageTag::from_str("es-419");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"es".to_string());
        assert!(tag.script().is_none());
        assert_eq!(tag.region().unwrap(), &"419".to_string());
        assert_eq!(tag.variants().count(), 0);
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=sl-IT-nedis
// Input: sl-IT-nedis
// Canonical: sl-IT-nedis
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: sl – Slovenian
// Region subtag: IT – Italy
// Variant subtags: nedis – Natisone dialect; Nadiza dialect
#[test]
fn bcp_47_validator_05() {
    let result = LanguageTag::from_str("sl-IT-nedis");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"sl".to_string());
        assert!(tag.script().is_none());
        assert_eq!(tag.region().unwrap(), &"IT".to_string());
        assert_eq!(
            tag.variants().cloned().collect::<Vec<String>>(),
            vec!["nedis".to_string()]
        );
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=en-US-boont
// Input: en-US-boont
// Canonical: en-US-boont
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: en – English
// Region subtag: US – United States
// Variant subtags: boont – Boontling
#[test]
fn bcp_47_validator_06() {
    let result = LanguageTag::from_str("en-US-boont");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"en".to_string());
        assert!(tag.script().is_none());
        assert_eq!(tag.region().unwrap(), &"US".to_string());
        assert_eq!(
            tag.variants().cloned().collect::<Vec<String>>(),
            vec!["boont".to_string()]
        );
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=mn-Cyrl-MN
// Input: mn-Cyrl-MN
// Canonical? mn-Cyrl-MN
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: mn – Mongolian
// Script subtag: Cyrl – Cyrillic
// Region subtag: MN – Mongolia
#[test]
fn bcp_47_validator_07() {
    let result = LanguageTag::from_str("mn-Cyrl-MN");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"mn".to_string());
        assert_eq!(tag.script().unwrap(), &"Cyrl".to_string());
        assert_eq!(tag.region().unwrap(), &"MN".to_string());
        assert_eq!(tag.variants().count(), 0);
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=x-fr-CH
// Input: x-fr-CH
// Canonical: x-fr-ch
// Input is canonical: False
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Private subtags: 'fr', 'CH'
// Messages: Consider to use the canonical form 'x-fr-ch' instead of 'x-fr-CH'.
#[test]
fn bcp_47_validator_08() {
    let result = LanguageTag::from_str("x-fr-CH");
    assert!(result.is_ok());
    if let Ok(LanguageTag::PrivateUse(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag, vec!["fr".to_string(), "CH".to_string()]);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=en-GB-boont-r-extended-sequence-x-private
// Input: en-GB-boont-r-extended-sequence-x-private
// Canonical: en-GB-boont-r-extended-sequence-x-private
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: en – English
// Region subtag: GB – United Kingdom
// Variant subtags: boont – Boontling
// Extension subtags: r, extended, sequence
// Private subtags: private
#[test]
fn bcp_47_validator_09() {
    let result = LanguageTag::from_str("en-GB-boont-r-extended-sequence-x-private");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"en".to_string());
        assert!(tag.script().is_none());
        assert_eq!(tag.region().unwrap(), &"GB".to_string());
        assert_eq!(
            tag.variants().cloned().collect::<Vec<String>>(),
            vec!["boont".to_string()]
        );
        assert_eq!(
            tag.extensions()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
            vec!["r-extended-sequence".to_string()]
        );
        assert_eq!(
            tag.private_use().cloned().collect::<Vec<String>>(),
            vec!["private".to_string()]
        );
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=sr-Cyrl
// Input: sr-Cyrl
// Canonical: sr-Cyrl
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: sr – Serbian
// Script subtag: Cyrl – Cyrillic
#[test]
fn bcp_47_validator_10() {
    let result = LanguageTag::from_str("sr-Cyrl");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"sr".to_string());
        assert_eq!(tag.script().unwrap(), &"Cyrl".to_string());
        assert!(tag.region().is_none());
        assert_eq!(tag.variants().count(), 0);
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=sr-Latn
// Input: sr-Latn
// Canonical: sr-Latn
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: sr – Serbian
// Script subtag: Latn – Latin
#[test]
fn bcp_47_validator_11() {
    let result = LanguageTag::from_str("sr-Latn");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"sr".to_string());
        assert_eq!(tag.script().unwrap(), &"Latn".to_string());
        assert!(tag.region().is_none());
        assert_eq!(tag.variants().count(), 0);
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=hy-Latn-IT-arevela
// Input: hy-Latn-IT-arevela
// Canonical: hy-Latn-IT-arevela
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: hy – Armenian
// Script subtag: Latn – Latin
// Region subtag: IT – Italy
// Variant subtags: arevela – Eastern Armenian
#[test]
fn bcp_47_validator_12() {
    let result = LanguageTag::from_str("hy-Latn-IT-arevela");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"hy".to_string());
        assert_eq!(tag.script().unwrap(), &"Latn".to_string());
        assert_eq!(tag.region().unwrap(), &"IT".to_string());
        assert_eq!(
            tag.variants().cloned().collect::<Vec<String>>(),
            vec!["arevela".to_string()]
        );
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

// See: https://schneegans.de/lv/?tags=zh-TW
// Input: zh-TW
// Canonical: zh-TW
// Input is canonical: True
// Well-formed: True
// Valid: True
// Registry date: 2021-05-11
// Primary language subtag: zh – Chinese
// Region subtag: TW – Taiwan
#[test]
fn bcp_47_validator_13() {
    let result = LanguageTag::from_str("zh-TW");
    assert!(result.is_ok());
    if let Ok(LanguageTag::Tag(tag)) = result {
        println!("{:?}", tag);
        assert_eq!(tag.language(), &"zh".to_string());
        assert!(tag.script().is_none());
        assert_eq!(tag.region().unwrap(), &"TW".to_string());
        assert_eq!(tag.variants().count(), 0);
        assert_eq!(tag.extensions().count(), 0);
        assert_eq!(tag.private_use().count(), 0);
    } else {
        panic!()
    }
}

#[test]
fn bcp_regular_grandfathered() {
    let values = &[
        "art-lojban",
        "cel-gaulish",
        "no-bok",
        "no-nyn",
        "zh-guoyu",
        "zh-hakka",
        "zh-min",
        "zh-min-nan",
        "zh-xiang",
    ];
    for value in values {
        let result = LanguageTag::from_str(value);
        assert!(result.is_ok());
        if let Ok(LanguageTag::Grandfathered(tag)) = result {
            println!("{:?}", tag);
            assert_eq!(tag, value.to_string());
        } else {
            panic!()
        }
    }
}

#[test]
fn bcp_irregular_grandfathered() {
    let values = &[
        "en-GB-oed",
        "i-ami",
        "i-bnn",
        "i-default",
        "i-enochian",
        "i-hak",
        "i-klingon",
        "i-lux",
        "i-mingo",
        "i-navajo",
        "i-pwn",
        "i-tao",
        "i-tay",
        "i-tsu",
        "sgn-BE-FR",
        "sgn-BE-NL",
        "sgn-CH-DE",
    ];
    for value in values {
        let result = LanguageTag::from_str(value);
        assert!(result.is_ok());
        if let Ok(LanguageTag::Grandfathered(tag)) = result {
            println!("{:?}", tag);
            assert_eq!(tag, value.to_string());
        } else {
            panic!()
        }
    }
}

#[test]
fn random_success() {
    let values = &["sl-IT-rozaj-biske-1994", "en-scotland-fonipa"];

    for value in values {
        print!("{} => ", value);
        let result = LanguageTag::from_str(value);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}

#[test]
fn spec_equality() {
    let tag_a = LanguageTag::from_str("mn-Cyrl-MN").unwrap();
    let tag_b = LanguageTag::from_str("MN-cYRL-mn").unwrap();
    let tag_c = LanguageTag::from_str("mN-cYrL-Mn").unwrap();

    assert_eq!(tag_a, tag_b);
    assert_eq!(tag_b, tag_c);
    assert_eq!(tag_a, tag_c);
}

#[test]
fn spec_duplicate_variants() {
    let result = LanguageTag::from_str("de-DE-1901-1901");
    println!("{:?}", result);
    assert!(result.is_err());
}

#[test]
fn some_equality() {
    let lhs = LanguageTag::from_str("en").unwrap();
    let rhs = LanguageTag::from_str("en").unwrap();

    assert_eq!(lhs, rhs);

    assert_eq!(Some(lhs), Some(rhs));
}
