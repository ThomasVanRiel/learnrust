mod config;
mod filter;
mod person;

use std::collections::HashSet;
use std::env;

use config::Config;
use person::Person;

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

    // Read csv to vec
    let mut people: Vec<Person> = reader
        .deserialize()
        .collect::<Result<Vec<Person>, _>>()
        .map_err(|e| e.to_string())?;

    // Filter
    for (filter, op, query) in &config.filters {
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

    // Check if stats are requested
    if config.stats {
        // Print stats
        println!("Rows: {}", people.len());

        // Check if there are items to show stats
        if !people.is_empty() {
            let ages: Vec<u32> = people.iter().map(|person| person.age).collect();
            println!(
                "age     min: {:>10} max: {:>10} avg: {:>10.2}",
                ages.iter().min().unwrap(),
                ages.iter().max().unwrap(),
                (ages.iter().sum::<u32>() as f64) / (ages.len() as f64),
            );

            let salaries: Vec<u32> = people.iter().map(|person| person.salary).collect();
            println!(
                "salary  min: {:>10} max: {:>10} avg: {:>10.2}",
                salaries.iter().min().unwrap(),
                salaries.iter().max().unwrap(),
                (salaries.iter().sum::<u32>() as f64) / (salaries.len() as f64),
            );

            let mut names: Vec<String> = people
                .iter()
                .map(|a| a.name.to_string())
                .collect::<Vec<String>>();
            names.sort();
            names.dedup();
            println!("name    {} unique values", names.len());

            // Or using a HashSet
            let cities: HashSet<&str> = people.iter().map(|p| p.city.as_str()).collect();
            println!("city    {} unique values", cities.len());
        }
    } else {
        // Else print resulting list
        println!("{:<20} {:>4} {:<12} {:>8}", "NAME", "AGE", "CITY", "SALARY");
        println!("{}", "-".repeat(52));

        for person in people {
            println!("{}", person)
        }
    }

    Ok(())
}
