use crate::{parse, Machine};
use geo::{coord, Coord};
use std::io;
use std::io::Write;

pub fn run(input: &str) -> i64 {
    let machines = parse(&input);
    solve(&machines)
}

const N: f64 = 10_000_000_000_000.0;

fn solve(machines: &Vec<Machine<Coord>>) -> i64 {
    machines
        .iter()
        .map(|machine| {
            let a = machine.a; // *3
            let b = machine.b; // *1
            let target = machine.target + coord! {x: N, y: N };

            // Решаем систему уравнений:
            // a1*x + b1*y = c1
            // a2*x + b2*y = c2

            let det = a.x * b.y - a.y * b.x;
            if det == 0.0 {
                // println!("Нет решения или бесконечно много решений");
                io::stdout().flush().unwrap();
                return 0;
            }

            // метод Крамера
            let a_moves = ((target.x * b.y - target.y * b.x) / det).round();
            let b_moves = ((target.y * a.x - target.x * a.y) / det).round();

            // проверка решения на целочисленность
            if a_moves * det != (target.x * b.y - target.y * b.x)
                || b_moves * det != (a.x * target.y - a.y * target.x)
            {
                // println!(
                //     "- Решение не является целочисленное: {:?}",
                //     (a_moves, b_moves)
                // );
                io::stdout().flush().unwrap();
                return 0;
            } else {
                return 3 * (a_moves as i64) + b_moves as i64;
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(875318608908, part2::solve(&data));
    }
}
