use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::{BinaryHeap, HashMap};
use std::io;
use std::io::Write;
// use std::io;
// use std::io::Read;
use crate::Point;
use std::rc::Rc;
// #region-begin

#[derive(Eq, PartialEq)]
struct State {
    effect: i32,
    turns: i32,
    moves: i32,
    position: Point,
    direction: Dir,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.effect.cmp(&self.effect)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Dir {
    North,
    South,
    East,
    West,
    None,
}

pub fn find_path_min_turns(
    maze: &Vec<Vec<bool>>,
    start: Point,
    end: Point,
) -> Option<(Vec<Point>, i32)> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::<(Point, Dir), i32>::new();
    let mut came_from = HashMap::<Point, (Point, Dir)>::new();
    let mut steps = 0;
    let mut best_effective = i32::MAX;
    let mut best_path = None;

    for &dir in &[Dir::North, Dir::South, Dir::East, Dir::West] {
        heap.push(State {
            effect: 0,
            turns: 1,
            moves: 0,
            position: start,
            direction: dir,
        });
        came_from.insert(start, (start, dir));
    }

    while let Some(State {
        effect: effective,
        turns,
        moves,
        position,
        direction,
    }) = heap.pop()
    {
        steps += 1;
        if steps % 1000 == 0 {
            println!(
                "Шаг {}: поз {:?}, ходы {}, повороты {}, эфф {}",
                steps, position, moves, turns, effective
            );
        }

        if position <= end {
            let mut path = vec![position];
            println!("eff: {:?}, best: {}", effective, best_effective);
            if effective < best_effective {
                best_effective = effective;
                print_path(maze, &path);

                let mut current = position;
                while let Some(&(prev_pos, _)) = came_from.get(&current) {
                    print!("{:?} -> {:?}", prev_pos, current);
                    if prev_pos == current {
                        break;
                    }
                    current = prev_pos;
                    path.push(current);
                }
                path.reverse();

                best_path = Some((path.clone(), turns));
            }
            println!(
                "Найден путь! Шаги: {}, Ходы: {}, Повороты: {}, Эффективность: {}",
                steps, moves, turns, effective
            );
            continue;
        }

        let state_key = (position, direction);
        if visited.get(&state_key).map_or(false, |&t| t <= effective) {
            continue;
        }
        visited.insert(state_key, effective);

        if steps % 10000 == 0 {
            println!(
                "Очередь: {}, Посещено: {}, Лучшая эфф.: {}",
                heap.len(),
                visited.len(),
                best_effective
            );
        }

        for new_dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            let new_turns = if new_dir == direction {
                turns
            } else {
                turns + 1
            };
            let new_moves = moves + 1;
            let new_effective = 1000 * new_turns + new_moves;

            let (dx, dy) = direction_to_delta(new_dir);
            let new_pos = (position.0 + dx, position.1 + dy);

            if is_valid_position(maze, new_pos) {
                if !came_from.contains_key(&new_pos) || new_effective < best_effective {
                    came_from.insert(new_pos, (position, direction));
                    if new_pos == end && new_effective < best_effective {
                        best_effective = new_effective;
                    }
                    heap.push(State {
                        effect: new_effective,
                        turns: new_turns,
                        moves: new_moves,
                        position: new_pos,
                        direction: new_dir,
                    });
                }
            }
        }
    }

    println!("Поиск завершен после {} шагов. Лучшая эффективность: {}, Путь найден: {}. Размер очереди: {}, Размер visited: {}",
             steps, best_effective, best_path.is_some(), heap.len(), visited.len());
    best_path
}

#[inline]
fn direction_to_delta(dir: Dir) -> Point {
    match dir {
        Dir::North => (0, -1),
        Dir::South => (0, 1),
        Dir::East => (1, 0),
        Dir::West => (-1, 0),
        _ => unreachable!(),
    }
}

#[inline]
fn is_valid_position(maze: &Vec<Vec<bool>>, pos: Point) -> bool {
    pos.0 >= 0
        && pos.1 >= 0
        && (pos.0 as usize) < maze[0].len()
        && (pos.1 as usize) < maze.len()
        && maze[pos.1 as usize][pos.0 as usize]
}

pub fn find_paths_with_effective(
    maze: &Vec<Vec<bool>>,
    start: Point,
    end: Point,
    target_effective: i32,
) -> Vec<(Vec<Point>, i32)> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::<(Point, Dir), i32>::new();
    let mut came_from = HashMap::<Point, Vec<(Point, Dir)>>::new();
    let mut found_paths = Vec::new();
    let mut steps = 0;

    let mut target_paths_count = 0;

    for &dir in &[Dir::North, Dir::South, Dir::East, Dir::West] {
        heap.push(State {
            effect: 1_000_000,
            turns: 1,
            moves: 0,
            position: start,
            direction: dir,
        });
    }

    while let Some(State {
        effect: effective,
        turns,
        moves,
        position,
        direction,
    }) = heap.pop()
    {
        // if effective != target_effective {
        //     continue;
        // }

        steps += 1;
        // if steps % 1000 == 0 {
        //     println!("Шаг {}: поз {:?}, ходы {}, повороты {}, эфф {}",
        //         steps, position, moves, turns, effective);
        // }

        if position == end {
            println!(
                "Шаг {}: поз {:?}, ходы {}, повороты {}, эфф {}, {} {} {}",
                steps,
                position,
                moves,
                turns,
                effective,
                target_effective,
                (target_effective - 1001) >= effective,
                effective <= target_effective + 1001
            );
            if target_effective - 1001 <= effective && effective <= target_effective + 1001 {
                target_paths_count += 1;
                collect_paths(maze, start, position, turns, &came_from, &mut found_paths);
                println!(
                    "\nНайден путь с целевой эффективностью! ({}/{})",
                    target_paths_count,
                    found_paths.len()
                );
            }
            continue;
        }

        let state_key = (position, direction);
        if visited.get(&state_key).map_or(false, |&t| t <= effective) {
            continue;
        }
        visited.insert(state_key, effective);
        // if target_effective - 1001 <= effective && effective <= target_effective + 1001 {
        println!(
            "visited: {:?}",
            visited
                .iter()
                .filter(|(_, &x)| { x == target_effective })
                .collect_vec()
        );
        // }

        for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            let new_turns = if dir == direction { turns } else { turns + 1 };
            let new_moves = moves + 1;
            let new_effective = 1000 * new_turns + new_moves;

            if new_effective > target_effective {
                continue;
            }

            let (dx, dy) = direction_to_delta(dir);
            let new_pos = (position.0 + dx, position.1 + dy);

            if is_valid_position(maze, new_pos) {
                came_from
                    .entry(new_pos)
                    .or_insert_with(Vec::new)
                    .push((position, direction));

                heap.push(State {
                    effect: new_effective,
                    turns: new_turns,
                    moves: new_moves,
                    position: new_pos,
                    direction: dir,
                });
            }
        }
    }

    println!("\nПоиск завершен после {} шагов", steps);
    println!(
        "Найдено {} путей с эффективностью {}",
        found_paths.len(),
        target_effective
    );

    found_paths
}

fn collect_paths(
    maze: &Vec<Vec<bool>>,
    start: Point,
    current: Point,
    turns: i32,
    came_from: &HashMap<Point, Vec<(Point, Dir)>>,
    found_paths: &mut Vec<(Vec<Point>, i32)>,
) {
    let mut stack = vec![(current, vec![current])];
    let mut visited = HashSet::new();

    while let Some((pos, path)) = stack.pop() {
        if pos == start {
            let mut complete_path = path.clone();
            complete_path.reverse();
            found_paths.push((complete_path.clone(), turns));
            println!("\nНайден новый путь! Повороты: {}", turns);
            print_path(maze, &complete_path);
            continue;
        }

        if let Some(prev_positions) = came_from.get(&pos) {
            for &(prev_pos, _) in prev_positions {
                if !visited.contains(&prev_pos) {
                    visited.insert(prev_pos);
                    let mut new_path = path.clone();
                    new_path.push(prev_pos);
                    stack.push((prev_pos, new_path));
                }
            }
        }
    }
}

pub fn print_path(maze: &Vec<Vec<bool>>, path: &Vec<Point>) {
    // let path_set: HashSet<_> = path.iter().cloned().collect();

    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if path.contains(&(x as i32, y as i32)) {
                print!("O");
            } else if maze[y][x] {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!();
}

// #region-end

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MazePath {
    pub parent: Option<Rc<MazePath>>,
    pub turns: i32,
    pub length: i32,
    pub pos: Point,
    pub dir: Dir,
    pub effect: i32,
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

    fn has_in_path(&self, pos: Point) -> bool {
        if pos == self.pos {
            return true;
        }
        self.parent
            .as_ref()
            .map_or(false, |parent| parent.has_in_path(pos))
    }

    pub fn get_full_path_iter(&self) -> impl Iterator<Item = Point> + '_ {
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
        other.effect.cmp(&self.effect)
    }
}

impl PartialOrd for MazePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn print_path_w4(maze: &Vec<Vec<bool>>, path: &Vec<Point>, head: Point) {
    let mut i = 1;
    for y in 0..maze.len() as i32 {
        for x in 0..maze[0].len() as i32 {
            if head.0 == x && head.1 == y {
                print!(" @  ");
                continue;
            }
            if path.contains(&(x, y)) {
                print!(" {:>2} ", i);
                i += 1;
            } else if maze[y as usize][x as usize] {
                print!("    ");
            } else {
                print!("[][]");
            }
        }
        println!();
    }
    println!();
}

const WIN: i32 = 96;

pub fn print_path_w1(maze: &[Vec<bool>], path: &[Point], head: Point) {
    // let offset = if head.1 < (maze.len() as i32 - WIN + 20) as i32 {head.1 - 20} else {WIN};
    let from = (head.1 - 15).min(WIN).max(0);
    for y in from..(maze.len() as i32 - WIN + from) as i32 {
        for x in 0..maze[0].len() as i32 {
            if head.0 == x && head.1 == y {
                print!("@");
                continue;
            }
            if path.contains(&(x, y)) {
                print!("O");
            } else if maze[y as usize][x as usize] {
                print!(" ");
            } else {
                print!(".");
            }
        }
        println!();
    }
    // println!();
    io::stdout().flush().unwrap();
}

pub fn find_all_paths(maze: &Vec<Vec<bool>>, start: Point, end: Point, goal_fx: i32) -> usize {
    let mut heap = BinaryHeap::new();
    let mut found_paths = Vec::new();
    let mut fx = HashMap::<(Point, Dir), i32>::new();
    let mut steps = 0;

    heap.push(MazePath {
        parent: None,
        turns: 0,
        length: 0,
        pos: start,
        dir: Dir::None,
        effect: 0,
    });

    while let Some(state) = heap.pop() {
        steps += 1;

        if state.pos == end && state.effect == goal_fx {
            found_paths.push(state);
            continue;
        }

        let state_key = (state.pos, state.dir);
        if fx.get(&state_key).map_or(false, |&t| t < state.effect) {
            continue;
        }
        fx.insert(state_key, state.effect);

        for new_dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            let (dx, dy) = direction_to_delta(new_dir);
            let new_pos = (state.pos.0 + dx, state.pos.1 + dy);
            let new_moves = state.length + 1;
            let can_move = is_valid_position(maze, new_pos);

            if can_move {
                let is_visited = state
                    .parent
                    .as_ref()
                    .map_or(false, |p| p.has_in_path(new_pos));

                if is_visited {
                    continue;
                }

                let new_turns = if new_dir == state.dir {
                    state.turns
                } else {
                    state.turns + 1
                };

                let new_effect = 1000 * new_turns + new_moves;

                if new_effect > goal_fx {
                    continue;
                }

                let item = MazePath {
                    parent: Some(Rc::new(state.clone())),
                    turns: new_turns,
                    length: new_moves,
                    dir: new_dir,
                    pos: new_pos,
                    effect: new_effect,
                };

                heap.push(item);
            }
        }
    }

    println!("\nПоиск завершен после {} шагов", steps);
    println!("Найдено {} путей", found_paths.len());

    let mut uniq_cells: HashSet<Point> = HashSet::new();

    for path in found_paths {
        path.get_full_path_iter().for_each(|it| {
            uniq_cells.insert(it);
        });
    }

    uniq_cells.len()
}
