use crate::{parse, Computer};
use std::collections::VecDeque;

pub fn run(input: &str) -> i64 {
    solve(&mut parse(input))
}

pub fn run_2(input: &str) -> i64 {
    solve_2(&mut parse(input))
}

fn solve(computer: &mut Computer) -> i64 {
    computer.raw_program.reverse();
    let out: &Vec<i64> = &computer.raw_program;

    let mut b: i64;
    let mut c: i64;
    let mut q: VecDeque<(i64, usize)> = VecDeque::new();
    q.push_front((0, 0));

    while let Some((a, step)) = q.pop_front() {
        let mut tmp_a: i64;
        for i in 0..8 {
            tmp_a = a | i;
            b = tmp_a & 7;
            b = b ^ 0b101;
            c = tmp_a >> b;
            b = b ^ c;
            b = b ^ 0b110;
            let out_b = b % 8;
            if out_b == out[step] {
                if step == out.len() - 1 {
                    return tmp_a;
                }
                q.push_back((tmp_a << 3, step + 1));
            }
        }
    }
    0
}

fn solve_2(computer: &mut Computer) -> i64 {
    computer.raw_program.reverse();
    let out: &Vec<i64> = &computer.raw_program;

    let mut q: VecDeque<(i64, usize)> = VecDeque::new();
    q.push_front((0, 0));

    let mut tmp = computer.clone();
    let len = tmp.program.len() as i64;

    while let Some((a, step)) = q.pop_front() {
        for i in 0..8 {
            tmp.a = a | i;
            tmp.ip = 0;
            tmp.run(len - 1);

            if let Some(output) = tmp.last_output {
                if output != out[step] {
                    continue;
                }
                if step == out.len() - 1 {
                    return a | i;
                }
                q.push_back(((a | i) << 3, step + 1));
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2};

    #[test]
    fn test_data_1() {
        let input = include_str!("../test.txt");
        let mut data = parse(input);

        assert_eq!(29328, part2::solve(&mut data));
    }

    #[test]
    fn test_data_2() {
        let input = include_str!("../test2.txt");
        let mut data = parse(input);

        assert_eq!(117440, part2::solve(&mut data));
    }

    #[test]
    fn test_data_1_solve_2() {
        let input = include_str!("../test.txt");
        let mut data = parse(input);

        assert_eq!(29328, part2::solve_2(&mut data));
    }

    #[test]
    fn test_data_2_solve_2() {
        let input = include_str!("../test2.txt");
        let mut data = parse(input);

        assert_eq!(117440, part2::solve_2(&mut data));
    }
}
