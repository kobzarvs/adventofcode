use day_17::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data.txt");
    part1::run(input);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data.txt");
    part2::run(input);
}

#[divan::bench]
fn part2_2() {
    let input = include_str!("../data.txt");
    part2::run_2(input);
}
