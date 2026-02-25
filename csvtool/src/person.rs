
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
    pub salary: u32,
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
