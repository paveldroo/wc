mod args;
mod test_file;

fn main() {
    let filename = args::parse_filename();
    let content = test_file::get_test_data(filename.clone());

    let bytes_count = content.len();
    println!("  {bytes_count:?} {filename}");
    // dbg!(content);
}
