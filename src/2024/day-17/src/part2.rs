#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::ErrorKind;
use crate::{parse, Computer};

pub fn run(input: &str) -> Result<i64, ErrorKind> {
    solve(parse(input))
}

fn solve(mut computer: Computer) -> Result<i64, ErrorKind> {
    let expected = "2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0".to_string();
    println!("initial state: {:?}", computer);
    
    
    for i in 10_000_000..=1_000_000_000 {
        let mut tmp = computer.clone();
        tmp.a = i;
        let result = tmp.run(i).join(",");
        if result == expected {
            println!("final state: {:?}, register A: {}", tmp, i);
            return Ok(i);
        }
    }

    Err(ErrorKind::NotFound)
}

#[cfg(test)]
mod tests {
    use crate::{parse, part2};
    
    #[test]
    fn solve() {
        let input = include_str!("../test.txt");
        let data = parse(input);

        assert_eq!(117440, part2::solve(data).unwrap());
    }
}
