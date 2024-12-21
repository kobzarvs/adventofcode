use std::collections::HashSet;
use std::io;
use std::io::Read;
use crate::{parse, Byte};
use geo::{coord, Coord};


pub fn run(input: &str) -> i32 {
    solve(&parse(input))
}

#[cfg(not(debug_assertions))]
pub const SIZE: i32 = 71;
#[cfg(debug_assertions)]
pub const SIZE: i32 = 7;

fn next_step(
    pos: Coord<i32>,
    step: i32,
    bytes: &[Byte],
    directions: &[Coord<i32>],
    path: &mut Vec<Coord<i32>>,
    visited: &mut HashSet<Coord<i32>>,
) -> Option<Vec<Coord<i32>>> {
    visited.insert(pos);
    path.push(pos);

    println!("step: {step}, pos: {:?}, len: {}", pos, path.len());
    print_map(bytes, visited.clone().into_iter().collect::<Vec<Coord<i32>>>().as_ref(), step as usize);
    println!("press key to next step...");
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();

    for dir in directions {
        let new_pos = pos + *dir;

        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= crate::part1::SIZE || new_pos.y >= crate::part1::SIZE || visited.contains(&new_pos) {
            continue;
        }

        if new_pos == coord! {x: SIZE - 1, y: SIZE - 1} {
            path.push(new_pos);
            visited.insert(new_pos);

            print_map(bytes, visited.clone().into_iter().collect::<Vec<Coord<i32>>>().as_ref(), step as usize);
            println!("*** SHORTEST PATH FOUND *** {} {:?}", path.len(), path);
            println!("press key to next step...");
            let mut buffer = [0; 1];
            io::stdin().read_exact(&mut buffer).unwrap();
            path.pop();
            visited.remove(&new_pos);

            continue;
        }

        let wall = bytes.into_iter().find(|&&it| it.1 == new_pos);

        if wall.is_some() && step >= wall.unwrap().0 - 1 {
            println!("step: {step}, {:?}, wall: {:?}", new_pos, wall.unwrap());
            continue;
        }

        if let Some(result) = next_step(new_pos, step + 1, bytes, directions, path, visited) {
            return Some(result);
        } else {
            path.pop();
            visited.remove(&new_pos);
        }
    }
    visited.remove(&pos);
    None
}

fn solve(bytes: &[Byte]) -> i32 {
    let pos = coord! {x: 0, y: 0};
    let target = coord! {x: SIZE - 1, y: SIZE - 1};
    // let map = HashMap::<Coord<i32>, bool>::new();

    let directions = [
        coord! {x:1, y:0},
        coord! {x:0, y:1},
        coord! {x:-1, y:0},
        coord! {x:0, y:-1},
    ];

    let mut visited: HashSet<Coord<i32>> = HashSet::new();
    let mut path: Vec<Coord<i32>> = vec![coord! {x:0, y:0}];

    if let Some(path) = next_step(pos, 1, bytes, &directions, &mut path, &mut visited) {
        println!("*** path len: {:?}, path: {:?}", path.len(), path);
    } else {
        println!("path not found!")
    }
    // {
    //     print_map(bytes, path.len() - 1);
    //     println!("press key to next step...");
    //     let mut buffer = [0; 1];
    //     io::stdin().read_exact(&mut buffer).unwrap();
    // }

    // for step in 0..bytes.len() {
    //     print_map(&map, bytes, step);

    // let mut buffer = [0; 1];
    // io::stdin().read_exact(&mut buffer).unwrap();
    // }

    0
}

struct Path {
    pos: Coord<i32>,
}

fn print_map(bytes: &[Byte], steps: &Vec<Coord<i32>>, step: usize) {
    // let mut steps: Vec<Coord<i32>> = vec![
    //     coord! {x: 0, y: 0},
    //     coord! {x: 1, y: 0},
    //     coord! {x: 1, y: 1},
    //     coord! {x: 1, y: 2},
    //     coord! {x: 2, y: 2},
    //     coord! {x: 3, y: 2},
    //     coord! {x: 3, y: 1},
    //     coord! {x: 4, y: 1},
    //     coord! {x: 4, y: 0},
    //     coord! {x: 5, y: 0},
    //     coord! {x: 6, y: 0},
    //     coord! {x: 6, y: 1},
    //     coord! {x: 6, y: 2},
    //     coord! {x: 5, y: 2},
    //     coord! {x: 5, y: 3},
    //     coord! {x: 4, y: 3},
    //     coord! {x: 4, y: 4},
    //     coord! {x: 3, y: 4},
    //     coord! {x: 3, y: 5},
    //     coord! {x: 3, y: 6},
    //     coord! {x: 4, y: 6},
    //     coord! {x: 5, y: 6},
    //     coord! {x: 6, y: 6},
    // ];
    for y in 0..crate::part1::SIZE {
        for x in 0..crate::part1::SIZE {
            let is_road = steps.contains(&coord! {x: x, y: y});
            let wall = bytes.iter().find(|&pos| pos.1 == coord! {x: x, y: y});

            if wall.is_some() && wall.unwrap().clone().1 == coord! {x: x, y: y} {
                if is_road {
                    print!("[o ]");
                } else {
                    print!("[{:>2}]", wall.unwrap().0);
                }
            } else if is_road {
                print!(" o  ");
            } else {
                print!(" .  ");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(todo!("expect"), part2::solve(&data));
    }
}
