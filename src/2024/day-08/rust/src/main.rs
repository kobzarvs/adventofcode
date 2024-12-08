use day_08::{read_file, Pos, Size};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file();

    let (radars, size) = parse(&input);

    let part_1 = solve_1(&radars, &size);
    println!("Part I: {:?}", part_1);

    let part_2 = solve_2(&radars, &size);
    println!("Part II: {:?}", part_2);

    Ok(())
}

fn solve_1(radar_map: &HashMap<String, HashSet<Pos>>, size: &Size) -> u64 {
    let check_point = |pos: &Pos| -> bool {
        pos.x > 0 && pos.x <= size.width && pos.y > 0 && pos.y <= size.height
    };

    let tmp = radar_map
        .par_iter()
        .flat_map(|(_name, radars)| {
            let projections = radars
                .into_iter()
                .combinations(2)
                .flat_map(|pair| {
                    let v = *pair[1] - *pair[0];
                    [*pair[0] - v, *pair[1] + v]
                })
                .filter(|pos| check_point(pos))
                .collect::<Vec<_>>();
            projections
        })
        .collect::<Vec<_>>();

    tmp.into_iter().unique().collect::<Vec<_>>().len() as u64
}

fn solve_2(radar_map: &HashMap<String, HashSet<Pos>>, size: &Size) -> u64 {
    let check_point = |pos: &Pos| -> bool {
        pos.x > 0 && pos.x <= size.width && pos.y > 0 && pos.y <= size.height
    };

    let tmp = radar_map
        .par_iter()
        .flat_map(|(_name, radars)| {
            radars
                .iter()
                .combinations(2)
                .flat_map(|pair| {
                    let mut p1 = *pair[0];
                    let mut p2 = *pair[1];
                    let v = p2 - p1;
                    let mut projections = vec![p1, p2];

                    // строим проекции радаров до упора в обе стороны
                    loop {
                        p1 = p1 + v;
                        p2 = p2 - v;
                        let c1 = check_point(&p1);
                        let c2 = check_point(&p2);
                        if c1 {
                            projections.push(p1);
                        }
                        if c2 {
                            projections.push(p2);
                        }
                        // если оба луча вышли за пределы поля то выходим из цикла
                        if !c1 && !c2 {
                            break;
                        }
                    }
                    projections
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .unique()
        .collect::<Vec<_>>();

    tmp.len() as u64
}

fn parse(input: &str) -> (HashMap<String, HashSet<Pos>>, Size) {
    let mut radar_map: HashMap<String, HashSet<Pos>> = HashMap::new();
    let mut size = Size {
        width: 0,
        height: 0,
    };

    for (y, line) in input.lines().enumerate() {
        if !line.is_empty() {
            size.width = line.len() as i32;

            line.chars().enumerate().for_each(|(x, c)| {
                let pos: Pos = Pos {
                    x: x as i32 + 1,
                    y: y as i32 + 1,
                };
                if c != '.' {
                    let radar_sym = String::from(c);
                    radar_map
                        .entry(radar_sym)
                        .and_modify(|radar_set| {
                            radar_set.insert(pos);
                        })
                        .or_insert(HashSet::from([pos]));
                }
            });
        }
    }

    size.height = input.lines().count() as i32;

    (radar_map, size)
}
