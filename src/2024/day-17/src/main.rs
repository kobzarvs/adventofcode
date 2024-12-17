#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io::ErrorKind;
use day_17::{part1, part2, read_file};

fn main() -> Result<(), ErrorKind>{
    let input = read_file();

    // let result = part1::run(&input)?;
    // println!("part I: {}", result);
    
    // let input = include_str!("../test2.txt");
    let result = part2::run(&input)?;
    println!("part II: {}", result);
    
    Ok(())
}
