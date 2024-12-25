use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let mut puzzle: Vec<u64> = vec![];
    let mut piece: u64;
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        match line {
            "#####" | "....." => {
                piece = if line.starts_with("#") { 0b11111 } else { 0 };
                for subline in lines.by_ref() {
                    if subline.is_empty() {
                        break;
                    }
                    piece = subline.chars().fold(piece, |mut acc, c| {
                        acc <<= 1;
                        acc |= if c == '#' { 1 } else { 0 };
                        acc
                    });
                }
                puzzle.push(piece);
            }
            _ => {}
        }
    }

    let result = puzzle
        .iter()
        .combinations(2)
        .map(|p| p[0] & p[1])
        .filter(|&it| it == 0)
        .count();

    println!("Day 25: {result}");
}
