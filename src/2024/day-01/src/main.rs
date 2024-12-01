use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{PathBuf};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| PathBuf::from("test.txt"));

    println!("Input file: {:?}", filename);

    let input = File::open(&filename)?;
    let reader = io::BufReader::new(input);

    // Векторы для хранения столбцов
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        if numbers.len() == 2 {
            column1.push(numbers[0]);
            column2.push(numbers[1]);
        }
    }

    // Сортируем колонки
    column1.sort_unstable();
    column2.sort_unstable();

    // Находим расстояния между соответствующими элементами
    let distances: Vec<i32> = column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();

    // Суммируем расстояния
    let total_distance: i32 = distances.iter().sum();

    // Part I
    println!("Part I (сумма расстояний): {}", total_distance);

    // Группируем второй вектор, подсчитывая количество каждого числа
    let mut count_map: HashMap<i32, usize> = HashMap::new();
    for &num in &column2 {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Вектор с повторами из второго столбца
    let mut vector3 = Vec::new();

    // Проходим по первому столбцу
    for &num in &column1 {
        if let Some(&count)  = count_map.get(&num) {
            vector3.push(num * count as i32);
        }
    }

    // Суммируем значения степеней сходства
    let sum_vector3: i32 = vector3.iter().sum();

    // Part II
    println!("Part II (степень сходства): {}", sum_vector3);

    Ok(())
}
