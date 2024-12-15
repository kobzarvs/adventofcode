use day_14::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_product_by_quadrant() {
    let input = include_str!("../data.txt");
    part1::run(input);
}

#[divan::bench]
fn part2_find_christmas_tree() {
    let input = include_str!("../data.txt");
    part2::run(input);
}
