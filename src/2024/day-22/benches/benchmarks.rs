use day_22::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    let input = include_str!("../datasets/release.txt");
    let data = parser::parse(input).unwrap();
    part_1::solve(&data);
}

#[divan::bench]
fn part_2() {
    let input = include_str!("../datasets/release.txt");
    let data = parser::parse(input).unwrap();
    part_2::solve(&data);
}
