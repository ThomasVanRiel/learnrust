// 4.1
fn count_matches(body: &str, pattern: &str) -> usize {
    let mut matches: usize = 0;
    for line in body.split("\n") {
        if Some(line.find(pattern)) {
            matches += 1;
        }
    }
    matches
}
// Solution
fn count_matches(body: &str, pattern: &str) -> usize {
    let mut matches: usize = 0;
    for line in body.lines() {
        if line.contains(pattern) {
            matches += 1;
        }
    }
    matches
}
// Or using iterators
fn count_matches(body: &str, pattern: &str) -> usize {
    body.lines().filter(|line| line.contains(pattern)).count()
}

// 4.2
fn first_even(list: &[i32]) -> Option<i32> {
    for num in list {
        if num % 2 == 0 {
            return Some(*num);
        }
    }
    None
}

// 4.3
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Self::Circle(r) => r * r * 3.14159,
            Self::Rectangle(w, h) => w * h,
        }
    }
}

// 4.4
fn parse_number(num: &str) -> Result<i32, String> {
    num.parse::<i32>()
        .map_err(|e| writeln!("Could not parse string due to error: \"{}\""))
}

fn parse_number(num: &str) -> Result<i32, String> {
    match num.parse::<i32>() {
        Ok(n) => n,
        Err(e) => writeln!("Could not parse string due to error: \"{}\""),
    }
}

// 4.5
fn dedup(list: Vec<String>) -> Vec<String> {
    let mut deduped = Vec::new();
    for item in list {
        if deduped.contains(item) 
    }
}
