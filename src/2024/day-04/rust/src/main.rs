use std::path::PathBuf;
use std::{env, fs};

static WINDOW_SIZE: usize = 4;
static XMAS: &str = "XMAS";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| PathBuf::from("test.txt"));
    println!("Input file: {:?}", filename);
    let input = fs::read_to_string(filename).expect("Unable to read file");

    //-------------------------------------------------------------------------------//

    let mut matrix: Vec<Vec<char>> = vec![];

    input.lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        println!("{:?}", row);
        matrix.push(row);
    });
    println!();

    let part_1 = find_xmas(&matrix);
    println!("Part 1: {:?}", part_1);
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| matrix[row][col]).collect())
        .collect()
}

fn find_xmas(grid: &Vec<Vec<char>>) -> i32 {
    let mut matrix = grid.iter().cloned().collect::<Vec<_>>();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut counts = 0;
    let mut new_matrix: Vec<Vec<String>> = vec![vec![" ".to_string(); cols]; rows];
    let mut visited_words: Vec<Vec<(usize, usize)>> = vec![vec![]; 2];

    fn yx(n: usize, y: usize, x: usize) -> (usize, usize) {
        if n == 0 {
            (y, x)
        } else {
            (x, y)
        }
    }

    for n in 0..2 {
        println!("N: {}", n);

        process_matrix(&matrix, (WINDOW_SIZE, WINDOW_SIZE), |window, y, x| {
            println!("{}:{}", y, x);
            for (ry, row) in window.iter().enumerate() {
                if visited_words[n].contains(&(y + ry, x)) {
                    continue;
                }
                println!("{:?}", row);
                let word = String::from_iter(row);
                let word_rev = String::from_iter(row.iter().rev());
                if word == XMAS {
                    counts += 1;
                    println!("{} : {:?} == {} {} {}", counts, row, XMAS, word, counts);
                    word.chars().enumerate().for_each(|(rx, c)| {
                        let sym = new_matrix[yx(n, y + ry, x + rx).0][yx(n, y + ry, x + rx).1].clone();
                        if sym == " " {
                            new_matrix[yx(n, y + ry, x + rx).0][yx(n, y + ry, x + rx).1] = c.to_string();
                        } else {
                            new_matrix[yx(n, y + ry, x + rx).0][yx(n, y + ry, x + rx).1] = format!("[{}]", sym);
                        }
                    });
                }
                if word_rev == XMAS {
                    counts += 1;
                    println!("{} : {:?} == {} {} {}", counts, row, XMAS, word_rev, counts);
                    word.chars().enumerate().for_each(|(rx, c)| {
                        let sym = new_matrix[yx(n, y + ry, x + rx).0][yx(n, y + ry, x + rx).1].clone();
                        if sym == " " {
                            new_matrix[yx(n, y + ry, x + rx).0][yx(n, y + ry, x + rx).1] = c.to_string();
                        } else {
                            new_matrix[yx(n, y + ry, x + rx).0][yx(n, y + ry, x + rx).1] = format!("[{}]", sym);
                        }
                    });
                }
                visited_words[n].push((y + ry, x));
            }

            if n == 1 {
                return;
            }

            // check diagonals
            let mut diag_1: Vec<char> = vec![];
            let mut diag_2: Vec<char> = vec![];
            for i in 0..WINDOW_SIZE {
                diag_1.push(window[i][i]);
                diag_2.push(window[i][WINDOW_SIZE - i - 1]);
            }
            let word = String::from_iter(diag_1.iter().cloned());
            let word_rev = String::from_iter(diag_1.iter().rev());
            let word2 = String::from_iter(diag_2.iter().cloned());
            let word2_rev = String::from_iter(diag_2.iter().rev());
            if word == XMAS {
                counts += 1;
                println!("diag {} : {:?} == {:?} {}", counts, XMAS, word, counts);
                for i in 0..WINDOW_SIZE {
                    new_matrix[yx(n,y+i,x+i).0][yx(n,y+i,x+i).1] = window[i][i].to_string();
                }
            }
            if word_rev == XMAS {
                counts += 1;
                println!("rev diag {} : {:?} == {:?} {}", counts, XMAS, word_rev, counts);
                for i in 0..WINDOW_SIZE {
                    new_matrix[yx(n,y+i,x+i).0][yx(n,y+i,x+i).1] = window[i][i].to_string();
                }
            }
            if word2 == XMAS {
                counts += 1;
                println!("diag {} : {:?} == {:?} {}", counts, XMAS, word2, counts);
                for i in 0..WINDOW_SIZE {
                    new_matrix[yx(n,y+i,x + WINDOW_SIZE - i - 1).0][yx(n,y+i,x + WINDOW_SIZE - i - 1).1] = window[i][WINDOW_SIZE - i - 1].to_string();
                }
            }
            if word2_rev == XMAS {
                counts += 1;
                println!("rev diag {} : {:?} == {:?} {}", counts, XMAS, word2_rev, counts);
                for i in 0..WINDOW_SIZE {
                    new_matrix[yx(n,y+i,x + WINDOW_SIZE - i - 1).0][yx(n,y+i,x + WINDOW_SIZE - i - 1).1] = window[i][WINDOW_SIZE - i - 1].to_string();
                }
            }
        });

        if n == 0 {
            matrix = transpose(&matrix.clone());
        }
    }

    grid.iter().for_each(|row| {
        println!("{:?}", row);
    });
    println!();
    new_matrix.iter().for_each(|row| {
        println!("{:?}", row);
    });

    counts
}

fn process_matrix<F>(matrix: &Vec<Vec<char>>, size: (usize, usize), mut processor: F)
where
    F: FnMut(&Vec<Vec<char>>, usize, usize)
{
    for y in 0..matrix.len() - size.0 + 1 {
        for x in 0..matrix[0].len() - size.1 + 1 {
            let mut window = vec![];
            for i in 0..size.0 {
                window.push(matrix[y + i][x..x + size.1].to_vec());
            }

            processor(&window, y, x);
        }
    }
}
