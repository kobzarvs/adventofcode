use day_19::{part1, part2, read_file};

fn main() {
    let input = read_file();

    let result = part1::run(&input);
    println!("part I: {:?}", result);

    let result = part2::run(&input);
    println!("part II: {}", result);
}
