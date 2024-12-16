#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use std::collections::HashMap;
use geo::{Coord};
use crate::{parse, Robot};


pub fn run(input: &str) -> u64 {
    let (mut map, directions, robot_pos) = parse(input);
    solve(&mut map, &directions, robot_pos)
}

fn solve(map: &mut HashMap<Coord<i32>, String>, directions: &[String], pos: Coord<i32>) -> u64 {
    let mut robot = Robot::new(map, pos, directions);
    robot.run();
    robot.calculate()
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn solve_small_example() {
        let input = include_str!("../test.txt");
        let (mut map, directions, robot_pos) = parse(input);

        assert_eq!(700, directions.len());
        assert_eq!(2028, part1::solve(&mut map, &directions, robot_pos));
    }

    #[test]
    #[ignore]
    fn solve_big_example() {
        let input = include_str!("../test_big.txt");
        let (mut map, directions, robot_pos) = parse(input);

        assert_eq!(700, directions.len());
        assert_eq!(10092, part1::solve(&mut map, &directions, robot_pos));
    }
}
