use std::{
    error::Error,
    io::{self, Read},
};

mod args;
mod input;

fn main() {
    if let Err(e) = run() {
        eprintln!("ccwc: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    let cfg = args::parse_args(&buffer)?;
    let mut content = String::new();
    if !cfg.filename.is_empty() {
        content = input::read_input(&cfg.filename)?;
    } else if !cfg.stdin.is_empty() {
        content = cfg.stdin;
    };

    let filename_str = format!(" {}", cfg.filename);

    if cfg.bytes {
        let bytes_count = content.len();
        println!("  {bytes_count}{filename_str}");
    } else if cfg.lines {
        let lines_count = content.lines().count();
        println!("  {lines_count}{filename_str}");
    } else if cfg.words {
        let words_count = content.split_whitespace().count();
        println!("  {words_count}{filename_str}");
    } else if cfg.chars {
        let chars_count = content.chars().count();
        println!("  {chars_count}{filename_str}");
    } else {
        let lines_count = content.lines().count();
        let words_count = content.split_whitespace().count();
        let bytes_count = content.len();
        println!("  {lines_count} {words_count} {bytes_count}{filename_str}");
    }

    Ok(())
}
