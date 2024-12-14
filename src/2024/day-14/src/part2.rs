use crate::parse;
use crate::models::{Robot, HEIGHT, WIDTH, CENTER};
use std::collections::HashSet;
use std::hash::{Hash, Hasher, DefaultHasher};
use std::collections::HashMap;

#[derive(Clone, Eq)]
struct RobotsState {
    positions: Vec<(i32, i32)>
}

impl PartialEq for RobotsState {
    fn eq(&self, other: &Self) -> bool {
        let mut self_pos = self.positions.clone();
        let mut other_pos = other.positions.clone();
        self_pos.sort_unstable();
        other_pos.sort_unstable();
        self_pos == other_pos
    }
}

impl Hash for RobotsState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut positions = self.positions.clone();
        positions.sort_unstable();
        positions.hash(state);
    }
}

pub fn run(input: &str) -> usize {
    solve(&parse(&input))
}

const N: usize = 100;

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
            picture.push(if count == 0 { '.' } else { char::from_digit(count as u32, 10).unwrap_or('*') });
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
    let mean_dist = robots.iter()
        .map(|r| ((r.p.x - CENTER) as f64).abs())
        .sum::<f64>() / n;
    
    // Считаем дисперсию расстояний от центральной линии
    let variance = robots.iter()
        .map(|r| {
            let dist = ((r.p.x - CENTER) as f64).abs();
            (dist - mean_dist).powi(2)
        })
        .sum::<f64>() / n;
    
    // Считаем энтропию распределения по столбцам
    let mut column_counts = vec![0i32; WIDTH as usize];
    for robot in robots {
        column_counts[robot.p.x as usize] += 1;
    }
    
    let entropy = column_counts.iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / n;
            -p * p.ln()
        })
        .sum::<f64>();
    
    (variance, entropy)
}

fn solve(robots: &[Robot]) -> usize {
    let mut robots = robots.to_vec();
    let mut hash_counts = HashMap::new();
    let mut n = 0;
    
    for step in 0..=6888 {
        let (variance, entropy) = calculate_center_line_metrics(&robots);
        let hash = get_picture_hash(&robots);
        let count = hash_counts.entry(hash).or_insert(0);
        
        if entropy < 3.9 && *count == 0 {
            println!("Step {}: Hash: {:?}, Center Variance = {:.2}, Column Entropy = {:.2}", 
                    step, hash, variance, entropy);
            print_grid(&robots);
            *count += 1;
            n += 1;
        }
        
        robots.iter_mut().for_each(|robot| {
            robot.do_move();
        });
    }
    
    println!("count: {n}");
    
    hash_counts.values().filter(|&&count| count > 1).count()
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
                print!(".");
            } else {
                print!("{:1}", count);
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

        assert_eq!(0, part2::solve(&data));
    }
}
