use day_07::read_file;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{Ordering, PartialEq};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file();

    let expressions = parse(&input)?;

    let part_1 = solve_1(&expressions);
    let part_2 = solve_2(&expressions);

    println!("Part I: {:?}", part_1);
    println!("Part II: {:?}", part_2);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<(i64, Vec<i64>)>, regex::Error> {
    let re = Regex::new(r"(?<result>\d+): (?<numbers>[\d\s]+$)")?;

    let mut expressions: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in input.lines() {
        if re.is_match(line) {
            let expr = re.captures(line).unwrap();
            let result_str = expr.name("result").unwrap().as_str();
            println!("result_str: {}", result_str);
            let result: i64 = result_str.parse().unwrap();
            let numbers: Vec<i64> = expr
                .name("numbers")
                .unwrap()
                .as_str()
                .split(" ")
                .map(|it| it.parse::<i64>().unwrap())
                .collect();

            expressions.push((result, numbers));
        }
    }
    println!("{:?}", expressions);

    Ok(expressions)
}

fn solve_1(expressions: &Vec<(i64, Vec<i64>)>) -> i64 {
    expressions
        .iter()
        .map(|(expected, numbers)| {
            for counter in 0..2_i64.pow(numbers.len() as u32 - 1) {
                let mut result = numbers[0];
                for i in 1..numbers.len() {
                    let bits = (counter & 1 << (i - 1)) >> (i - 1);
                    match bits {
                        0b00 => result *= numbers[i],
                        0b01 => result += numbers[i],
                        _ => continue,
                    }
                }
                if result == *expected {
                    return result;
                }
            }
            return 0;
        })
        .sum()
}

fn solve_2(expressions: &Vec<(i64, Vec<i64>)>) -> i64 {
    expressions
        .iter()
        .map(|(expected, numbers)| {
            for counter in 0..3_i64.pow(numbers.len() as u32 - 1) {
                if counter % 3 > 2 {
                    continue;
                }
                let mut result = numbers[0];
                for i in 1..numbers.len() {
                    let bits = (counter & 0b11 << (i - 1)*2) >> (i - 1)*2;
                    match bits {
                        0b00 => result *= numbers[i],
                        0b01 => result += numbers[i],
                        0b10 => result = format!("{}{}", result, numbers[i]).parse::<i64>().unwrap(),
                        _ => continue,
                    }
                }
                if result == *expected {
                    return result;
                }
            }
            return 0;
        })
        .sum()
}
