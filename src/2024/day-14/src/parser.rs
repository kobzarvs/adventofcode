use geo::coord;
use regex::Regex;
use crate::models::Robot;

pub fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    
    input.lines().map(|line| {
        let groups = re.captures(line).unwrap();
        let px: i32 = groups.name("px").unwrap().as_str().parse().unwrap();
        let py: i32 = groups.name("py").unwrap().as_str().parse().unwrap();
        let vx: i32 = groups.name("vx").unwrap().as_str().parse().unwrap();
        let vy: i32 = groups.name("vy").unwrap().as_str().parse().unwrap();
        Robot {
            p: coord! {x: px, y: py},
            v: coord! {x: vx, y: vy},
        }
    }).collect()
}
