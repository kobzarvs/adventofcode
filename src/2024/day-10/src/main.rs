use day_10::{parse, part1, part2, read_file};

fn main() {
    let input = read_file();

    let (map, start_points) = parse(&input);

    let result = part1::solve(&map, &start_points);
    println!("part I: {}", result); // 36 - 638

    let result = part2::solve(&map, &start_points);
    println!("part II: {}", result); // 81 - 1289
}
