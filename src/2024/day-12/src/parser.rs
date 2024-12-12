use std::collections::HashMap;
use crate::part2::Complex64;

pub fn parse_part1(input: &str) -> Vec<Vec<char>> {
    let mut result = vec![];
    input.lines().for_each(|line| {
        let mut row: Vec<char> = vec![];
        line.chars().for_each(|c| {
            row.push(c);
        });
        result.push(row);
    });

    result
}

pub fn parse_part2(input: &str) -> HashMap<Complex64, char> {
    let mut complex_map: HashMap<Complex64, char> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            complex_map.insert(Complex64::new(x as i32, y as i32), c);
        });
    });
    complex_map
}
