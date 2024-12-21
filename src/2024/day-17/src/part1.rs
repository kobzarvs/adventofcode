use crate::{parse, Computer};
use itertools::Itertools;

pub fn run(input: &str) -> String {
    solve(parse(input))
}

fn solve(computer: Computer) -> String {
    computer.clone().run(-1).into_iter().map(|x| x.to_string()).join(",")
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1::solve(data));
    }
}
