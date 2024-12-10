use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    id: usize,
    start: usize,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Hole {
    start: usize,
    size: usize,
}

impl Ord for Hole {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start.cmp(&other.start) {
            std::cmp::Ordering::Equal => self.size.cmp(&other.size),
            other => other,
        }
    }
}



fn parse(input: &str) -> (Vec<File>, BTreeSet<Hole>) {
    let mut pos = 0;
    let mut id = 0;

    let mut files: Vec<File> = vec![];
    let mut holes: BTreeSet<Hole> = BTreeSet::new();

    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .chunks(2)
        .into_iter()
        .for_each(|mut chunk| {
            let file_size = chunk.next().unwrap();
            let hole_size = chunk.next().unwrap_or(0);

            files.push(File{id, start: pos, size: file_size});
            pos += file_size;

            if hole_size > 0 {
                holes.insert(Hole { start: pos, size: hole_size });
                pos += hole_size;
            }

            id += 1;
        });

    (files, holes)
}

pub fn solve(input: &str) -> usize {
    let (mut files, holes) = parse(input);
    let mut result = 0;

    let holes = Rc::new(RefCell::new(holes));
    let mut iter_holes = holes.borrow_mut();

    for file in files.iter_mut().rev() {
        let pass_holes = iter_holes.range(..=Hole{start: file.start, size: 0});
        
        for hole in pass_holes.cloned().collect::<Vec<_>>() {
            if hole.size >= file.size {
                if hole.size > file.size {
                    let new_hole = Hole {
                        start: hole.start + file.size,
                        size: hole.size - file.size,
                    };
                    iter_holes.insert(new_hole);
                }
                
                file.start = hole.start;
                iter_holes.remove(&hole);
                break;
            }
        }

        for i in 0..file.size {
            result += file.id * (file.start + i);
        }
    }

    if result != 6381624803796 && result != 2858 {
        println!("Wrong result: {:?}", result);
    }
    result
}
