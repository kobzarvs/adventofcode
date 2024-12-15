use std::rc::Rc;
use crate::parse;
use crate::models::{Robot, HEIGHT, WIDTH, CENTER, MIDDLE};


pub fn run(input: &str) -> usize {
    solve(&parse(input))
}

const N: usize = 100;

fn solve(robots: &[Robot]) -> usize {
    let mut robots = robots.to_vec();
    
    // print_robots(&robots, Some(|robot| robot.p.x != CENTER && robot.p.y != MIDDLE));
    for _ in 0..N {
        robots.iter_mut().for_each(|robot| { robot.do_move(); });
    }
    // print_robots(&robots, Some(|robot| robot.p.x != CENTER && robot.p.y != MIDDLE));
    
    let robots = Rc::new(robots);
    let q1 = robots.iter().filter(|r| r.p.x > CENTER && r.p.y > MIDDLE).count();
    let q2 = robots.iter().filter(|r| r.p.x < CENTER && r.p.y > MIDDLE).count();
    let q3 = robots.iter().filter(|r| r.p.x < CENTER && r.p.y < MIDDLE).count();
    let q4 = robots.iter().filter(|r| r.p.x > CENTER && r.p.y < MIDDLE).count();
    
    q1 * q2 * q3 * q4
}

#[allow(dead_code)]
fn print_robots(robots: &[Robot], filter: Option<fn(&Robot) -> bool>) {
    let mut grid = vec![vec![0i32; WIDTH as usize]; HEIGHT as usize];
    
    for robot in robots {
        let x = robot.p.x as usize;
        let y = robot.p.y as usize;
        
        // Если есть фильтр и робот не проходит его - ставим отрицательное значение
        if let Some(f) = filter {
            if !f(robot) {
                grid[y][x] -= 1;
            } else {
                grid[y][x] += 1;
            }
        } else {
            grid[y][x] += 1;
        }
    }
    
    println!("\nField {}x{}:", WIDTH, HEIGHT);
    for row in grid {
        for count in row {
            match count {
                0 => print!(" . "),
                n if n < 0 => print!("{:2} ", n), // отрицательные числа для отфильтрованных
                n => print!("{:2} ", n),          // положительные для прошедших фильтр
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(12, part1::solve(&data));
    }
}
