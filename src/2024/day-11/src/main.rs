use day_11::{part1, part2, read_file};

fn main() {
    let input = read_file();

    let result = part1::run(&input);
    println!("part I: {}", result); // 55312 - 202019

    let result = part2::run(&input);
    println!("part II: {}", result); // 65601038650482 - 239321955280205
}

