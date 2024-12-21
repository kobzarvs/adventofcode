use day_18::{part1, part2, path_finding, read_file};

fn main() {
    let input = read_file();
    
    println!("{}", input);
    let result = part1::run(&input);
    println!("part I: {}", result);

    // let result = part2::run(&input);
    // println!("part II: {}", result);
    if let Some(pos) = path_finding() {
        println!("part II: {}, {}", pos.0, pos.1);
    } else {
        println!("part II: no solution");
    }
}
