#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use crate::{get_next_pos, parse, DIRS};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (matrix, _matrix2, start_point, _size) = parse(input);

    let mut moves = DIRS.iter().cycle();
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut pos = start_point;
    let mut dir = moves.next().unwrap();
    let mut steps = 1;

    while let Some(&place) = matrix.get(&get_next_pos(&pos, dir)) {
        if place == '#' {
            dir = moves.next().unwrap();
        }

        if !visited.contains(&pos) {
            steps += 1;
        }

        visited.insert((pos.0, pos.1));
        pos = (pos.0 + dir.0, pos.1 + dir.1);

        if steps > 100000 {
            break;
        }
    }

    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
