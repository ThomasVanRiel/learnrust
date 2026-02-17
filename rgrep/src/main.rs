use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        if line.contains(query.as_str()) {
            println!("{line}");
        }
    }
}
