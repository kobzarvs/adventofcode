#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use day_15::{part2, read_file};
use std::io;

fn main() -> io::Result<()> {
    let input = read_file();

    // let result = part1::run(&input);
    // println!("part I: {}", result);

    let result = part2::run(&input).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    println!("part II: {}", result);

    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
