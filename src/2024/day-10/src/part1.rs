use crate::{Pos, Topo, DIRECTIONS};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

fn try_step(from: &Pos, map: &Topo, result: &Arc<Mutex<i32>>, visited: &Arc<Mutex<HashSet<Pos>>>) {
    if visited.lock().unwrap().contains(from) {
        return;
    } else {
        visited.lock().unwrap().insert(*from);
    }

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
            try_step(&(*from + dir), map, result, visited);
        });
}

pub fn solve(src_map: &Topo, start_points: &[Pos]) -> i32 {
    let result = Arc::new(Mutex::new(0));

    start_points.par_iter().for_each(|start| {
        let visited = Arc::new(Mutex::new(HashSet::new()));
        try_step(start, src_map, &result, &visited)
    });

    let result = *result.lock().unwrap();
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use crate::{parse, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let (map, start_points) = parse(input);

        assert_eq!(36, part1::solve(&map, &start_points));
    }

    #[test]
    fn try_step() {
        let input = include_str!("../test.txt");
        let (map, start_points) = parse(input);
        let result = Arc::new(Mutex::new(0));
        let visited = Arc::new(Mutex::new(HashSet::new()));
        part1::try_step(&start_points[0], &map, &result, &visited);

        assert_eq!(5, *result.lock().unwrap());
    }
}
