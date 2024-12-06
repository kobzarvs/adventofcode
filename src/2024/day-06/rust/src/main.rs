use day_06::read_file;
use regex::Regex;
use std::cmp::{Ordering, PartialEq};
use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

// static NamedMoves: HashMap<Dir, (i32, i32)> = HashMap::from([
//     (Dir::Up, (-1, 0)),
//     (Dir::Right, (0, 1)),
//     (Dir::Down, (1, 0)),
//     (Dir::Left, (0, -1)),
// ]);

static Moves: &[(i32, i32)] = &[
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1)
];


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file();

    let (matrix, start_point) = parse(&input);

    let part_1 = solve_1(&matrix, start_point);
    let part_2 = solve_2(&matrix, start_point);

    println!("Part I: {:?}", part_1);
    println!("Part II: {:?}", part_2);

    Ok(())
}

fn parse(input: &str) -> (HashMap<(i32, i32), char>, (i32, i32)) {
    let mut matrix: HashMap<(i32, i32), char> = HashMap::new();
    let mut start_point: (i32, i32) = (-1, -1);

    for (y, line) in input.lines().enumerate() {
        if !line.is_empty() {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| {
                    matrix.insert((y as i32, x as i32), c);
                    if c == '^' {
                        start_point = (y as i32, x as i32);
                    }
                });
        }
    }

    (matrix, start_point)
}

fn get_next_pos(pos: &(i32, i32), dir: &(i32, i32)) -> (i32, i32) {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn solve_1(matrix: &HashMap<(i32, i32), char>, start_point: (i32, i32)) -> usize {
    let mut moves = Moves.clone().iter().cycle();
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut pos = start_point;
    let mut dir = moves.next().unwrap();
    let mut steps = 1;

    while let Some(&place) = matrix.get(&get_next_pos(&pos, &dir)) {
        if place == '#' {
            dir = moves.next().unwrap();
        }

        if !visited.contains(&pos) {
            steps += 1;
        }

        visited.insert((pos.0, pos.1));
        pos = (pos.0 + dir.0, pos.1 + dir.1);

        if steps > 100000 {
            break;
        }
    }

    steps
}

fn solve_2(matrix: &HashMap<(i32, i32), char>, start_point: (i32, i32)) -> usize {
    let mut moves = Moves.clone().iter().cycle();
    let mut next_moves = Moves.clone().iter().cycle();
    next_moves.next();
    next_moves.next();
    let mut pos = start_point;
    let mut dir = moves.next().unwrap();
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut bumped = HashSet::<(i32, i32)>::new();
    let mut steps = 1;
    let mut barrier = (-1, -1);

    let mut reset = || {
        moves = Moves.clone().iter().cycle();
        next_moves = Moves.clone().iter().cycle();
        next_moves.next();
        next_moves.next();
        dir = moves.next().unwrap();
        pos = start_point;
        visited.clear();
    };

    loop {
        if steps > 100000 {
            break;
        }

        let next_pos = next_moves.next().unwrap();

        match matrix.get(&next_pos) {
            Some(&place) => {
                if place == '#' {
                    if bumped.contains(&get_next_pos(&pos, &dir)) {
                        println!("Bumped: {:?} {:?}", pos, dir);
                        reset();
                        continue;
                    }
                    bumped.insert((pos.0, pos.1));
                    dir = moves.next().unwrap();
                    println!("new dir: {:?} from pos {:?}", dir, pos);
                }

                if !visited.contains(&pos) {
                    steps += 1;
                }

                visited.insert((pos.0, pos.1));
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
            None => {
                println!("exit from matrix {:?} {:?}", pos, dir);
                reset();
                break;
            }
        }
        // println!("{}: {:?} {:?} {:?} {:?}", steps, dir, pos, matrix[&pos], place);
    }

    steps
}

