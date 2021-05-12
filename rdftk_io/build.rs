use std::fs::{read_to_string, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const PEST_EXTENSION: &str = "pest";

const COMMON_NAME: &str = "common";

const FILE_SUFFIX: &str = "-in";

const PEST_MODS: &[&str] = &["nq", "nt", "turtle"];

fn read_a_file(dir_path: &Path, file_name: &str) -> String {
    let file_path = dir_path.join(&format!("{}.{}", file_name, PEST_EXTENSION));
    let content = read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));
    println!("cargo:rerun-if-changed={:?}", file_path);
    content
}

fn main() {
    let src_path = PathBuf::from(MANIFEST_DIR).join("src");

    let common_rules = read_a_file(&src_path.join(COMMON_NAME), COMMON_NAME);

    for module in PEST_MODS {
        let file_path = src_path.join(module);
        let file_name = format!("{}{}", module, FILE_SUFFIX);
        let module_rules = read_a_file(&file_path, &file_name);

        let combined_path = file_path.join(&format!("{}.{}", module, PEST_EXTENSION));
        let write_file = File::create(&combined_path).unwrap();
        let mut writer = BufWriter::new(&write_file);
        writer
            .write_all(module_rules.as_ref())
            .unwrap_or_else(|_| panic!("Unable to write common rules to file {:?}", combined_path));
        writer.write_all(common_rules.as_ref()).unwrap_or_else(|_| {
            panic!(
                "Unable to write module-specific rules to file {:?}",
                combined_path
            )
        });
    }
}
