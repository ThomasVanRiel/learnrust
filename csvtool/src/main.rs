use serde::Deserialize;
use std::{env, iter::Filter, ops::Deref};

#[derive(Debug)]
enum FilterOp {
    Eq,
    Ne,
    Gt,
    St,
    Ge,
    Se,
}

impl FilterOp {
    fn compare<T: std::cmp::PartialOrd>(&self, rhs: T, lhs: T) -> bool {
        match self {
            FilterOp::Eq => rhs == lhs,
            FilterOp::Ne => rhs != lhs,
            FilterOp::Gt => rhs > lhs,
            FilterOp::St => rhs < lhs,
            FilterOp::Ge => rhs >= lhs,
            FilterOp::Se => rhs <= lhs,
        }
    }
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
    filter: Option<(String, FilterOp, String)>,
}

impl Config {
    fn build_filter(filter_string: &String) -> Option<(String, FilterOp, String)> {
        let ops = [
            ("==", FilterOp::Eq),
            ("!=", FilterOp::Ne),
            (">=", FilterOp::Ge),
            ("<=", FilterOp::Se),
            (">", FilterOp::Gt),
            ("<", FilterOp::St),
        ];
        for (op_str, op_obj) in ops {
            if let Some(op_pos) = filter_string.find(op_str) {
                // Check if two sides are present around the operator
                if filter_string.len() > op_pos + op_str.len() {
                    return Some((
                        filter_string[..op_pos].to_string().to_lowercase(),
                        op_obj,
                        filter_string[op_pos + op_str.len()..]
                            .to_string()
                            .to_lowercase(),
                    ));
                } else {
                    println!("Filter syntax is col{op_str}query, not {filter_string}");
                    return None;
                }
            }
        }

        // If no filter string was found
        println!("Filter syntax is col<Op>query, not {filter_string}");
        None
    }

    fn new(args: &[String]) -> Result<Config, String> {
        let filename = args.get(1);
        let filter = if let Some(pos) = args.iter().position(|a| a.eq("--filter")) {
            if let Some(filter_string) = args.get(pos + 1) {
                Config::build_filter(filter_string)
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

        if let Some((filter, op, query)) = &config.filter {
            match filter.as_str() {
                "name" => {
                    if op.compare(record.name.to_lowercase(), query.to_lowercase()) {
                        record.print();
                    }
                }
                "age" => match query.parse::<u32>() {
                    Ok(age_query) => {
                        if op.compare(record.age, age_query) {
                            record.print();
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {} while parsing string \"{}\" to u32", e, query);
                        std::process::exit(1);
                    }
                },
                "city" => {
                    if op.compare(record.city.to_lowercase(), query.to_lowercase()) {
                        record.print();
                    }
                }
                "salary" => match query.parse::<u32>() {
                    Ok(salary_query) => {
                        if op.compare(record.salary, salary_query) {
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
