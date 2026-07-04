use std::error::Error;

mod args;
mod input;

fn main() {
    if let Err(e) = run() {
        eprintln!("ccwc: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let cfg = args::parse_args()?;
    let filename = cfg.filename;
    let content = input::read_input(&filename)?;

    if cfg.bytes {
        let bytes_count = content.len();
        println!("  {bytes_count} {filename}");
    } else if cfg.lines {
        let lines_count = content.lines().count();
        println!("  {lines_count} {filename}");
    } else if cfg.words {
        let words_count = content.split_whitespace().count();
        println!("  {words_count} {filename}");
    } else if cfg.chars {
        let chars_count = content.chars().count();
        println!("  {chars_count} {filename}");
    }

    Ok(())
}
