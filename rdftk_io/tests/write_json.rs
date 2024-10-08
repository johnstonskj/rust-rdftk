#![cfg(feature = "json")]

use objio::{HasOptions, ObjectWriter};
use rdftk_io::json::{JsonOptions, JsonWriter};

mod common;

#[test]
fn write_json_plain() {
    let graph = common::tony_benn_graph(common::TonyBennType::default());

    let writer = JsonWriter::default();

    let result = writer.write_to_string(&graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: JSON (plain)\n{}", output);

    assert!(output.starts_with("{\"<http://en.wikipedia.org/wiki/Tony_Benn>\":{\"http://purl.org/dc/elements/1.1/description\""));
    assert!(output.ends_with("{\"type\":\"literal\",\"value\":\"Tony Benn\"}]}}"));
}

#[test]
fn write_json_pretty() {
    let graph = common::tony_benn_graph(common::TonyBennType::default());

    let writer = JsonWriter::default().with_options(JsonOptions::default().with_pretty_print(true));

    let result = writer.write_to_string(&graph);
    assert!(result.is_ok());
    let output = result.unwrap();
    println!("# format: JSON (pretty)\n{}", output);

    assert!(output.starts_with(
        r##"{
  "<http://en.wikipedia.org/wiki/Tony_Benn>": {"##
    ));
    assert!(output.ends_with(
        r##"      }
    ]
  }
}"##
    ));
}
