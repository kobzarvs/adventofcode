use day_07::read_file;
use rayon::prelude::*;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file();

    let expressions = parse(&input)?;

    let part_1 = solve(&expressions, 2);
    println!("Part I: {:?}", part_1);

    let part_2 = solve(&expressions, 3);
    println!("Part II: {:?}", part_2);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<(u64, Vec<u64>)>, regex::Error> {
    let re = Regex::new(r"(?<result>\d+): (?<numbers>[\d\s]+$)")?;

    let mut expressions: Vec<(u64, Vec<u64>)> = Vec::new();

    for line in input.lines() {
        if re.is_match(line) {
            let expr = re.captures(line).unwrap();
            let result_str = expr.name("result").unwrap().as_str();
            let result: u64 = result_str.parse().unwrap();
            let numbers: Vec<u64> = expr
                .name("numbers")
                .unwrap()
                .as_str()
                .split(" ")
                .map(|it| it.parse::<u64>().unwrap())
                .collect();

            expressions.push((result, numbers));
        }
    }

    Ok(expressions)
}

fn solve_1(expressions: &Vec<(u64, Vec<u64>)>) -> u64 {
    expressions
        .par_iter()
        .map(|(expected, numbers)| {
            for counter in 0..2_u64.pow(numbers.len() as u32 - 1) {
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

fn solve_2(expressions: &Vec<(u64, Vec<u64>)>) -> u64 {
    expressions
        .par_iter()
        .map(|(expected, numbers)| {
            for counter in 0..4_u64.pow(numbers.len() as u32 - 1) {
                if counter % 3 > 2 {
                    continue;
                }
                let mut result = numbers[0];
                for i in 1..numbers.len() {
                    let bits = (counter & 0b11 << (i - 1)*2) >> (i - 1)*2;
                    match bits {
                        0b00 => result *= numbers[i],
                        0b01 => result += numbers[i],
                        0b10 => result = format!("{}{}", result, numbers[i]).parse::<u64>().unwrap(),
                        _ => continue
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

fn solve(expressions: &Vec<(u64, Vec<u64>)>, base: u64) -> u64 {
    expressions
        .par_iter() // Используем параллельный итератор
        .filter(|(expected, numbers)| {
            for ops in 0..base.pow(numbers.len() as u32 - 1) {
                let mut result = numbers[0];
                let mut tops = ops;
                for i in 1..numbers.len() {
                    match tops % base {
                        0 => result *= numbers[i],
                        1 => result += numbers[i],
                        _ => result = format!("{}{}", result, numbers[i]).parse::<u64>().unwrap(),
                    }
                    tops = tops / base;
                    if result > *expected {
                        return break;
                    }
                }
                if result == *expected {
                    return true;
                }
            }
            false
        })
        .map(|(result, _)| result)
        .sum()
}
