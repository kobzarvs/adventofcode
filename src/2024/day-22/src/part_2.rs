use crate::shared;
use crate::shared::LIMIT;
use ahash::{AHashMap, AHashSet};
use rayon::prelude::*;
use smallvec::SmallVec;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub fn solve(numbers: &[i64]) -> (i64, Duration) {
    let time = Instant::now();
    let sums = Mutex::new(AHashMap::new());

    numbers.par_iter().for_each(|&n| {
        let secrets = shared::get_secrets(n);
        let prices: SmallVec<[i64; LIMIT]> = secrets.iter().map(|&v| v % 10).collect();
        let mut local_sums = AHashMap::new();
        let mut seen = AHashSet::new();

        for i in 0..prices.len() - 4 {
            let delta_sequence = (
                prices[i + 1] - prices[i],
                prices[i + 2] - prices[i + 1],
                prices[i + 3] - prices[i + 2],
                prices[i + 4] - prices[i + 3],
            );
            if seen.insert(delta_sequence) {
                local_sums
                    .entry(delta_sequence)
                    .and_modify(|delta_price| *delta_price += prices[i + 4])
                    .or_insert(prices[i + 4]);
            }
        }

        let mut global_sums = sums.lock().unwrap();
        for (seq_key, seq_price_sum) in local_sums {
            global_sums
                .entry(seq_key)
                .and_modify(|global_sum| *global_sum += seq_price_sum)
                .or_insert(seq_price_sum);
        }
    });

    let final_sums = sums.lock().unwrap();
    (*final_sums.values().max().unwrap(), time.elapsed())
}

#[cfg(test)]
mod tests {
    use crate::{parser, part_2};

    #[test]
    fn part_2() {
        let input = include_str!("../datasets/test_2.txt");
        let data = parser::parse(input).unwrap();
        assert_eq!(part_2::solve(&data).0, 23);
    }
}
