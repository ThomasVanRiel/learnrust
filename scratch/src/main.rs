struct Config<'a> {
    query: &'a str,
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", longer("Short String", "Longer String"));
    let config;
    {
        let text = String::from("hello world");
        config = Config { query: &text };
        println!("{}", config.query);
    }
}
