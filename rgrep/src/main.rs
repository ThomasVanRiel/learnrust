use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &Vec<String>) -> Result<Config, String> {
    match (args.get(1), args.get(2)) {
        (Some(q), Some(f)) => Ok(Config {
            query: q.to_string(),
            filename: f.to_string(),
        }),
        _ => Err(String::from("Usage: rgrep <query> <filename>"))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match parse_config(&args) {
        Ok(config) => config,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let contents = match fs::read_to_string(&config.filename) {
        Ok(text) => text,
        Err(error) => {
            println!("Error reading file '{}': {}", &config.filename, error);
            return;
        }
    };

    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("{line}");
        }
    }
}
