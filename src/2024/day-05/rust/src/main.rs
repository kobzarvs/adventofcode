use day_05::{process_matrix, read_file};


fn main() {
    let input = read_file();


    let part_1 = solve_1();
    let part_2 = solve_2();

    println!("Part I: {:?}", part_1);
    println!("Part II: {:?}", part_2);
}

fn parse(input: String, matrix: &mut Vec<Vec<char>>) {
    input.lines().for_each(|line| {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    });
}

fn solve_1() -> usize {
    let mut count = 0;

    count
}

fn solve_2() -> usize {
    let mut count = 0;

    count
}
