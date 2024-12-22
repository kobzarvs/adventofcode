extern crate core;

use std::fs;
use day_22::*;

#[cfg(not(debug_assertions))]
pub const INPUT_FILE: &str = "datasets/release.txt";
#[cfg(debug_assertions)]
pub const INPUT_FILE: &str = "datasets/debug.txt";


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    match fs::read_to_string(INPUT_FILE) {
        Ok(data) => {
            let data = parser::parse(&data)?;

            let part_1 = part_1::solve(&data);
            println!("Part I:  {:>12?}, time: {:?}", part_1.0, part_1.1);

            let part_2 = part_2::solve(&data);
            println!("Part II: {:>12?}, time: {:?}", part_2.0, part_2.1);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
