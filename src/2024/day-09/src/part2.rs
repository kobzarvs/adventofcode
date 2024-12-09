use std::cell::RefCell;
use std::iter::zip;
use std::rc::Rc;
use itertools::{Itertools};
use rayon::prelude::*;
use tracing::{info, trace};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum SectorType {
    Free(usize),
    File(usize, usize),
}

fn parse(input: &str) -> Vec<SectorType> {
    let mut index = 0;
    let mut file_id = 0;

    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .chunks(2)
        .into_iter()
        .flat_map(|mut chunk| {
            let file_blocks = chunk.next().unwrap();
            let free_blocks = chunk.next().unwrap_or(0);

            let mut file_vec: Vec<SectorType> = Vec::with_capacity(file_blocks);
            let mut free_vec: Vec<SectorType> = Vec::with_capacity(free_blocks);

            for _ in 0..file_blocks {
                file_vec.push(SectorType::File(file_id, index));
                index += 1;
            }

            for _ in 0..free_blocks {
                free_vec.push(SectorType::Free(index));
                index += 1;
            }

            file_vec.extend(free_vec);
            file_id += 1;

            file_vec
        })
        .collect()
}

#[tracing::instrument(level = "trace")]
pub fn process(input: &str) -> miette::Result<String> {
    let disk_map: Vec<SectorType> = parse(input);

    let disk_map = Rc::new(RefCell::new(disk_map));

    let cloned_disk_map = Rc::clone(&disk_map);
    let disk_map_snapshot: Vec<_> = cloned_disk_map.borrow().iter().cloned().collect();

    disk_map_snapshot
        .into_iter()
        .filter(|it| matches!(it, SectorType::File(..)))
        .rev()
        .chunk_by(|file| {
            match file {
                SectorType::File(id, _) => Some(*id),
                _ => None,
            }
        })
        .into_iter()
        .for_each(|(file_id, file_chunks)| {
            let file_chunks = file_chunks.collect::<Vec<_>>();
            trace!("file: {:?} chunks: {:?}", file_id.unwrap(), file_chunks);

            let mut last_free_index: Option<SectorType> = None;
            let mut free_chunk_id = 0;

            // Обрабатываем свободные сектора
            let free_sectors_snapshot: Vec<_> = cloned_disk_map
                .borrow()
                .par_iter()
                .filter(|it| matches!(*it, SectorType::Free(..)))
                .cloned()
                .collect();

            free_sectors_snapshot
                .iter()
                .chunk_by(|&free| {
                    let last_free = last_free_index.clone();
                    last_free_index = Some(free.clone());
                    match (last_free, free.clone()) {
                        (None, SectorType::Free(_)) => free_chunk_id,
                        (Some(SectorType::Free(last)), SectorType::Free(current)) if current.abs_diff(last) <= 1 => {
                            free_chunk_id
                        }
                        _ => {
                            free_chunk_id += 1;
                            free_chunk_id
                        }
                    }
                })
                .into_iter()
                .any(|(it, free_chunks)| {
                    let free_chunks = free_chunks.collect::<Vec<_>>();
                    info!("\tfree: {:?} chunks: {:?}", it, free_chunks);
                    if file_chunks.len() <= free_chunks.len() {
                        let mut borrowed_disk_map = cloned_disk_map.borrow_mut();

                        zip(file_chunks.clone(), free_chunks.clone()).for_each(|pattern| {
                            if let (SectorType::File(id, file_node), SectorType::Free(free_sector)) = pattern {
                                if file_node < *free_sector {
                                    return;
                                }
                                borrowed_disk_map[*free_sector] = SectorType::File(id, *free_sector);
                                borrowed_disk_map[file_node] = SectorType::Free(file_node);
                            }
                        });

                        true
                    } else {
                        false
                    }
                });
        });

    let result = Rc::clone(&disk_map)
        .borrow()
        .iter()
        .filter(|it| matches!(it, SectorType::File(..)))
        .fold(0, |acc, pattern| match pattern {
            SectorType::File(id, index) => acc + id * index,
            _ => acc,
        });

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
