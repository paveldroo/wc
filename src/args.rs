use std::{env, error::Error};

pub struct Config {
    pub filename: String,
    pub bytes: bool,
    pub lines: bool,
    pub words: bool,
}

pub fn parse_args() -> Result<Config, Box<dyn Error>> {
    let raw_args: Vec<String> = env::args().collect();

    let mut filename: Option<String> = None;
    let mut bytes = false;
    let mut lines = false;
    let mut words = false;

    for arg in raw_args.iter().skip(1) {
        match arg.as_str() {
            "-c" => bytes = true,
            "-l" => lines = true,
            "-w" => words = true,
            other => {
                if other.starts_with('-') || other.is_empty() {
                    return Err(format!("unknown argument `{other}`").into());
                }
                if filename.is_none() {
                    filename = Some(other.to_string());
                } else {
                    return Err(format!("unknown argument `{other}`").into());
                }
            }
        }
    }

    let filename = filename.ok_or("missing filename as argument")?;

    Ok(Config {
        filename,
        bytes,
        lines,
        words,
    })
}
