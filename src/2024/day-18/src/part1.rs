use crate::{parse, Byte, MazePath, COORD_DIRS};
use geo::{coord, Coord};
use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashSet;
use std::collections::{BinaryHeap, HashMap};
use std::io;
use std::io::{Read, Write};
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

pub fn run(input: &str) -> i32 {
    solve(&parse(input))
}

#[cfg(not(debug_assertions))]
pub const SIZE: i32 = 71;
#[cfg(debug_assertions)]
pub const SIZE: i32 = 7;

#[cfg(not(debug_assertions))]
pub const FALLEN: usize = 1024;
#[cfg(debug_assertions)]
pub const FALLEN: usize = 12;

// #[derive(Debug, Clone)]
// struct State {
//     turns: i32,
//
// }

fn solve(bytes: &[Byte]) -> i32 {
    println!("**** {:?}", bytes);
    let bytes = bytes[0..FALLEN].as_ref();
    let mut b_map: HashSet<Coord<i32>> = HashSet::new();
    for &b in bytes {
        b_map.insert(b.1);
    }

    let limit = 1024;
    let (w, h) = (71, 71);
    let start = (0, 0);
    let end = (70, 70);

    let mut graph = Graph::<(usize, usize), ()>::new();
    let mut node_map = vec![vec![None; h]; w];

    // Create grid nodes
    for x in 0..w {
        for y in 0..h {
            let index = graph.add_node((x, y));
            node_map[x][y] = Some(index);
        }
    }

    // Add edges
    for x in 0..w {
        for y in 0..h {
            if b_map.contains(&coord! {x: (x + 1) as i32, y: y as i32})
                || b_map.contains(&coord! {x: x as i32, y: y as i32})
                || b_map.contains(&coord! {x: x as i32, y: (y + 1) as i32})
            {
                continue;
            }

            if x + 1 < w {
                let edge = (node_map[x][y].unwrap(), node_map[x + 1][y].unwrap());
                graph.add_edge(edge.0, edge.1, ());
            }
            if y + 1 < h {
                let edge = (node_map[x][y].unwrap(), node_map[x][y + 1].unwrap());
                graph.add_edge(edge.0, edge.1, ());
            }
        }
    }

    let mut path = None;

    for (i, (index, b)) in bytes.iter().enumerate() {
        if i == limit {
            if let Some((_, p)) = astar(
                &graph,
                node_map[start.0][start.1].unwrap(),
                |finish| finish == node_map[end.0][end.1].unwrap(),
                |_| 1,
                |_| 0,
            ) {
                path = Some(p);
                println!("{}", path.as_ref().unwrap().len() - 1);
            }
        }

        // for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        //     let nx = b.x + dx;
        //     let ny = b.y + dy;
        //     if nx < w && ny < h {
        //         if let (Some(n1), Some(n2)) = (node_map[b.x][b.y], node_map[nx][ny]) {
        //             graph.remove_edge(n1, n2);
        //         }
        //     }
        // }

        if i >= limit {
            if let Some(ref p) = path {
                if p.contains(&node_map[b.x as usize][b.y as usize].unwrap()) {
                    if let Some((_, new_path)) = astar(
                        &graph,
                        node_map[start.0][start.1].unwrap(),
                        |n| n == node_map[end.0][end.1].unwrap(),
                        |_| 1,
                        |_| 0,
                    ) {
                        path = Some(new_path);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    0
}

fn solve2(bytes: &[Byte]) -> i32 {
    let target = coord! {x: SIZE - 1, y: SIZE - 1};
    // let map = HashMap::<Coord<i32>, bool>::new();

    let bytes = bytes[0..FALLEN].as_ref();
    // println!("bytes: {:?}, len: {}", bytes, bytes.len());

    let mut visited: HashMap<(Coord<i32>, i32), i32> = HashMap::new();
    let mut costs: HashMap<Coord<i32>, i32> = HashMap::new();
    let mut shortest_path: Vec<Coord<i32>> = vec![];
    let mut shortest_path_len = i32::MAX;
    // let mut queue = VecDeque::new();
    let mut heap = BinaryHeap::new();
    let mut window = coord! {x: 0, y: -1};

    let start = MazePath {
        parent: None,
        turns: 0,
        length: 0,
        pos: coord! {x: 0, y: 0},
        dir: Coord::zero(),
        cost: 0,
        priority: target.x + target.y,
    };

    // queue.push_back(start);
    heap.push(start);

    // while let Some(curr) = queue.pop_front() {
    while let Some(curr) = heap.pop() {
        // let cost = visited.get(&(curr.pos, curr.turns)).unwrap_or(&i32::MAX);

        let path = curr.get_full_path_iter().collect_vec();
        // print!("\x1B[2J\x1B[1;1H");
        // io::stdout().flush().unwrap();
        print_map_w2(bytes, &path, &mut window, &curr.pos);
        // let mut buffer = [0; 1];
        // io::stdin().read_exact(&mut buffer).unwrap();

        if curr.pos == target {
            // if curr.length < shortest_path_len {
            shortest_path.clear();
            for pos in curr.get_full_path_iter() {
                shortest_path.push(pos);
            }
            shortest_path_len = curr.length;
            println!("\n\nFound path: {}\n\n", curr.length);
            print_map_w2(bytes, &shortest_path, &mut window, &curr.pos);
            // }
            break;
        }

        // if curr.length > *cost {
        //     // println!("pos: {:?}, cost: {} < {}", curr.pos, cost, curr.length);
        //     continue;
        // }

        // if curr.pos != Coord::zero() {
        //     visited
        //         .entry((curr.pos, curr.turns))
        //         .and_modify(|it| {
        //             *it = curr.length.min(*it);
        //         })
        //         .or_insert(curr.length);
        //
        //     // println!("visited: {:?}", visited);
        // }

        let rc_curr = Rc::new(curr);
        let curr_clone = rc_curr.clone();

        // let dirs = if curr_clone.pos.y > SIZE / 2 || curr_clone.pos.x > SIZE / 2 {
        //     [COORD_DIRS[1], COORD_DIRS[0], COORD_DIRS[3], COORD_DIRS[2]]
        // } else {
        //     COORD_DIRS
        // };

        for dir in COORD_DIRS {
            let new_pos = curr_clone.pos + dir;

            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= SIZE || new_pos.y >= SIZE {
                // println!("overflow: {:?} => {:?}", new_pos, dir);
                continue;
            }

            if curr_clone.has_in_path(&new_pos) {
                // println!("cross: {:?} => {:?}", new_pos, dir);
                continue;
            }

            if bytes.into_iter().find(|&&it| it.1 == new_pos).is_some() {
                // println!("wall");
                continue;
            }

            let turns = if curr_clone.dir != dir {
                curr_clone.turns + 1
            } else {
                curr_clone.turns
            };
            // let mut cost = 0;
            // for i in (new_pos.x-2)..=(new_pos.x+2) {
            //     for j in (new_pos.y - 2)..=(new_pos.y + 2) {
            //         if let Some(c) = visited.get(&(coord! {x: i, y: j}, turns)) {
            //             cost += c;
            //         }
            //     }
            // }
            // let mut columns: HashMap<i32, i32> = HashMap::new();
            // let mut rows: HashMap<i32, i32> = HashMap::new();
            // curr_clone.iter().for_each(|it| {
            //     columns.entry(it.pos.x).and_modify(|it| *it += 1).or_insert(1);
            //     rows.entry(it.pos.y).and_modify(|it| *it += 1).or_insert(1);
            // });
            // let c_entropy = columns.iter().fold(0_f64, |acc, (key, count)| {
            //     let p: f64 = *count as f64 / curr_clone.length as f64;
            //     acc + -p * p.ln()
            // });
            // let r_entropy = rows.iter().fold(0_f64, |acc, (key, count)| {
            //     let p: f64 = *count as f64 / curr_clone.length as f64;
            //     acc + -p * p.ln()
            // });
            // let entropy = ((c_entropy + r_entropy) * 10_000_f64) as i32;

            let manhattan_dist = target.x - new_pos.x + target.y - new_pos.y;
            let new_cost = curr_clone.cost + 1;

            if curr_clone.cost < *costs.get(&new_pos).unwrap_or(&i32::MAX) {
                costs.insert(new_pos, new_cost);
                let new_item = MazePath {
                    parent: Some(curr_clone.clone()),
                    turns,
                    length: curr_clone.length + 1,
                    pos: new_pos,
                    dir,
                    cost: curr_clone.cost + 1,
                    priority: manhattan_dist, //(SIZE-dir.y) * (manhattan_dist.x * manhattan_dist.y) * cost * turns,
                };
                // println!("new_item: {:?}", new_item);
                // queue.push_back(new_item);
                heap.push(new_item);
            }
        }
    }

    0
}

struct Path {
    pos: Coord<i32>,
}

fn print_map_w4(bytes: &[Byte], steps: &Vec<Coord<i32>>, _step: usize) {
    for y in 0..SIZE {
        for x in 0..SIZE {
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

const WIN: i32 = 35;

fn print_map_w2(
    bytes: &[Byte],
    steps: &Vec<Coord<i32>>,
    window: &mut Coord<i32>,
    pos: &Coord<i32>,
) {
    if pos.y < window.y + 10 {
        window.y = window.y.min(pos.y - 10).max(-1);
    } else if pos.y > window.y + WIN {
        window.y = window.y.max(pos.y - WIN + 10).min(SIZE);
    }
    println!("from: {}, to: {}, pos: {:?}", window.y, window.y + WIN, pos);
    for y in window.y..=(window.y + WIN).min(SIZE) {
        for x in -1..=SIZE {
            let curr = coord! {x: x, y: y};
            if *pos == curr {
                print!("@ ");
                continue;
            }

            let is_road = steps.contains(&curr);
            let wall = bytes.iter().find(|&pos| pos.1 == curr);

            if wall.is_some() && wall.unwrap().clone().1 == curr {
                if is_road {
                    print!("# ");
                } else {
                    print!("# ");
                }
            } else if is_road {
                print!(". ");
            } else {
                if x == -1 && (y == -1) || x == SIZE && (y == -1) {
                    print!("_");
                } else if x < 0 || x == SIZE {
                    print!("|");
                } else if y < 0 || y == SIZE {
                    print!("__");
                } else {
                    print!("  ");
                }
            }
        }
        println!();
    }
    io::stdout().flush().unwrap();
    std::thread::sleep(Duration::from_millis(16));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(22, part1::solve(&data));
    }
}
