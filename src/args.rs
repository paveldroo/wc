use std::env;

pub fn parse_filename() -> String {
    let args: Vec<String> = env::args().collect();
    let mut filename = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" => {
                if i + 1 < args.len() {
                    filename = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for -c");
                    std::process::exit(1)
                }
            }
            unknown => {
                eprintln!("Error: Unknown argument '{}'", unknown);
                std::process::exit(1)

            }
        }
    }

    match filename {
        Some(n) => n,
        None => {
            eprintln!("Error: Missing required argument -c");
            std::process::exit(1)
        }
    }
}
