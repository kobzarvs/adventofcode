use day_05::read_file;
use regex::Regex;
use std::cmp::{Ordering, PartialEq};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file();

    let (rules, updates) = parse(&input)?;
    let index = create_index(&rules);

    let part_1 = solve_1(&index, &updates);
    let part_2 = solve_2(&index, &updates);

    println!("Part I: {:?}", part_1);
    println!("Part II: {:?}", part_2);

    Ok(())
}

fn parse(input: &str) -> Result<(Vec<(usize, usize)>, Vec<Vec<usize>>), regex::Error> {
    let re = Regex::new(r"(?<page_1>\d+)\|(?<page_2>\d+)")?;

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    for line in input.lines() {
        if re.is_match(line) {
            let rule = re.captures(line).unwrap();
            let page_1: usize = rule.name("page_1").unwrap().as_str().parse().unwrap();
            let page_2: usize = rule.name("page_2").unwrap().as_str().parse().unwrap();
            rules.push((page_1, page_2));
        } else if !line.is_empty() {
            let pages: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
            updates.push(pages);
        }
    }

    Ok((rules, updates))
}

fn solve_1(index: &HashMap<usize, Vec<(usize, Order)>>, updates: &Vec<Vec<usize>>) -> usize {
    let valid_updates = get_updates(index, updates, true);

    valid_updates
        .iter()
        .fold(0, |acc, &update| acc + update[update.len() / 2])
}

fn solve_2(index: &HashMap<usize, Vec<(usize, Order)>>, updates: &Vec<Vec<usize>>) -> usize {
    let mut invalid_updates = get_updates(index, updates, false);

    let sum = invalid_updates
        .iter_mut()
        .map(|invalid_update| {
            let mut invalid_update = invalid_update.clone();

            invalid_update.sort_by(|&a, &b| {
                index
                    .get(&a)
                    .map_or(Ordering::Equal, |order_for_a| {
                        if order_for_a
                            .iter()
                            .any(|(num, order)| *num == b && *order == Order::Right)
                        {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
            });

            invalid_update[invalid_update.len() / 2]
        })
        .sum();

    sum
}

#[derive(Debug, Copy, Clone)]
enum Order {
    Left,
    Right,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Order::Left, Order::Left) => true,
            (Order::Right, Order::Right) => true,
            _ => false,
        }
    }
}

fn create_index(rules: &[(usize, usize)]) -> HashMap<usize, Vec<(usize, Order)>> {
    let mut index: HashMap<usize, Vec<(usize, Order)>> = HashMap::new();

    for &(x, y) in rules {
        index.entry(x).or_default().push((y, Order::Right));
        index.entry(y).or_default().push((x, Order::Left));
    }

    index
}

fn get_updates<'a>(
    index: &'a HashMap<usize, Vec<(usize, Order)>>,
    updates: &'a [Vec<usize>],
    is_valid: bool,
) -> Vec<&'a Vec<usize>> {
    updates
        .iter()
        .filter(|&update| {
            let matches_order = update.windows(2).all(|pair_pages| {
                let &[first, second] = pair_pages else {
                    return false;
                };

                index.get(&first).map_or(false, |orders| {
                    orders
                        .iter()
                        .any(|(other_page, order)| *other_page == second && *order == Order::Right)
                })
            });

            is_valid == matches_order
        })
        .collect()
}
