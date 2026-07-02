mod args;

fn main() {
    let filename = args::parse_filename();
    dbg!(filename);
}
