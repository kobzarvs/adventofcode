use crate::{parse, Machine};
use geo::line_intersection::line_intersection;
use geo::LineIntersection::SinglePoint;
use geo::{Coord, Line};


pub fn run(input: &str) -> i64 {
    let machines = parse(input);
    solve(&machines)
}

fn find_intersection(start1: Coord, end1: Coord, start2: Coord, end2: Coord) -> Option<Coord> {
    let line_1 = Line::new(start1, end1);
    let line_2 = Line::new(start2, end2);
    let result = line_intersection(line_1, line_2);

    match result {
        Some(SinglePoint { intersection, .. }) => Some(intersection),
        _ => None,
    }
}

fn solve(machines: &Vec<Machine<Coord>>) -> i64 {
    let result = machines
        .iter()
        .map(|machine| {
            let a = machine.a; // *3
            let b = machine.b; // *1
            let target = machine.target;

            if let Some(cross_point) =
                find_intersection(Coord::default(), b * 100.0, target, target + a * -100.0)
            {
                if (target.x - cross_point.x) as i64 % a.x as i64 == 0
                    && (target.y - cross_point.y) as i64 % a.y as i64 == 0
                {
                    return (cross_point.x / b.x + 3.0 * (target.x - cross_point.x) / a.x) as i64;
                }
            }

            0
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(480, part1::solve(&data));
    }
}
