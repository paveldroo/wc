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
    let args_struct = args::parse_args()?;
    let filename = args_struct.filename.unwrap();
    let content = input::read_input(&filename)?;

    match args_struct.mode {
        Some(args::Mode::Bytes) => {
            let bytes_count = content.len();
            println!("  {bytes_count} {filename}");
        }
        Some(args::Mode::Lines) => {
            let lines_count = content.lines().count();
            println!("  {lines_count} {filename}")
        },
        _ => {}
    }

    Ok(())
}
