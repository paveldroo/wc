use std::{env, error::Error};

pub fn parse_filename() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut filename = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" => {
                let value = args.get(i + 1).ok_or("missing value for -c")?;
                filename = Some(value.clone());
                i += 2;
            }
            unknown => return Err(format!("unknown argument `{unknown}`").into()),
        }
    }

    filename.ok_or("missing required argument -c".into())
}
