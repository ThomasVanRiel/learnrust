use serde::Deserialize;
use std::env;

enum FilterType {
    Eq,
    Gt,
    St,
    Ge,
    Se,
}

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
    salary: u32,
}

impl Person {
    fn print(&self) {
        println!(
            "{:<20} {:>4} {:<12} {:>8}",
            self.name, self.age, self.city, self.salary
        );
    }
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
                    Some((
                        parts[0].to_string().to_lowercase(),
                        parts[1].to_string().to_lowercase(),
                    ))
                } else {
                    println!("Filter syntax is col=query, not {filter_string}");
                    None
                }
            } else {
                println!("--filter flag specified but no filter provided.");
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

    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    let mut reader = match csv::Reader::from_path(config.filename) {
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

        if let Some((filter, query)) = &config.filter {
            match filter.as_str() {
                "name" => {
                    if record.name.to_lowercase() == query.to_lowercase() {
                        record.print();
                    }
                }
                "age" => match query.parse::<u32>() {
                    Ok(age_query) => {
                        if record.age == age_query {
                            record.print();
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {} while parsing string \"{}\" to u32", e, query);
                        std::process::exit(1);
                    }
                },
                "city" => {
                    if record.city.to_lowercase() == query.to_lowercase() {
                        record.print();
                    }
                }
                "salary" => match query.parse::<u32>() {
                    Ok(salary_query) => {
                        if record.salary == salary_query {
                            record.print();
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {} while parsing string \"{}\" to u32", e, query);
                        std::process::exit(1);
                    }
                },
                _ => {
                    eprintln!("Filter target not in record headings!")
                }
            }
        } else {
            record.print();
        }
    }
}
