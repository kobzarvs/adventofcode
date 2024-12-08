use std::cmp::max;
use std::collections::{HashMap, HashSet};
use day_08::read_file;
use itertools::{Itertools, MultiProduct};
use rayon::prelude::*;
use regex::Regex;
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
enum Item {
    Antenna(String),
    AntiNode,
    Empty,
}

type Pos = (i32, i32);
type Matrix = HashMap<Pos, Item>;
type Antenna = HashSet<(crate::Item, Pos)>;

#[derive(Debug)]
struct Size {
    width: i32,
    height: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file();

    let (matrix, antenna, size) = parse(&input);

    let part_1 = solve_1(&matrix, &antenna, &size);
    println!("Part I: {:?}", part_1);

    let part_2 = solve_2(&matrix, &antenna, &size);
    println!("Part II: {:?}", part_2);

    Ok(())
}

fn solve_1(matrix: &Matrix, antenna_map: &HashMap<String, HashSet<(Item, Pos)>>, size: &Size) -> u64 {
    let tmp = antenna_map
        .par_iter()
        .flat_map(|(name, radars)| {
            let antiradars = radars
                .into_iter()
                .combinations(2)
                .flat_map(|pair| {
                    let vx = pair[1].1.0 - pair[0].1.0;
                    let vy = pair[1].1.1 - pair[0].1.1;
                    [(pair[0].1.0 - vx, pair[0].1.1 - vy), (pair[1].1.0 + vx, pair[1].1.1 + vy)]
                })
                .filter(|pos| pos.0 > 0 && pos.0 <= size.width && pos.1 > 0 && pos.1 <= size.height)
                .collect::<Vec<_>>();
            antiradars
        })
        .collect::<Vec<_>>()
        .into_iter()
        .unique()
        .collect::<Vec<_>>();

    tmp.len() as u64
}

fn solve_2(matrix: &Matrix, antenna_map: &HashMap<String, HashSet<(Item, Pos)>>, size: &Size) -> u64 {
    let check_point = |pos: Pos| -> bool {
        pos.0 > 0 && pos.0 <= size.width && pos.1 > 0 && pos.1 <= size.height
    };

    let tmp = antenna_map
        .par_iter()
        .flat_map(|(name, radars)| {
            let antiradars = radars
                .iter()
                .combinations(2)
                .flat_map(|pair| {
                    let (p1_x, p1_y) = (pair[0].1.0, pair[0].1.1);
                    let (p2_x, p2_y) = (pair[1].1.0, pair[1].1.1);
                    let vx = p2_x - p1_x;
                    let vy = p2_y - p1_y;
                    let mut p1 = (p1_x, p1_y);
                    let mut p2 = (p2_x, p2_y);
                    let mut antinodes = vec![p1, p2];
                    let mut steps = 0;

                    loop {
                        p1 = (p1.0 - vx, p1.1 - vy);
                        p2 = (p2.0 + vx, p2.1 + vy);
                        let c1 = check_point(p1);
                        let c2 = check_point(p2);
                        if c1 {
                            antinodes.push(p1);
                        }
                        if c2 {
                            antinodes.push(p2);
                        }
                        if !c1 && !c2 {
                            break;
                        }
                    }
                    antinodes
                })
                .collect::<Vec<_>>();
            antiradars
        })
        .collect::<Vec<_>>()
        .into_iter()
        .unique()
        .collect::<Vec<_>>();

    tmp.len() as u64
}


fn parse(input: &str) -> (Matrix, HashMap<String, HashSet<(crate::Item, Pos)>>, Size) {
    let mut matrix: Matrix = HashMap::new();
    let mut antenna_map: HashMap<String, HashSet<(Item, Pos)>> = HashMap::new();
    let mut size = Size { width: 0, height: 0 };

    for (y, line) in input.lines().enumerate() {
        if !line.is_empty() {
            size.width = line.len() as i32;

            line.chars().enumerate().for_each(|(x, mut c)| {
                let pos: Pos = (x as i32 + 1, y as i32 + 1);
                if c == '.' {
                    matrix.insert(pos, Item::Empty);
                } else {
                    let antenna_sym = String::from(c);
                    let antenna_type = Item::Antenna(antenna_sym.clone());
                    let antenna: (Item, Pos) = (antenna_type.clone(), pos);
                    matrix.insert(pos, antenna_type.clone());
                    antenna_map.entry(antenna_sym)
                        .and_modify(|antenna_set| {
                            antenna_set.insert(antenna.clone());
                        })
                        .or_insert(HashSet::from([antenna.clone()]));
                }
            });

        }
    }

    size.height = input.lines().count() as i32;

    (matrix, antenna_map, size)
}
