use std::num::ParseIntError;

pub fn parse(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.lines().map(|l| Ok(l.parse()?)).collect()
}