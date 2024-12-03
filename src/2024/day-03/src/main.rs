use std::{env, fs};
use std::path::PathBuf;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| PathBuf::from("test.txt"));
    println!("Input file: {:?}", filename);
    let input = fs::read_to_string(filename).expect("Unable to read file");

    //-------------------------------------------------------------------------------//

    let re = Regex::new(r"(mul\(\d+,\d+\))|(don't\(\))|(do\(\))").unwrap();
    let (mut part_1, mut part_2) = (0, 0);
    let mut skip = false;

    for matched_expr in re.find_iter(&input) {
        match matched_expr.as_str() {
            "do()" => skip = false,
            "don't()" => skip = true,
            expr => {
                let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                let result = re_mul.captures(expr).and_then(|captures| {
                    let a = captures.get(1)?.as_str().parse::<u32>().unwrap();
                    let b = captures.get(2)?.as_str().parse::<u32>().unwrap();
                    Some(a * b)
                }).unwrap_or(0);

                part_1 += result;

                if !skip {
                    part_2 += result;
                }
            }
        }
    }

    println!("Part I: {}", part_1);
    println!("Part II: {}", part_2);
}
