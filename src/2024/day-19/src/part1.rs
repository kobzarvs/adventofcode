use crate::parse;
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> i32 {
    let (patterns, designs) = parse(input);
    solve(&patterns, &designs)
}

fn solve(patterns: &HashSet<String>, designs: &[String]) -> i32 {
    // проверяем, можно ли составить строку из паттернов
    fn can_construct(
        design: &str,
        patterns: &HashSet<String>,
        memo: &mut HashMap<String, bool>,
    ) -> bool {
        // пустая строка может быть составлена всегда
        if design.is_empty() {
            return true;
        }

        // если уже вычисляли для данной строки, возвращаем сохраненное значение
        if let Some(&result) = memo.get(design) {
            return result;
        }

        // проверяем, можем ли разделить строку на паттерн + остаток
        for pattern in patterns {
            if design.starts_with(pattern) {
                let remaining = &design[pattern.len()..];
                if can_construct(remaining, patterns, memo) {
                    memo.insert(design.to_string(), true);
                    return true;
                }
            }
        }

        // ничего не найдено
        memo.insert(design.to_string(), false);
        false
    }

    // мемоизация
    let mut memo = HashMap::new();

    // количество дизайнов, которые можно составить
    let count = designs
        .iter()
        .filter(|design| can_construct(design, patterns, &mut memo))
        .count();

    count as i32
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let (patterns, designs) = parse(input);

        assert_eq!(6, part1::solve(&patterns, &designs));
    }
}
