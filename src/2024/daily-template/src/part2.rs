use crate::{parse};

pub fn run(input: &str) -> todo!("type") {
    solve(&parse(&input))
}

fn solve(data: todo!("model")) -> todo!("type") {
    todo!("result")
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(todo!("expect"), part2::solve(&data));
    }
}
