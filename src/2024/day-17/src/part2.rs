#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::ErrorKind;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use rayon::prelude::*;
use crate::{parse, Computer};

pub fn run(input: &str) -> Result<i64, ErrorKind> {
    solve(parse(input))
}

fn solve(computer: Computer) -> Result<i64, ErrorKind> {
    let expected = "2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0".to_string();
    println!("initial state: {:?}", computer);
    
    let timeout = Duration::from_secs(10);
    let chunk_size = 1000;

    let total = Arc::new(AtomicU64::new(0));
    let completed = Arc::new(AtomicU64::new(0));
    let timeouts = Arc::new(AtomicU64::new(0));

    let total_clone = total.clone();
    let completed_clone = completed.clone();
    let timeouts_clone = timeouts.clone();

    // Запускаем поток для вывода статистики
    std::thread::spawn(move || {
        loop {
            println!("\rTotal: {}, Completed: {}, Timeouts: {}", 
                total_clone.load(Ordering::Relaxed),
                completed_clone.load(Ordering::Relaxed),
                timeouts_clone.load(Ordering::Relaxed)
            );
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    let result = (1_000_000_000..=10_000_000_000)
        .collect::<Vec<i64>>()
        .into_par_iter()
        .chunks(chunk_size)
        .find_map_first(|chunk| {
            let start = Instant::now();
            
            for i in chunk {
                total.fetch_add(1, Ordering::Relaxed);
                
                if start.elapsed() > timeout {
                    timeouts.fetch_add(1, Ordering::Relaxed);
                    return None;
                }
                
                let mut tmp = computer.clone();
                tmp.a = i;
                let result = tmp.run(i).join(",");
                completed.fetch_add(1, Ordering::Relaxed);
                
                if result == expected {
                    let mut final_state = computer.clone();
                    final_state.a = i;
                    final_state.run(i);
                    println!("\nfinal state: {:?}, register A: {}", final_state, i);
                    return Some(i);
                }
            }
            None
        })
        .ok_or(ErrorKind::NotFound);

    result
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
