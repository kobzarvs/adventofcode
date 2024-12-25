use itertools::Itertools;
use std::fs;

fn main() {
    let result = fs::read_to_string("data.txt")
        .unwrap()
        .lines()
        .chunks(8)
        .into_iter()
        .map(|chunk| {
            chunk
                .take(7)
                .flat_map(|it| it.chars())
                .fold(0, |mut acc, c| {
                    acc <<= 1;
                    acc |= if c == '#' { 1 } else { 0 };
                    acc
                })
        })
        .collect::<Vec<u64>>()
        .iter()
        .combinations(2)
        .map(|p| p[0] & p[1])
        .filter(|&it| it == 0)
        .count();
    
    println!("Day 25: {result}");
}
