use {{crate_name}}::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::run();
}

#[divan::bench]
fn part2() {
    part2::run();
}
