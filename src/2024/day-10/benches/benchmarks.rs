use day_10::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data.txt");
    let (map, start_points) = day_10::parser::parse(&input);

    part1::solve(&map, &start_points);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data.txt");
    let (map, start_points) = day_10::parser::parse(&input);

    part2::solve(&map, &start_points);
}
