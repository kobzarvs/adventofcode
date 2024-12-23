use day_23::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() {
    let input = include_str!("../datasets/release.txt");
    let graph = parser::parse(input).unwrap();
    part_1::solve(&graph);
}

#[divan::bench]
fn part_2() {
    let input = include_str!("../datasets/release.txt");
    let graph = parser::parse(input).unwrap();
    part_2::solve(&graph);
}
