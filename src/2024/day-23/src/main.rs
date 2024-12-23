use day_23::{parser, part_1, part_2};
use std::fs;

#[cfg(not(debug_assertions))]
pub const INPUT_FILE: &str = "datasets/release.txt";
#[cfg(debug_assertions)]
pub const INPUT_FILE: &str = "datasets/debug.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        match fs::read_to_string(INPUT_FILE) {
            Ok(data) => {
                let graph = parser::parse(&data)?;

                let part_1 = part_1::solve(&graph);
                println!("Part I:{}\ntime: {:?}", part_1.0, part_1.1);

                let part_2 = part_2::solve(&graph);
                println!("Part II: {}\ntime: {:?}", part_2.0, part_2.1);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        Ok(())
    }
}
