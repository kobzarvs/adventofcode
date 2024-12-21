use std::cmp::Ordering;
use std::rc::Rc;
use geo::Coord;


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

    // pub fn each(&self) -> impl Iterator<Item = Self> + '_ {
    //     std::iter::once(self).chain(self.iter().map(|instance| instance))
    // }
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

pub fn parse_maze(input: &str) -> (Vec<Vec<bool>>, (i32, i32), (i32, i32)) {
    let mut maze = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(false),
                '.' => row.push(true),
                'S' => {
                    row.push(true);
                    start = (x as i32, y as i32);
                },
                'E' => {
                    row.push(true);
                    end = (x as i32, y as i32);
                },
                _ => panic!("Неверный символ в лабиринте: {}", c),
            }
        }
        maze.push(row);
    }

    (maze, start, end)
}
