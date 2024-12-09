use std::cmp::max;
use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub const DIRS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn get_next_pos(pos: &(i32, i32), dir: &(i32, i32)) -> (i32, i32) {
    (pos.0 + dir.0, pos.1 + dir.1)
}

type FieldInfo = (i32, i32, i32, i32, char);
type Matrix1 = HashMap<(i32, i32), char>;
type Matrix2 = HashMap<(i32, i32), FieldInfo>;

pub fn parse(
    input: &str,
) -> (
    Matrix1,
    Matrix2,
    (i32, i32),
    (i32, i32),
) {
    let mut matrix: HashMap<(i32, i32), char> = HashMap::new();
    let mut matrix2: HashMap<(i32, i32), FieldInfo> = HashMap::new();
    let mut start_point: (i32, i32) = (-1, -1);
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    for (y, line) in input.lines().enumerate() {
        if !line.is_empty() {
            line.chars().enumerate().for_each(|(x, mut c)| {
                if c == '^' {
                    start_point = (y as i32 + 1, x as i32 + 1);
                    c = 0 as char;
                } else if c == '.' {
                    c = 0 as char;
                }
                matrix.insert((y as i32 + 1, x as i32 + 1), c);
                matrix2.insert((y as i32 + 1, x as i32 + 1), (0, 0, 0, 0, c));
                width = max(width, x as i32);
                height = max(height, y as i32);
            });
        }
    }

    (matrix, matrix2, start_point, (height + 1, width + 1))
}
