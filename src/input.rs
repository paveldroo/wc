use std::{fs, io, path};

pub fn read_input(filename: String) -> Result<String, io::Error> {
    fs::read_to_string(get_fixture_path(filename))
}

fn get_fixture_path(filename: String) -> path::PathBuf {
    let mut path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(filename);
    path
}
