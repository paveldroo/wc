use std::{env, error::Error};

pub struct Config {
    pub filename: Option<String>,
    pub bytes: bool,
    pub lines: bool,
}

pub fn parse_args() -> Result<Config, Box<dyn Error>> {
    let raw_args: Vec<String> = env::args().collect();
    let mut cfg = Config {
        filename: None,
        bytes: false,
        lines: false,
    };

    for (idx, arg) in raw_args.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        match arg.as_str() {
            "-c" => cfg.bytes = true,
            "-l" => cfg.lines = true,
            other => {
                if other.chars().next().unwrap().to_string() == "-" {
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

    if cfg.filename.is_none() {
        return Err("missing filename as argument".into());
    }

    Ok(cfg)
}
