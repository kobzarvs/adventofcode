#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use std::collections::HashMap;
use std::io;
use geo::{Coord};
use crate::{parse_2};
use crate::robot2::Robot2;

pub fn run(input: &str) -> io::Result<u64> {
    let (map, directions, robot_pos) = parse_2(input);
    Ok(solve(&map, &directions, robot_pos)?)
}

fn solve(map: &HashMap<Coord<i32>, i32>, directions: &[String], pos: Coord<i32>) -> io::Result<u64> {
    let mut robot = Robot2::new(map, pos, directions);
    robot.run()?;
    Ok(robot.calculate())
}


// #[cfg(test)]
// mod tests {
//     use crate::{parse, part2};
// 
//     #[test]
//     fn solve_small_example() {
//         let input = include_str!("../test.txt");
//         let (map, directions) = parse(input);
// 
//         assert_eq!(2028, part2::solve(&map, &directions));
//     }
// 
//     #[test]
//     fn solve_big_example() {
//         let input = include_str!("../test_big.txt");
//         let (map, directions) = parse(input);
// 
//         assert_eq!(10092, part2::solve(&map, &directions));
//     }
// }
