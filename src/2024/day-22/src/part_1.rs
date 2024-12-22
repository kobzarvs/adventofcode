use std::time::{Duration, Instant};
use rayon::prelude::*;
use crate::shared;

pub fn solve(numbers: &[i64]) -> (i64, Duration) {
    let time = Instant::now();
    let res = numbers
        .par_iter()
        .map(|n| shared::get_secrets(*n)[2000])
        .sum();
    
    (res, time.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::{parser, part_1};

    #[test]
    fn part_1() {
        let input = include_str!("../datasets/test_1.txt");
        let data = parser::parse(input).unwrap();
        assert_eq!(part_1::solve(&data).0, 37327623);
    }
}
