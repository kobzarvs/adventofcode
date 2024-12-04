use day_04::{process_matrix, read_file};

static XMAS: &str = "XMAS";
static SAMX: &str = "SAMX";

fn main() {
    let input = read_file();

    let mut matrix: Vec<Vec<char>> = vec![];

    parse(input, &mut matrix);

    let part_1 = solve_1(&matrix);
    let part_2 = solve_2(&matrix);

    println!("Part I: {:?}", part_1); // 2427
    println!("Part II: {:?}", part_2); // 1900
}

fn parse(input: String, matrix: &mut Vec<Vec<char>>) {
    input.lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    });
}

fn solve_1(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let mut visited_horizont: Vec<(usize, usize)> = vec![];
    let mut visited_vertical: Vec<(usize, usize)> = vec![];
    let size = 4;

    process_matrix(&matrix.clone(), XMAS.len(), |window, y, x| {
        for (ry, row) in window.iter().enumerate() {
            if !visited_horizont.contains(&(y + ry, x)) {
                let word = String::from_iter(row);
                if word == XMAS || word == SAMX {
                    count += 1;
                }
                visited_horizont.push((y + ry, x));
            }

            if !visited_vertical.contains(&(y, x + ry)) {
                let vertical_word: Vec<char> = (0..size).map(|i| window[i][ry]).collect();
                let word = vertical_word.into_iter().collect::<String>();
                if word == XMAS || word == SAMX {
                    count += 1;
                }
                visited_vertical.push((y, x + ry));
            }
        }

        // check diagonals
        let diag_1: String = (0..size).map(|i| window[i][i]).collect();
        let diag_2: String = (0..size).map(|i| window[i][size - i - 1]).collect();

        count += [diag_1, diag_2].iter()
            .filter(|&d| d == XMAS || d == SAMX)
            .count() as i32;
    });

    count
}

fn solve_2(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let size = 3;

    process_matrix(&matrix, size, |window, _y, _x| {
        let mut diag_left_top: Vec<char> = vec![];
        let mut diag_left_bottom: Vec<char> = vec![];

        for i in 0..size {
            diag_left_top.push(window[i][i]);
            diag_left_bottom.push(window[size - i - 1][i]);
        }

        let pass = [diag_left_bottom, diag_left_top].iter().all(|diag| {
            let word = diag.into_iter().collect::<String>();
            ["MAS", "SAM"].contains(&word.as_str())
        });

        if pass {
            count += 1;
        }
    });

    count
}
