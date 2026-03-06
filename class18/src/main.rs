use anyhow::{Context, Result};
use thiserror::Error;

fn read_number(path: &str) -> Result<i32, AppError> {
    let content = std::fs::read_to_string(path)?;
    let number = content.trim().parse::<i32>()?;
    Ok(number)
}

#[derive(Error, Debug)]
enum AppError {
    #[error("file not found: {0}")]
    NotFound(String),

    #[error("parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
fn main() {
    let result = read_number("file.txt");
    println!("{:?}", result);
}
