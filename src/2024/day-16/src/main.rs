use std::fs;
use std::collections::HashSet;
use day_16::labyrinth_solver::{find_all_paths, find_path_min_turns, print_path, print_path_w4};
use day_16::parser::parse;
use day_16::labyrinth_solver::find_paths_with_effective;

fn main() {
    // let input = fs::read_to_string("test.txt").expect("Ошибка чтения файла");
    // let (maze, start, end) = parse(&input);
    
    // println!("Ищем пути с эффективностью {}", target_effective);
    //
    // let paths = find_paths_with_effective(&maze, start, end, target_effective);
    //
    // if !paths.is_empty() {
    //     let mut unique_cells = HashSet::new();
    //     for (path, turns) in paths.iter() {
    //         println!("path: {:?} len: {}", path, path.len());
    //         for pos in path {
    //             unique_cells.insert(*pos);
    //         }
    //     }
    //     println!("\nВсего уникальных пройденных клеток: {}", unique_cells.len());
    // } else {
    //     println!("Пути с заданной эффективностью не найдены");
    // }

    // let input = fs::read_to_string("test.txt").expect("Ошибка чтения файла");
    // let (maze, start, end) = parse(&input);
    // let result = find_all_paths(&maze, start, end, 7036);
    // println!("Small example: {}", result);
    // 
    // let input = fs::read_to_string("test_big.txt").expect("Ошибка чтения файла");
    // let (maze, start, end) = parse(&input);
    // let result = find_all_paths(&maze, start, end, 11048);
    // println!("Big example: {}", result);

    let input = fs::read_to_string("data.txt").expect("Ошибка чтения файла");
    let (maze, start, end) = parse(&input);
    let result = find_all_paths(&maze, start, end, 111480);
    println!("Part II: {}", result);
}
