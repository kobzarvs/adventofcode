use std::io::{self, Write};
use std::collections::HashMap;

pub fn run(input: &str) -> u64 {
    solve(input)
}


fn process_number(num: u64, cache: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    if let Some(cached) = cache.get(&num) {
        return cached.clone();
    }

    let mut result = Vec::with_capacity(2);
    
    if num == 0 {
        result.push(1);
    } else {
        let digit_count = if num == 0 { 1 } else { (num as f64).log10().floor() as u32 + 1 };
        
        if digit_count % 2 == 0 {
            let divisor = 10_u64.pow(digit_count / 2);
            let left = num / divisor;
            let right = num % divisor;
            
            result.push(left);
            result.push(if right == 0 { 0 } else { right });
        } else {
            result.push(num * 2024);
        }
    }

    cache.insert(num, result.clone());
    result
}

fn solve(input: &str) -> u64 {
    // Начальное состояние: считаем количество каждого числа
    let mut current_state: HashMap<u64, u64> = HashMap::new();
    for num in input.split_whitespace().map(|n| n.parse::<u64>().unwrap()) {
        *current_state.entry(num).or_default() += 1;
    }

    // println!("Initial state: {} unique numbers", current_state.len());
    
    let mut cache: HashMap<u64, Vec<u64>> = HashMap::new();

    for _i in 0..25 {
        // print!("\rstep: {} unique numbers: {}", _i, current_state.len());
        // io::stdout().flush().unwrap();

        let mut new_state: HashMap<u64, u64> = HashMap::new();
        
        // Обрабатываем каждое уникальное число
        for (&num, &count) in current_state.iter() {
            let processed = process_number(num, &mut cache);
            
            // Добавляем результаты с учетом количества исходного числа
            for &new_num in processed.iter() {
                *new_state.entry(new_num).or_default() += count;
            }
        }
        
        if new_state.is_empty() {
            // println!("\nNo more numbers to process");
            break;
        }
        
        current_state = new_state;
    }

    // println!();
    
    // Суммируем все количества
    current_state.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        assert_eq!(55312, part1::solve(input));
    }
}
