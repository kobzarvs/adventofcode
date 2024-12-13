use crate::{parse, Machine};

pub fn run(input: &str) -> usize {
    solve(&parse(&input))
}

fn solve(machines: &Vec<Machine>) -> usize {
    42
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(480, part1::solve(&data));
    }
}
