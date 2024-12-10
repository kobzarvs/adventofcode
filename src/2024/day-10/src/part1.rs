use crate::{Pos, Topo, DIRECTIONS};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;


fn try_step(from: &Pos, map: &Topo, result: &Arc<Mutex<i32>>, visited: &Arc<Mutex<HashSet<Pos>>>) {
    if visited.lock().unwrap().contains(&from) {
        return;
    } else {
        visited.lock().unwrap().insert(from.clone());
    }

    if *map.get(&*from).unwrap() == 9 {
        *result.lock().unwrap() += 1;
    }

    DIRECTIONS
        .into_par_iter()
        .filter(|dir| {
            map.get(&(*from + *dir))
                .is_some_and(|high| *high - *map.get(&from).unwrap() == 1)
        })
        .for_each(|dir| {
            try_step(&(*from + dir), &map, result, visited);
        });
}

pub fn solve(src_map: &Topo, start_points: &[Pos]) -> i32 {
    let result = Arc::new(Mutex::new(0));

    start_points
        .par_iter()
        .for_each(|start| {
            let visited: Arc<Mutex<HashSet<Pos>>> = Arc::new(Mutex::new(HashSet::new()));
            try_step(start, &src_map, &result, &visited)
        });

    let result = *result.lock().unwrap();
    result
}
