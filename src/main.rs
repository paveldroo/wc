use std::{
    error::Error, io::{self, IsTerminal, Read},
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
    let cfg = args::parse_args()?;
    let content = if cfg.filename.is_empty() {
        if io::stdin().is_terminal() {
            return Err("missing filename as argument".into());
        }
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        if buffer.is_empty() {
            return Err("no stdin data and no filename was provided".into());
        }
        buffer.to_string()
    } else {
        input::read_input(&cfg.filename)?
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
