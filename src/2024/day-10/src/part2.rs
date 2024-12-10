use crate::{Pos, Topo, DIRECTIONS};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn try_step(from: &Pos, map: &Topo, result: &Arc<Mutex<i32>>) {
    // if end point
    if *map.get(from).unwrap() == 9 {
        *result.lock().unwrap() += 1;
        return;
    }

    DIRECTIONS
        .into_par_iter()
        .filter(|dir| {
            map.get(&(*from + *dir))
                .is_some_and(|high| *high - *map.get(from).unwrap() == 1)
        })
        .for_each(|dir| {
            try_step(&(*from + dir), map, result);
        });
}

pub fn solve(src_map: &Topo, start_points: &[Pos]) -> i32 {
    let result = Arc::new(Mutex::new(0));

    start_points
        .par_iter()
        .for_each(|start| {
            try_step(start, src_map, &result);
        });

    let result = *result.lock().unwrap();
    result
}


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::{parse, part2};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let (map, start_points) = parse(input);

        assert_eq!(81, part2::solve(&map, &start_points));
    }

    #[test]
    fn try_step() {
        let input = include_str!("../test.txt");
        let (map, start_points) = parse(input);
        let result = Arc::new(Mutex::new(0));

        part2::try_step(&start_points[0], &map, &result);

        assert_eq!(20, *result.lock().unwrap());
    }
}
