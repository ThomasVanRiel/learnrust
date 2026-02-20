use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
    salary: u32,
}

struct Config {
    filename: String,
    filter: Option<(String, String)>,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        let filename = args.get(1);
        let filter = if let Some(pos) = args.iter().position(|a| a.eq("--filter")) {
            if let Some(filter_string) = args.get(pos + 1) {
                let parts: Vec<&str> = filter_string.split("=").take(2).collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        match (filename, filter) {
            (Some(filename), filter) => Ok(Config {
                filename: filename.to_string(),
                filter,
            }),
            _ => Err(String::from(
                "Usage: csvtool <file> [--filter heading=query]",
            )),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(f) => f,
        None => {
            eprintln!("Usage: csvtool <file>");
            std::process::exit(1);
        }
    };

    let mut reader = match csv::Reader::from_path(filename) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("{:<20} {:>4} {:<12} {:>8}", "NAME", "AGE", "CITY", "SALARY");
    println!("{}", "-".repeat(52));
    for result in reader.deserialize() {
        let record: Person = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };
        println!(
            "{:<20} {:>4} {:<12} {:>8}",
            record.name, record.age, record.city, record.salary
        );
    }
}
