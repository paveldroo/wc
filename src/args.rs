use std::{env, error::Error};

pub enum Mode {
    Bytes,
    Lines,
}

pub struct Args {
    pub filename: Option<String>,
    pub mode: Option<Mode>,
}

pub fn parse_args() -> Result<Args, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut args_struct = Args{filename: None, mode: None};
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" => {
                let value = args.get(i + 1).ok_or("missing value for -c")?;
                args_struct.filename = Some(value.to_string());
                args_struct.mode = Some(Mode::Bytes);
                i += 2;
            }
            "-l" => {
                let value = args.get(i + 1).ok_or("missing value for -l")?;
                args_struct.filename = Some(value.to_string());
                args_struct.mode = Some(Mode::Lines);
                i += 2;
            }
            unknown => return Err(format!("unknown argument `{unknown}`").into()),
        }
    }

    if args_struct.filename == None {
        return Err("missing filename as argument".into())
    }

    Ok(args_struct)
}
