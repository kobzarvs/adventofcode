pub fn run() {
    todo!("run parse & solve");
}

fn solve() -> i32 {
    todo!("add your code");
    42
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");

        todo!("modify this code");

        let data = parse(input);
        assert_eq!(42, part2::solve(&data));
    }
}
