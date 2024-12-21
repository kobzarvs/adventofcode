#![allow(unused_mut)]

use crate::Size;
use geo::{coord, Coord};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MazePath {
    pub parent: Option<Rc<MazePath>>,
    pub turns: i32,
    pub length: i32,
    pub pos: Coord<i32>,
    pub dir: Coord<i32>,
    pub cost: i32,
    pub priority: i32,
}

pub struct MazePathIterator {
    current: Option<Rc<MazePath>>,
}

impl MazePath {
    pub fn iter(&self) -> MazePathIterator {
        MazePathIterator {
            current: self.parent.clone(),
        }
    }

    pub fn has_in_path(&self, pos: &Coord<i32>) -> bool {
        if *pos == self.pos {
            return true;
        }
        self.parent
            .as_ref()
            .map_or(false, |parent| parent.has_in_path(pos))
    }

    pub fn get_full_path_iter(&self) -> impl Iterator<Item = Coord<i32>> + '_ {
        std::iter::once(self.pos).chain(self.iter().map(|path| path.pos))
    }
}

impl Iterator for MazePathIterator {
    type Item = Rc<MazePath>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone()?;
        self.current = current.parent.clone();
        Some(current)
    }
}

impl Ord for MazePath {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for MazePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MazeCell {
    Free,    // "."
    Wall,    // "#" 
}

pub type Maze = HashMap<Coord<i32>, MazeCell>;

pub fn parse_maze(input: &str) -> (Maze, Coord<i32>, Coord<i32>, Size) {
    let mut maze: Maze = HashMap::new();
    let mut start = Coord::zero();
    let mut end = Coord::zero();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = coord! {x: x as i32, y: y as i32};

            match c {
                '#' => {
                    maze.insert(coord, MazeCell::Wall);
                },
                '.' => {
                    maze.insert(coord, MazeCell::Free);
                },
                'S' => {
                    start = coord;
                    maze.insert(coord, MazeCell::Free);
                },
                'E' => {
                    end = coord;
                    maze.insert(coord, MazeCell::Free);
                },
                _ => panic!("Неверный символ в лабиринте: {}", c),
            };
        }
    }

    (maze, start, end, Size { height, width })
}
