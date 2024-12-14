use day_13::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_line_intersect() {
    let input = include_str!("../data.txt");
    part2::run(input);
}

#[divan::bench]
fn part1_complex_nums() {
    let input = include_str!("../data.txt");
    part1_complex_nums::run(input);
}

#[divan::bench]
fn part2_kramor_equation() {
    let input = include_str!("../data.txt");
    part2::run(input);
}
