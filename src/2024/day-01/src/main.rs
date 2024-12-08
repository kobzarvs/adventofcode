use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};


#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();
    divan::main();


    let filename = get_filename();

    println!("Input file: {:?}", filename);

    let input = fs::read_to_string(&filename).unwrap();

    let (mut column1, mut column2) = parse(input);
    column1.sort_unstable();
    column2.sort_unstable();

    // Part I
    let total_distance = solve_1(&mut column1, &mut column2);
    println!("Part I (сумма расстояний): {}", total_distance);

    // Part II
    let sum_vector3 = solve_2(&mut column1, &mut column2);
    println!("Part II (степень сходства): {}", sum_vector3);

    Ok(())
}

fn solve_2(column1: &mut Vec<i32>, column2: &mut Vec<i32>) -> i32 {
    // Группируем второй вектор, подсчитывая количество каждого числа
    let mut count_map: HashMap<i32, usize> = HashMap::new();
    for num in column2 {
        *count_map.entry(*num).or_insert(0) += 1;
    }

    // Вектор с повторами из второго столбца
    let mut vector3 = Vec::new();

    // Проходим по первому столбцу
    for num in column1 {
        if let Some(&count) = count_map.get(&num) {
            vector3.push(*num * count as i32);
        }
    }

    // Суммируем значения степеней сходства
    vector3.iter().sum()
}

fn solve_1(column1: &mut Vec<i32>, mut column2: &mut Vec<i32>) -> i32 {
    // Находим расстояния между соответствующими элементами
    let total_distance: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    total_distance
}

fn parse(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut column1 = vec![];
    let mut column2 = vec![];

    for line in input.lines() {
        let mut numbers = line.split_whitespace();

        column1.push(numbers.next().unwrap().parse::<i32>().unwrap());
        column2.push(numbers.next().unwrap().parse::<i32>().unwrap());
    }
    (column1, column2)
}

pub fn get_filename() -> PathBuf {
    let args: Vec<String> = env::args().collect();

    let filename = args
        .get(1)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| PathBuf::from("test.txt"));
    filename
}
