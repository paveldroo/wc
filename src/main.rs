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
    let filename = args::parse_args()?;
    let content = input::read_input(&filename)?;

    let bytes_count = content.len();
    println!("  {bytes_count} {filename}");
    Ok(())
}
