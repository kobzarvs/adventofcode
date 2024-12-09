use day_09::{part1, part2, read_file};
use code_timing_macros::time_snippet;

fn main() {
    let input = read_file();

    let result = time_snippet!(part1::solve(&input));
    println!("part I: {}", result);

    let result = time_snippet!(part2::solve(&input));
    println!("part II: {}", result);
}
