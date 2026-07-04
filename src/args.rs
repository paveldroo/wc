use std::{env, error::Error};

pub struct Config {
    pub filename: String,
    pub bytes: bool,
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
    pub stdin: String,
}

pub fn parse_args(stdin_buffer: &String) -> Result<Config, Box<dyn Error>> {
    let raw_args: Vec<String> = env::args().collect();

    let mut filename: Option<String> = None;
    let mut bytes = false;
    let mut lines = false;
    let mut words = false;
    let mut chars = false;

    for arg in raw_args.iter().skip(1) {
        match arg.as_str() {
            "-c" => bytes = true,
            "-l" => lines = true,
            "-w" => words = true,
            "-m" => chars = true,
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

    if filename.is_none() && stdin_buffer.is_empty() {
        return Err("missing filename as argument".into());
    };

    Ok(Config {
        filename: filename.unwrap_or_default(),
        bytes,
        lines,
        words,
        chars,
        stdin: stdin_buffer.to_string(),
    })
}
