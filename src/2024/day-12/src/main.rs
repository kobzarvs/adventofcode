use day_12::{part1, part2, read_file};

fn main() {
    let input = read_file();

    let result = part1::run(&input);
    println!("part I: {}", result); // 1930 - 1370258

    let result = part2::run(&input);
    println!("part II: {}", result); // 1206 - ???
}

