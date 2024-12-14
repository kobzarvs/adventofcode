use std::io;
use std::io::Write;
use crate::{parse_to_complex, Complex32, Machine};

pub fn run(input: &str) -> i64 {
    let machines = parse_to_complex(input);
    solve(&machines)
}

fn sub(a: Complex32, b: Complex32) -> Option<Complex32> {
    if a.re >= b.re && a.im >= b.im {
        Some(a - b)
    } else {
        None
    }
}

fn solve(machines: &Vec<Machine<Complex32>>) -> i64 {
    machines
        .iter()
        .flat_map(|machine| {
            let a = machine.a; // *1
            let b = machine.b; // *3
            let target = machine.target;
            let mut current: Complex32;
            let mut result: Vec<i64> = vec![];

            for i in 0..100_i64 {
                if let Some(result) = sub(target, b * i) {
                    current = result;
                } else {
                    break;
                }

                for j in 0..100_i64 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if let Some(r) = sub(current, a * j) {
                        if r != Complex32::default() {
                            continue;
                        }
                        io::stdout().flush().unwrap();
                        result.push(j*3 + i);
                        break;
                    } else {
                        break;
                    }
                }
            }
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{parse_to_complex, part1_complex_nums};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let machines = parse_to_complex(input);

        assert_eq!(480, part1_complex_nums::solve(&machines));
    }
}
