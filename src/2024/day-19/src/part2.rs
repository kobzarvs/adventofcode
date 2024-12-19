use crate::parse;
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> i64 {
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

    // Извлечение всех паттернов из префиксного дерева, которые начинаются с префикса
    fn find_prefixes<'a>(&'a self, s: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut node = &self.root;
        let mut current_prefix = String::new();

        for ch in s.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                current_prefix.push(ch);
                node = next_node;
                if node.is_end {
                    result.push(&s[..current_prefix.len()]);
                }
            } else {
                break;
            }
        }
        result
    }
}

fn solve(patterns: &HashSet<String>, designs: &[String]) -> i64 {
    // Построим префиксное дерево из паттернов
    let mut trie = Trie::new();
    for pattern in patterns {
        trie.insert(pattern);
    }

    // количество способов собрать строку из паттернов
    fn count_ways(
        design: &str,
        trie: &Trie,
        memo: &mut HashMap<String, i64>,
    ) -> i64 {
        if let Some(&result) = memo.get(design) {
            return result;
        }
        if design.is_empty() {
            return 1;
        }

        let mut total_ways = 0;
        for prefix in trie.find_prefixes(design) {
            let remaining = &design[prefix.len()..];
            total_ways += count_ways(remaining, trie, memo);
        }
        memo.insert(design.to_string(), total_ways);
        total_ways
    }

    // кеш для хранения промежуточных результатов
    let mut memo = HashMap::new();

    // количество способов для каждого дизайна и суммируем их
    let total: i64 = designs
        .iter()
        .fold(0, |acc, design| acc + count_ways(design, &trie, &mut memo));

    total
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let (patterns, designs) = parse(input);

        assert_eq!(16, part2::solve(&patterns, &designs));
    }
}
