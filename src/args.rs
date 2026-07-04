use std::{env, error::Error};

pub struct Config {
    pub filename: Option<String>,
    pub bytes: bool,
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
}

pub fn parse_args() -> Result<Config, Box<dyn Error>> {
    let raw_args: Vec<String> = env::args().collect();

    let mut cfg = Config {
        filename: None,
        bytes: false,
        lines: false,
        words: false,
        chars: false,
    };

    for arg in raw_args.iter().skip(1) {
        match arg.as_str() {
            "-c" => cfg.bytes = true,
            "-l" => cfg.lines = true,
            "-w" => cfg.words = true,
            "-m" => cfg.chars = true,
            other => {
                if other.starts_with('-') || other.is_empty() {
                    return Err(format!("unknown argument `{other}`").into());
                }
                if cfg.filename.is_none() {
                    cfg.filename = Some(other.to_string());
                } else {
                    return Err(format!("unknown argument `{other}`").into());
                }
            }
        }
    }

    Ok(cfg)
}
