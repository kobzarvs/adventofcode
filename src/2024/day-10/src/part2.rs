use crate::{Pos, Topo, DIRECTIONS};
use std::collections::HashSet;


fn try_step(from: &Pos, map: &Topo, visited: &mut HashSet<Pos>, paths: &mut i32) {
    if *map.get(from).unwrap() == 9 {
        *paths += 1;
        return;
    }

    visited.insert(*from);

    for dir in DIRECTIONS.iter() {
        let next_pos = *from + *dir;

        if map
            .get(&next_pos)
            .is_some_and(|high| *high - *map.get(from).unwrap() == 1)
            && !visited.contains(&next_pos)
        {
            try_step(&next_pos, map, visited, paths);
        }
    }

    visited.remove(from);
}

pub fn solve(src_map: &Topo, start_points: &[Pos]) -> i32 {
    let mut total_paths = 0;

    for start in start_points {
        let mut visited = HashSet::new();
        let mut paths = 0;
        try_step(start, src_map, &mut visited, &mut paths);
        total_paths += paths;
    }

    total_paths
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2, Pos};
    use std::collections::{HashSet, HashMap};

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
        let mut visited = std::collections::HashSet::new();
        let mut paths = 0;

        part2::try_step(&start_points[0], &map, &mut visited, &mut paths);

        assert_eq!(20, paths);
    }

    #[test]
    fn test_try_step() {
        let mut map = HashMap::new();
        map.insert(Pos{x:0, y:0}, 0);
        map.insert(Pos{x:0, y:1}, 1);
        map.insert(Pos{x:0, y:2}, 2);
        map.insert(Pos{x:0, y:3}, 3);
        map.insert(Pos{x:0, y:4}, 4);
        map.insert(Pos{x:0, y:5}, 5);
        map.insert(Pos{x:0, y:6}, 6);
        map.insert(Pos{x:0, y:7}, 7);
        map.insert(Pos{x:0, y:8}, 8);
        map.insert(Pos{x:0, y:9}, 9);

        let start = Pos{x:0, y:0};
        let mut visited = HashSet::new();
        let mut paths = 0;

        part2::try_step(&start, &map, &mut visited, &mut paths);

        assert_eq!(paths, 1);
    }
}
