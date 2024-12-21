use crate::parse;
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> i32 {
    let (patterns, designs) = parse(input);
    solve(&patterns, &designs)
}

// Структура для узла префиксного дерева
#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool, // Указывает, является ли узел концом паттерна
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    // Добавление строки в префиксное дерево
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        node.is_end = true;
    }

    // Проверяем, можно ли разделить строку на паттерны из дерева
    fn can_construct(&self, s: &str, memo: &mut HashMap<String, bool>) -> bool {
        if let Some(&result) = memo.get(s) {
            return result; // Возвращаем сохраненный результат
        }
        if s.is_empty() {
            return true; // Пустую строку можно составить
        }

        let mut node = &self.root;
        for (i, ch) in s.chars().enumerate() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
                if node.is_end {
                    // Если достигли конца паттерна, проверяем остаток строки
                    if self.can_construct(&s[i + 1..], memo) {
                        memo.insert(s.to_string(), true);
                        return true;
                    }
                }
            } else {
                break;
            }
        }

        memo.insert(s.to_string(), false);
        false
    }
}

fn solve(patterns: &HashSet<String>, designs: &[String]) -> i32 {
    // Построим префиксное дерево из паттернов
    let mut trie = Trie::new();
    for pattern in patterns {
        trie.insert(pattern);
    }

    // мемоизация
    let mut memo = HashMap::new();

    // количество строк, которые могут быть составлены из паттернов
    let count = designs
        .iter()
        .filter(|design| trie.can_construct(design, &mut memo))
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
