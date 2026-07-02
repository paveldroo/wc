mod args;
mod test_file;

fn main() {
    let filename = args::parse_filename();
    let content = test_file::get_test_data(filename);
    dbg!(content);
}
