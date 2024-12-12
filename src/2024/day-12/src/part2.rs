//! Решение второй части задачи 12 дня с оптимизацией.
//!
//! Задача заключается в подсчете суммы произведений площади каждого региона на количество
//! его уникальных непрерывных границ. Регион - это группа одинаковых символов, соединенных
//! по горизонтали или вертикали.

use std::collections::{HashMap, HashSet};
use num_complex::Complex;
use rayon::prelude::*;
use crate::{parse_part2};

/// Тип для работы с комплексными координатами
pub type Complex64 = Complex<i32>;

/// Запускает решение для входных данных
///
/// # Arguments
///
/// * `input` - Строка с входными данными
///
/// # Returns
///
/// Сумму произведений площади каждого региона на количество его уникальных границ
pub fn run(input: &str) -> i32 {
    let map = parse_part2(input);
    solve(&map)
}

/// Основная функция решения задачи
///
/// # Arguments
///
/// * `complex_map` - Карта в виде HashMap, где ключ - комплексная координата, значение - символ
///
/// # Returns
///
/// Результат решения задачи
fn solve(complex_map: &HashMap<Complex64, char>) -> i32 {
    // Находим все регионы
    let mut regions: HashMap<Complex64, i32> = HashMap::new();
    let mut areas: HashMap<i32, i32> = HashMap::new();
    let mut i = 0;

    for (&pos, &c) in complex_map {
        if regions.contains_key(&pos) {
            continue;
        }
        get_all_region_points(pos, c, i, complex_map, &mut regions, &mut areas);
        i += 1;
    }

    let regions_count = i;

    // Подсчитываем результат для части 2
    (0..regions_count)
        .into_par_iter() // Параллельный итератор
        .map(|region_id| {
            let area = areas.get(&region_id).copied().unwrap_or(0);
            let sides = get_sides(region_id, &regions);
            area * sides
        })
        .sum()
}

/// Итеративно находит все точки, принадлежащие одному региону.
///
/// # Arguments
///
/// * `pos` - Текущая позиция
/// * `c` - Символ региона
/// * `region_id` - Идентификатор региона
/// * `map` - Исходная карта
/// * `regions` - Карта регионов (заполняется в процессе работы)
/// * `areas` - Хранит площади регионов
fn get_all_region_points(
    pos: Complex64,
    c: char,
    region_id: i32,
    map: &HashMap<Complex64, char>,
    regions: &mut HashMap<Complex64, i32>,
    areas: &mut HashMap<i32, i32>,
) {
    let mut stack = vec![pos];
    let mut area = 0;

    while let Some(current) = stack.pop() {
        if regions.contains_key(&current) {
            continue;
        }
        if map.get(&current).copied() != Some(c) {
            continue;
        }

        regions.insert(current, region_id);
        area += 1;

        for d in 0..4 {
            stack.push(current + Complex64::i().powi(d));
        }
    }

    areas.insert(region_id, area);
}

/// Подсчитывает количество уникальных непрерывных границ региона
///
/// # Arguments
///
/// * `region_id` - Идентификатор региона
/// * `regions` - Карта регионов
///
/// # Returns
///
/// Количество уникальных границ
fn get_sides(region_id: i32, regions: &HashMap<Complex64, i32>) -> i32 {
    let mut pieces = HashSet::new();

    // Собираем все границы
    for (&pos, &id) in regions {
        if id != region_id {
            continue;
        }

        for d in 0..4 {
            let next = pos + Complex64::i().powi(d);
            if regions.get(&next).copied() != Some(region_id) {
                pieces.insert((pos, Complex64::i().powi(d)));
            }
        }
    }

    let mut unique_sides = HashSet::new();

    // Объединяем границы в непрерывные линии
    for &(pos, dir) in &pieces {
        let mut side = (pos, pos, dir);
        loop {
            let new_left = side.0 - dir * Complex64::i();
            let new_right = side.1 + dir * Complex64::i();

            let new_side = (
                if pieces.contains(&(new_left, dir)) { new_left } else { side.0 },
                if pieces.contains(&(new_right, dir)) { new_right } else { side.1 },
                dir
            );

            if new_side == side {
                break;
            }
            side = new_side;
        }
        unique_sides.insert(side);
    }

    unique_sides.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = include_str!("../test.txt");
        let data = parse_part2(input);
        assert_eq!(1206, solve(&data));
    }
}
