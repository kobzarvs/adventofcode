use {{crate_name}}::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data.txt");
    part_1::solve(input);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data.txt");
    part_2::solve(input);
}
