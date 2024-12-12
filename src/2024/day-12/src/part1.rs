use crate::{parse_part1, Pos};
use std::collections::{HashSet, VecDeque};

pub fn run(input: &str) -> i32 {
    let data = parse_part1(input);
    solve(&data)
}

fn solve(map: &[Vec<char>]) -> i32 {
    calculate_region_value(map)
}

fn get_neighbors(pos: Pos, max_x: usize, max_y: usize) -> Vec<Option<Pos>> {
    let mut neighbors = Vec::new();
    if pos.x > 0 {
        neighbors.push(Some(Pos {
            x: pos.x - 1,
            y: pos.y,
        }));
    } else {
        neighbors.push(None)
    }
    if pos.x < (max_x - 1) as i32 {
        neighbors.push(Some(Pos {
            x: pos.x + 1,
            y: pos.y,
        }));
    } else {
        neighbors.push(None)
    }
    if pos.y > 0 {
        neighbors.push(Some(Pos {
            x: pos.x,
            y: pos.y - 1,
        }));
    } else {
        neighbors.push(None)
    }
    if pos.y < (max_y - 1) as i32 {
        neighbors.push(Some(Pos {
            x: pos.x,
            y: pos.y + 1,
        }));
    } else {
        neighbors.push(None)
    }
    neighbors
}

fn calculate_region_value(map: &[Vec<char>]) -> i32 {
    let max_x = map[0].len();
    let max_y = map.len();
    let mut visited = HashSet::new();
    let mut total_value = 0;

    for y in 0..max_y {
        for x in 0..max_x {
            let pos = Pos {
                x: x as i32,
                y: y as i32,
            };
            if visited.contains(&pos) {
                continue;
            }

            let region_char = map[y][x];
            let mut region_size = 0;
            let mut boundary_length = 0;
            let mut queue = VecDeque::new();
            queue.push_back(pos);

            while let Some(current) = queue.pop_front() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);
                region_size += 1;

                for neighbor in get_neighbors(current, max_x, max_y) {
                    match neighbor {
                        Some(pos) => {
                            if map[pos.y as usize][pos.x as usize] == region_char {
                                if !visited.contains(&pos) {
                                    queue.push_back(pos);
                                }
                            } else {
                                boundary_length += 1;
                            }
                        }
                        _ => boundary_length += 1,
                    }
                }
            }

            total_value += region_size * boundary_length;
        }
    }

    total_value
}

#[cfg(test)]
mod tests {
    use crate::{parse_part1, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");

        let data = parse_part1(input);
        assert_eq!(1930, part1::solve(&data));
    }
}
