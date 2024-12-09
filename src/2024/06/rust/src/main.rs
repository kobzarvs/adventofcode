use day_06::{read_file, Pos, Size};
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use code_timing_macros::time_snippet;

const DIRS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file();

    let (matrix, matrix2, start_point, size) = time_snippet!(parse(&input));

    let part_1 = time_snippet!(solve_1(&matrix, start_point));
    // let part_2 = solve_2(&matrix2, start_point, size);

    println!("\nPart I: {:?}", part_1);
    // println!("\nPart II: {:?}", part_2);

    Ok(())
}

fn parse(
    input: &str,
) -> (
    HashMap<(i32, i32), char>,
    HashMap<(i32, i32), (i32, i32, i32, i32, char)>,
    (i32, i32),
    (i32, i32),
) {
    let mut matrix: HashMap<(i32, i32), char> = HashMap::new();
    let mut matrix2: HashMap<(i32, i32), (i32, i32, i32, i32, char)> = HashMap::new();
    let mut start_point: (i32, i32) = (-1, -1);
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    for (y, line) in input.lines().enumerate() {
        if !line.is_empty() {
            line.chars().enumerate().for_each(|(x, mut c)| {
                if c == '^' {
                    start_point = (y as i32 + 1, x as i32 + 1);
                    c = 0 as char;
                } else if c == '.' {
                    c = 0 as char;
                }
                matrix.insert((y as i32 + 1, x as i32 + 1), c);
                matrix2.insert((y as i32 + 1, x as i32 + 1), (0, 0, 0, 0, c));
                width = max(width, x as i32);
                height = max(height, y as i32);
            });
        }
    }

    (matrix, matrix2, start_point, (height + 1, width + 1))
}

fn get_next_pos(pos: &(i32, i32), dir: &(i32, i32)) -> (i32, i32) {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn solve_1(matrix: &HashMap<(i32, i32), char>, start_point: (i32, i32)) -> usize {
    let mut moves = DIRS.iter().cycle();
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


fn solve_2(
    input_matrix: &HashMap<(i32, i32), (i32, i32, i32, i32, char)>,
    start_point: (i32, i32),
    size: (i32, i32),
) -> usize {
    let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let get_next_dir = |dir: &(i32, i32)| -> (i32, i32) {
        let i = directions.iter().position(|&d| d == *dir).unwrap();
        directions[(i + 1) % 4]
    };

    let mut matrix = input_matrix.clone();
    let mut pos = start_point;
    let mut dir = directions[0];
    let mut turn = false;

    let mut valid_barriers = HashSet::<(i32, i32)>::new();
    let mut barriers = HashSet::<(i32, i32)>::new();
    let mut barriers_count = 0;
    let mut barrier_pos: Option<((i32, i32), (i32, i32))> = None;
    let mut visited: Vec<((i32, i32), (i32, i32))> = vec![];
    let mut steps = 0;


    matrix.insert(start_point, (0, 0, 0, 0, '.'));

    loop {
        if steps > 1_000_000_000 {
            break;
        }

        let next_pos = get_next_pos(&pos, &dir);
        let mut next_place = matrix.get(&next_pos);

        match next_place {
            Some(&(up, rg, dn, lf, next_place)) => {
                if pos == (7, 5) {
                    println!("debug: {:?} {:?} {:?}", pos, dir, barrier_pos);
                }
                if visited.contains(&(pos, dir)) && barrier_pos.is_some() {
                    let (bp, bd) = barrier_pos.unwrap();
                    println!("visited: {:?}\n{:?}\n{:?}\n{:?}", pos, dir, visited, barriers);
                    // запоминаем барьеры приводящие к time loop
                    valid_barriers.insert(bp);

                    // убираем наш барьер с дороги
                    barrier_pos = None;
                    println!("\rLoop: {:?} {:?}", pos, dir);

                    let v: Vec<(i32, i32)> = visited.iter().map(|(p, d)| *p).collect();
                    let b: Vec<(i32, i32)> = barriers.iter().cloned().collect();
                    print_matrix2(&matrix, size, &b);

                    // checkpoint = visited.len();
                    visited.clear();
                    (pos, dir) = (start_point, directions[0]);

                    continue;
                }

                // если впереди барьер
                if next_place == '#' {
                    // меняем направление движения
                    turn = true;
                } else {
                    // если впереди чисто и нет выставленных барьеров,
                    // проверяем не ставили ли мы ранее барьер в этой точке
                    if barrier_pos.is_none() && !barriers.contains(&pos) {
                        // запоминаем длину пути до барьера
                        // checkpoint = visited.len();
                        // запоминаем позицию и направление впереди которого будет барьер
                        barrier_pos = Some((pos, dir));
                        matrix.insert(next_pos, (0, 0, 0, 0, '#'));
                        // добавляем барьер в сет, чтобы дважды не поставить одинаковый
                        barriers.insert(pos);

                        // меняем направление движение
                        turn = true;
                    }
                }
            }
            None => {
                // если выход за границы поля и нет установленных барьеров
                // это конец - выходим из бесконечного цикла
                if barrier_pos.is_none() {
                    println!("\n--- exit from matrix {:?} {:?} {}", pos, dir, steps);
                    break;
                } else {
                    let (bp, bd) = barrier_pos.unwrap();
                    let np = get_next_pos(&bp, &bd);

                    // убираем наш барьер с дороги
                    matrix.entry(np).and_modify(|it| *it = (0, 0, 0, 0, '.'));
                    barrier_pos = None;

                    // возвращаем персонаж на старт
                    visited.clear();
                    (pos, dir) = (start_point, directions[0]);
                }
            }
        }

        visited.push((pos, dir));

        // идем вперед только если не меняли направление движения
        if !turn {
            pos = get_next_pos(&pos, &dir);
        } else {
            dir = get_next_dir(&dir);
            turn = false;
        }

        matrix.entry(pos).and_modify(|x| match dir {
            (-1, 0) => {
                (*x).4 = '^';
            }
            (0, 1) => {
                (*x).4 = '>';
            }
            (1, 0) => {
                (*x).4 = 'v';
            }
            (0, -1) => {
                (*x).4 = '<';
            }
            _ => {}
        });

        steps += 1;
    }

    println!("\nSteps: {} {} {}", steps, barriers_count, valid_barriers.len());

    valid_barriers.len()
}


fn print_matrix2(matrix: &HashMap<(i32, i32), (i32, i32, i32, i32, char)>, size: (i32, i32), objs: &[(i32, i32)]) {
    for y in 1..=size.0 {
        for x in 1..=size.1 {
            if objs.contains(&(y, x)) {
                print!("X ");
            } else {
                let (u, r, d, l, c) = matrix[&(y, x)];
                if ['#', '@', '$', '^', '<', 'v', '>'].contains(&c) {
                    print!("{} ", c);
                } else {
                    print!(". ");
                }
            }
        }
        println!();
    }
}

fn print_matrix(
    matrix: &mut HashMap<(i32, i32), (i32, i32, i32, i32, char)>,
    pos: (i32, i32),
    dir: &mut (i32, i32),
    size: (i32, i32),
    start_point: (i32, i32),
) {
    matrix.entry(start_point).and_modify(|x| (*x).4 = '$');
    // matrix.entry(barrier_pos).and_modify(|x| (*x).4 = '@');

    for y in 1..=size.0 {
        for x in 1..=size.1 {
            let (u, r, d, l, c) = matrix[&(y, x)];
            if ['#', '@', '$', '^', '<', 'v', '>'].contains(&c) {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}
