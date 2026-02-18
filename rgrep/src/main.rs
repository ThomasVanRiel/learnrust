use std::env;
use std::fs;
use std::process;

enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}

struct Config {
    query: String,
    filename: String,
    mode: SearchMode,
    line_numbers: bool,
}

impl Config{ 
    // Equivalent to C++ static method
    fn new(args: &Vec<String>) -> Result<Config, String> {
        let has_i_flag = args.iter().any(|a| a == "-i");
        let has_n_flag = args.iter().any(|a| a == "-n");

        let non_flags: Vec<&String> = args.iter() // Create iterator over vec
            .skip(1)                              // Skip program name 
            .filter(|a| !a.starts_with("-"))      // Keep elements that don't start with '-'
            .collect();                           // Build collection (Vec<String>) from iterator

        match (non_flags.get(0), non_flags.get(1)) {
            (Some(q), Some(f)) => Ok(Config {
                query: q.to_string(),
                filename: f.to_string(),
                mode: if has_i_flag {
                    SearchMode::CaseInsensitive
                } else {
                    SearchMode::CaseSensitive
                },
                line_numbers: has_n_flag,
            }),
            _ => Err(String::from("Usage: rgrep [-i] <query> <filename>"))
        }
    } 

    // Equivalent to C++ method
    fn search(&self, contents: &str) -> Vec<String> {
        let mut matches: Vec<String> = Vec::new();
        for (index, line) in contents.lines().enumerate() {
            let is_match = match &self.mode {
                SearchMode::CaseSensitive => line.contains(self.query.as_str()),
                SearchMode::CaseInsensitive => line.to_lowercase().contains(&self.query.to_lowercase()),
            };

            if is_match {
                let prefix = if self.line_numbers {
                    format!("{}:", index + 1)
                } else {
                    String::new()
                };
                matches.push(format!("{prefix}{line}"));
            }
        }
        return matches;
    }
}

fn run(config: &Config) -> Result<(), String> {
    let contents = fs::read_to_string(&config.filename)
        .map_err(|e| e.to_string())?;

    // Search query in file
    for line in config.search(&contents) {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    // Get args
    let args: Vec<String> = env::args().collect();

    // Parse config from args
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    if let Err(error) = run(&config) {
        eprintln!("{}", error);
        process::exit(1);
    }

    
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let config = Config {
            query: String::from("hello"),
            filename: String::from("test.txt"),
            mode: SearchMode::CaseSensitive,
            line_numbers: false,
        };

        let contents = "hello world\nGoodbye world\nhello again";
        let results = config.search(contents);

        assert_eq!(results, vec!["hello world", "hello again"]);
    }

    #[test]
    fn case_insensitive_search() {
        let config = Config {
            query: String::from("hello"),
            filename: String::from("test.txt"),
            mode: SearchMode::CaseInsensitive,
            line_numbers: false,
        };

        let contents = "Hello world\nGoodbye world\nhello again";
        let results = config.search(contents);

        assert_eq!(results, vec!["Hello world", "hello again"]);
    }
}
