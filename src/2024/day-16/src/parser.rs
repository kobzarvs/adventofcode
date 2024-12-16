pub fn parse(input: &str) -> (Vec<Vec<bool>>, (i32, i32), (i32, i32)) {
    let mut maze = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(false),
                '.' => row.push(true),
                'S' => {
                    row.push(true);
                    start = (x as i32, y as i32);
                },
                'E' => {
                    row.push(true);
                    end = (x as i32, y as i32);
                },
                _ => panic!("Неверный символ в лабиринте: {}", c),
            }
        }
        maze.push(row);
    }
    
    (maze, start, end)
}
