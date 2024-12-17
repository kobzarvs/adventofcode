#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::ErrorKind;
use std::ops::Deref;
use crate::{parse, Computer};

pub fn run(input: &str) -> Result<String, ErrorKind> {
    solve(parse(input))
}

fn solve(mut computer: Computer) -> Result<String, ErrorKind> {
    println!("initial state: {:?}", computer);

    let result = computer.run(0);

    println!("final state: {:?}", computer);
    println!("result: {:?}", result);
    
    Ok(result.join(","))
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1::solve(data).unwrap());
    }
}
