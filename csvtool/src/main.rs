use serde::Deserialize;
use std::env;

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

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Format string from println! is "{:<20} {:>4} {:<12} {:>8}"
        write!(
            f,
            "{:<20} {:>4} {:<12} {:>8}",
            self.name, self.age, self.city, self.salary
        )
    }
}

struct Config {
    filename: String,
    filter: Option<(String, FilterOp, String)>,
    sort: Option<String>,
    limit: Option<usize>,
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

        // Check if filter flag is in args and get query
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

        // Check if sort flag is in args and get key
        let sort = if let Some(pos) = args.iter().position(|a| a.eq("--sort")) {
            if let Some(sort_key) = args.get(pos + 1) {
                Some(sort_key.to_string())
            } else {
                println!("--sort flag specified but no key provided.");
                None
            }
        } else {
            None
        };

        // Check if limit flag is in args and get key
        let limit = if let Some(pos) = args.iter().position(|a| a.eq("--limit")) {
            if let Some(limit_string) = args.get(pos + 1) {
                Some(limit_string.parse::<usize>().map_err(|e| {
                    format!("Error: {e} while parsing string \"{limit_string}\" to usize")
                })?)
            } else {
                println!("--limit flag specified but no amount specified.");
                None
            }
        } else {
            None
        };

        match (filename, filter, sort, limit) {
            (Some(filename), filter, sort, limit) => Ok(Config {
                filename: filename.to_string(),
                filter,
                sort,
                limit,
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

    if let Err(error) = run(&config) {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run(config: &Config) -> Result<(), String> {
    let mut reader = match csv::Reader::from_path(&config.filename) {
        Ok(r) => r,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    println!("{:<20} {:>4} {:<12} {:>8}", "NAME", "AGE", "CITY", "SALARY");
    println!("{}", "-".repeat(52));

    // Read csv to vec
    let mut people: Vec<Person> = reader
        .deserialize()
        .collect::<Result<Vec<Person>, _>>()
        .map_err(|e| e.to_string())?;

    // Filter
    if let Some((filter, op, query)) = &config.filter {
        let numeric_query = if filter == "age" || filter == "salary" {
            Some(
                query
                    .parse::<u32>()
                    .map_err(|e| format!("Error: {e} while parsing string \"{query}\" to u32"))?,
            )
        } else {
            None
        };

        people.retain(|record| match filter.as_str() {
            "name" => op.compare(record.name.to_lowercase(), query.to_lowercase()),
            "age" => op.compare(record.age, numeric_query.unwrap()),
            "city" => op.compare(record.city.to_lowercase(), query.to_lowercase()),
            "salary" => op.compare(record.salary, numeric_query.unwrap()),
            _ => false,
        });
    }

    // Sort
    if let Some(sort_key) = &config.sort {
        people.sort_by(|a, b| match sort_key.as_str() {
            "name" => a.name.cmp(&b.name),
            "age" => a.age.cmp(&b.age),
            "city" => a.city.cmp(&b.city),
            "salary" => a.salary.cmp(&b.salary),
            _ => std::cmp::Ordering::Equal,
        });
    }

    // Limit
    if let Some(limit) = config.limit {
        people.truncate(limit);
    }

    // Print resulting list
    for person in people {
        println!("{}", person)
    }

    Ok(())
}
