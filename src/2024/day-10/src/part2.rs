use crate::{Pos, Topo, DIRECTIONS};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn try_step(from: &Pos, map: &Topo, result: &Arc<Mutex<i32>>) {
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
            try_step(&(*from + dir), &map, result);
        });
}

pub fn solve(src_map: &Topo, start_points: &[Pos]) -> i32 {
    let result = Arc::new(Mutex::new(0));

    start_points
        .par_iter()
        .for_each(|start| {
            try_step(start, &src_map, &result);
        });

    let result = *result.lock().unwrap();
    result
}
