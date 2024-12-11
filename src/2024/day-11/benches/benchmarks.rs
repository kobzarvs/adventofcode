use day_11::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../test.txt");

    part1::run(input);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../test.txt");
    part2::run(input);
}
