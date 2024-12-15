use crate::models::{Robot, CENTER, HEIGHT, WIDTH};
use crate::parse;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn run(input: &str) -> i32 {
    solve(&parse(&input))
}

fn get_picture_hash(robots: &[Robot]) -> u64 {
    // Создаем строковое представление рисунка
    let mut picture = String::with_capacity(WIDTH as usize * HEIGHT as usize);
    let mut grid = vec![vec![0i32; WIDTH as usize]; HEIGHT as usize];

    // Заполняем сетку
    for robot in robots {
        let x = robot.p.x as usize;
        let y = robot.p.y as usize;
        grid[y][x] += 1;
    }

    // Преобразуем сетку в строку
    for row in grid {
        for count in row {
            picture.push(if count == 0 {
                '.'
            } else {
                char::from_digit(count as u32, 10).unwrap_or('*')
            });
        }
    }

    // Вычисляем хеш от строки
    let mut hasher = DefaultHasher::new();
    picture.hash(&mut hasher);
    hasher.finish()
}

fn calculate_center_line_metrics(robots: &[Robot]) -> (f64, f64) {
    let n = robots.len() as f64;

    // Считаем среднее расстояние только по X от центральной линии
    let mean_dist = robots
        .iter()
        .map(|r| ((r.p.x - CENTER) as f64).abs())
        .sum::<f64>()
        / n;

    // Считаем дисперсию расстояний от центральной линии
    let variance = robots
        .iter()
        .map(|r| {
            let dist = ((r.p.x - CENTER) as f64).abs();
            (dist - mean_dist).powi(2)
        })
        .sum::<f64>()
        / n;

    // Считаем энтропию распределения по столбцам
    let mut column_counts = vec![0_i32; WIDTH as usize];
    let mut row_counts = vec![0_i32; HEIGHT as usize];
    for robot in robots {
        column_counts[robot.p.x as usize] += 1;
        row_counts[robot.p.y as usize] += 1;
    }

    let entropy = column_counts
        .into_iter()
        .filter(|&count| count > 0)
        .map(|count| {
            let p = count as f64 / n;
            -p * p.ln()
        })
        .sum::<f64>()
        + row_counts
            .into_iter()
            .filter(|&count| count > 0)
            .map(|count| {
                let p = count as f64 / n;
                -p * p.ln()
            })
            .sum::<f64>();

    (variance, entropy)
}

fn solve(data: &[Robot]) -> i32 {
    let mut robots = data.to_vec();
    let mut entropy_map: HashMap<u64, (i32, f64, f64)> = HashMap::new();
    // let mut n = 0;
    let mut hash_counts = HashMap::new();

    for step in 0..=10_000 {
        let (variance, entropy) = calculate_center_line_metrics(&robots);
        let hash = get_picture_hash(&robots);
        let count = hash_counts.entry(hash).or_insert(0);
        
        if *count == 0 {
            entropy_map.insert(hash, (step, variance, entropy));
        }
        *count += 1;

        // if entropy <= 8.5 && *count == 0 {
        //     println!(
        //         "Step {}: Hash: {:?}, Center Variance = {:.2}, Column Entropy = {:.12}",
        //         step, hash, variance, entropy
        //     );
        //     print_grid(&robots);
        //     *count += 1;
        //     n += 1;
        // }

        robots.iter_mut().for_each(|robot| {
            robot.do_move(1);
        });
    }

    let (hash, (step, variance, entropy)) = entropy_map
        .into_iter()
        .min_by(|a, b| a.1 .2.total_cmp(&(b.1 .2)))
        .unwrap();

    let mut robots = data.to_vec();
    robots.iter_mut().for_each(|robot| {
        robot.do_move(step);
    });
    println!(
        "Step {}: Hash: {:?}, Center Variance = {:.2}, Column Entropy = {:.12}",
        step, hash, variance, entropy
    );
    print_grid(&robots);

    step
}

fn print_grid(robots: &[Robot]) {
    let mut grid = vec![vec![0i32; WIDTH as usize]; HEIGHT as usize];

    for robot in robots {
        let x = robot.p.x as usize;
        let y = robot.p.y as usize;
        grid[y][x] += 1;
    }

    for row in grid {
        for count in row {
            if count == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(31, part2::solve(&data));
    }
}
