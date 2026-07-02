mod args;
mod input;

fn main() {
    let filename = args::parse_filename();
    let content = input::get_test_data(filename.clone());

    let bytes_count = content.len();
    println!("  {bytes_count} {filename}");
    // dbg!(content);
}
