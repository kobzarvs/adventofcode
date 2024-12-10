use std::collections::HashMap;
use day_10::{part2, part1, read_file, Pos};
use code_timing_macros::time_snippet;

fn main() {
    let input = read_file();

    let (map, start_points) = parse(&input);

    let result = time_snippet!(part1::solve(&map, &start_points));
    println!("part I: {}", result); // 36

    let result = time_snippet!(part2::solve(&map, &start_points));
    println!("part II: {}", result); // 81
}

pub fn parse(input: &str) -> (HashMap<Pos, i32>, Vec<Pos>) {
    let mut map: HashMap<Pos, i32> = HashMap::new();
    let mut start_points: Vec<Pos> = vec![];

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let high = c.to_digit(10).unwrap() as i32;
            map.insert(
                Pos {
                    x: x as i32 + 1,
                    y: y as i32 + 1,
                },
                high,
            );
            if high == 0 {
                start_points.push(Pos {
                    x: x as i32 + 1,
                    y: y as i32 + 1,
                });
            }
        })
    });

    (map, start_points)
}
