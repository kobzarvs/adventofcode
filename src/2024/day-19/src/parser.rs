use std::collections::{HashSet};

pub fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut patterns: HashSet<String> = HashSet::new();
    let mut designs: Vec<String> = vec![];

    input.lines().enumerate().for_each(|(row, line)| {
        if row == 0 {
            line.split(",")
                .for_each(|s| {
                    patterns.insert(s.trim().to_string());
                });
        } else if !line.is_empty() {
            designs.push(line.to_string());
        }
    });

    (patterns, designs)
}
