use std::env;
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| PathBuf::from("test.txt"));
    println!("Input file: {:?}", filename);
    let input = fs::read_to_string(filename).expect("Unable to read file");

    //-------------------------------------------------------------------------------//

    let reports = get_reports(&input);

    let safe_reports = reports
        .iter()
        .filter(|report| is_safe_report(&report, false));

    println!("Part I: {:?}", safe_reports.count());

    let safe_reports = reports
        .iter()
        .filter(|report| is_safe_report(&report, true));

    println!("Part II: {:?}", safe_reports.count());

    Ok(())
}

fn get_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines() // Читаем файл построчно
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(line: &[i32], can_fix: bool) -> bool {
    let dir = (line[1] - line[0]).signum();

    for window in line.windows(2) {
        let diff = window[1] - window[0];
        let diff_dist = diff.abs();
        let diff_dir = diff.signum();

        if diff_dir != dir || diff_dist < 1 || diff_dist > 3 {
            if can_fix {
                return line.iter().combinations(line.len() - 1).any(|sub_report| {
                    let combo_values = sub_report.iter().map(|&&x| x).collect_vec();
                    is_safe_report(&combo_values, false)
                });
            }
            return false;
        }
    }
    true
}
