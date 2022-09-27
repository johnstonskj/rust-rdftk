use rdftk_iri::IRI;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
struct Test {
    id: String,
    name: String,

    #[serde(default)]
    ignored: bool,

    #[serde(default)]
    does_not_parse: bool,

    // config
    charset: Option<String>,
    // abs
    url: Option<String>,
    expect_url: Option<String>,
    // relative
    base: Option<String>,
    rel: Option<String>,
    expect_rel: Option<String>,
    scheme: Option<String>,
    expect_scheme: Option<String>,
    expect_host: Option<String>,
    expect_port: Option<String>,
    expect_path: Option<String>,
    expect_query: Option<String>,
    expect_fragment: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
struct Group {
    name: String,
    link: String,
    desc: String,
    test: Vec<Test>,
}

#[derive(Clone, Debug, Deserialize)]
struct Tests {
    desc: String,
    group: Vec<Group>,
}

#[derive(Clone, Debug, Deserialize)]
struct AllTests {
    tests: Tests,
}

const JSON_SRC: &str = include_str!("cweb_iri-tests_master_iris.json");

#[test]
#[ignore]
fn test_all_tests() {
    let tests: AllTests = serde_json::from_str(JSON_SRC).unwrap();
    let tests = tests.tests;

    println!("# Tests");
    println!();
    println!("*{}*", tests.desc);
    println!();

    test_groups(&tests.group)
}

fn test_groups(groups: &[Group]) {
    for group in groups {
        println!("## Group: {}", group.name);
        println!();
        println!("*{}*", group.desc);
        println!();

        test_tests(&group.test)
    }
}

fn test_tests(tests: &[Test]) {
    for test in tests {
        if test.ignored {
            println!("### Test: {} ({}) is ignored", test.name, test.id);
            println!();
        } else {
            println!("### Test: {} ({})", test.name, test.id);
            println!();

            if let (Some(_), Some(_)) = (&test.url, &test.expect_url) {
                test_simple_parse(test);
            } else if let (Some(_), Some(_), Some(_)) = (&test.base, &test.rel, &test.expect_rel) {
                test_base_rel_parse(test);
            }
        }
    }
}

fn test_simple_parse(test: &Test) {
    let test_url = test.url.as_ref().unwrap();
    println!("    let uri = {:?};", test_url);
    println!();
    if test.does_not_parse {
        assert!(IRI::from_str(&test_url).is_err());
    } else {
        let actual = IRI::from_str(&test_url).unwrap();
        let expected = IRI::from_str(&test.expect_url.as_ref().unwrap()).unwrap();
        assert_eq!(actual, expected);
    }
}

fn test_base_rel_parse(test: &Test) {
    let base_url = test.base.as_ref().unwrap();
    let rel_url = test.rel.as_ref().unwrap();
    println!("    let base = {:?};", base_url);
    println!("    let rel = {:?};", rel_url);
    println!();
    println!("*** 1");
    let base = IRI::from_str(&base_url).unwrap();
    println!("*** 2");
    let rel = IRI::from_str(&rel_url).unwrap();
    println!("*** 3");
    let actual = base.resolve(&rel).unwrap();
    println!("*** 4");
    let expected = IRI::from_str(&test.expect_rel.as_ref().unwrap()).unwrap();
    println!("*** 5");
    assert_eq!(actual, expected);
}
