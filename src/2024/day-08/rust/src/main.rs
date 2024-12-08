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

fn solve_1(radar_map: &HashMap<String, HashSet<Pos>>, size: &Size) -> usize {
    let check_point = |pos: &Pos| -> bool {
        pos.x > 0 && pos.x <= size.width && pos.y > 0 && pos.y <= size.height
    };

    let tmp = radar_map
        .par_iter()
        .flat_map(|(_name, radars)| {
            radars
                .into_iter()
                .combinations(2)
                .flat_map(|pair| {
                    let v = *pair[1] - *pair[0];
                    [*pair[0] - v, *pair[1] + v]
                })
                .filter(check_point)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    tmp.iter().unique().count()
}

fn solve_2(radar_map: &HashMap<String, HashSet<Pos>>, size: &Size) -> usize {
    let check_point = |pos: &Pos, points: &mut Vec<Pos>, callback: fn(&mut Vec<Pos>, Pos)| -> bool {
        let result = pos.x > 0 && pos.x <= size.width && pos.y > 0 && pos.y <= size.height;
        if result { callback(points, *pos); }
        result
    };

    radar_map
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
                        let c1 = check_point(&p1, &mut projections, |pts, p| pts.push(p));
                        let c2 = check_point(&p2, &mut projections, |pts, p| pts.push(p));
                        // если оба луча вышли за пределы поля, то выходим из цикла
                        if !c1 && !c2 {
                            break projections;
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .unique()
        .count()
}

fn parse(input: &str) -> (HashMap<String, HashSet<Pos>>, Size) {
    let mut radar_map: HashMap<String, HashSet<Pos>> = HashMap::new();
    let size = Size {
        width: input.lines().next().unwrap().len() as i32,
        height: input.lines().count() as i32,
    };

    for (y, line) in input.lines().enumerate() {
        if !line.is_empty() {
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

    (radar_map, size)
}
