use std::collections::HashMap;
use crate::Pos;

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
