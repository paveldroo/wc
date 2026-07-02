use std::{fs, path::PathBuf};

pub fn get_test_data(filename: String) -> String {
    fs::read_to_string(get_fixture_path(filename)).expect("Failed to read tests.txt")
}

fn get_fixture_path(filename: String) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(filename);
    path
}
