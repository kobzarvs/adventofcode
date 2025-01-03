use std::collections::HashMap;
use std::{env, fs};
use std::ops::{Add, Sub};
use std::path::PathBuf;
use geo::{coord, Coord};

pub type Topo = HashMap<Pos, i32>;

pub fn transpose(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| matrix[row][col]).collect())
        .collect()
}

pub fn process_matrix<F>(matrix: &[Vec<char>], size: usize, mut processor: F)
where
    F: FnMut(&Vec<Vec<char>>, usize, usize),
{
    for y in 0..matrix.len() - size + 1 {
        for x in 0..matrix[0].len() - size + 1 {
            let mut window = vec![];
            for i in 0..size {
                window.push(matrix[y + i][x..x + size].to_vec());
            }

            processor(&window, y, x);
        }
    }
}

pub fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("test.txt"));
    println!("Input file: {:?}", filename);
    fs::read_to_string(filename).expect("Unable to read file")
}

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub type Point = (i32, i32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Dir {
    North,
    South,
    East,
    West,
}


#[derive(Debug)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

pub const DIRECTIONS: [Pos; 4] = [
    Pos { x: 1, y: 0 },
    Pos { x: -1, y: 0 },
    Pos { x: 0, y: 1 },
    Pos { x: 0, y: -1 },
];

pub const COORD_DIRS: [Coord<i32>; 4] = [
    coord! {x:1, y:0},
    coord! {x:0, y:1},
    coord! {x:-1, y:0},
    coord! {x:0, y:-1},
];
