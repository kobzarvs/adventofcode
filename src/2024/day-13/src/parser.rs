use geo::{coord, Coord};
use num_complex::Complex;
use regex::Regex;

pub type Complex32 = Complex<i64>;

#[derive(Debug, Clone)]
pub struct Machine<T> {
    pub a: T,
    pub b: T,
    pub target: T,
}

pub fn parse_to_complex(input: &str) -> Vec<Machine<Complex32>> {
    let re = Regex::new(r".*: .[=+](?<x>\d+), [XY][+=](?<y>\d+)\s*").unwrap();
    let mut machines: Vec<Machine<Complex32>> = vec![];

    let mut a = Complex32::default();
    let mut b = Complex32::default();
    let mut target = Complex32::default();

    input.lines().enumerate().for_each(|(row, line)| {
        if line.is_empty() {
            return;
        }
        if re.is_match(line) {
            let pos = re.captures(line).unwrap();
            let x: i64 = pos.name("x").unwrap().as_str().parse().unwrap();
            let y: i64 = pos.name("y").unwrap().as_str().parse().unwrap();
            // Button A: X+29, Y+71
            // Button B: X+52, Y+31
            // Prize: X=5388, Y=4716
            match row % 4 {
                0 => a = Complex32::new(x, y),
                1 => b = Complex32::new(x, y),
                2 => {
                    target = Complex32::new(x, y);
                    machines.push(Machine { a, b, target });
                }
                _ => {}
            }
        }
    });

    machines
}

pub fn parse(input: &str) -> Vec<Machine<Coord>> {
    let re = Regex::new(r".*: .[=+](?<x>\d+), [XY][+=](?<y>\d+)\s*").unwrap();
    let mut machines: Vec<Machine<Coord>> = vec![];

    let mut a = Coord::default();
    let mut b = Coord::default();
    let mut target = Coord::default();

    input.lines().enumerate().for_each(|(row, line)| {
        if line.is_empty() {
            return;
        }
        if re.is_match(line) {
            let pos = re.captures(line).unwrap();
            let x: f64 = pos.name("x").unwrap().as_str().parse().unwrap();
            let y: f64 = pos.name("y").unwrap().as_str().parse().unwrap();

            match row % 4 {
                0 => a = coord! {x: x, y: y},
                1 => b = coord! {x: x, y: y},
                2 => {
                    target = coord! {x: x, y: y};
                    machines.push(Machine::<Coord> { a, b, target });
                }
                _ => {}
            }
        }
    });

    machines
}
