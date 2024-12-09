use itertools::Itertools;


pub fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut index = 0;
    let mut file_id = 0;

    let mut file_vec: Vec<(usize, usize)> = vec![];
    let mut free_vec: Vec<usize> = vec![];

    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .chunks(2)
        .into_iter()
        .for_each(|mut chunk| {
            let file_blocks = chunk.next().unwrap();
            let free_blocks = chunk.next().unwrap_or(0);

            for _ in 0..file_blocks {
                file_vec.push((file_id, index));
                index += 1;
            }

            for _ in 0..free_blocks {
                free_vec.push(index);
                index += 1;
            }

            file_id += 1;
        });

    file_vec.reverse();

    (file_vec, free_vec)
}

pub fn solve(input: &str) -> usize {
    let (files, unused) = parse(input);
    let mut result = 0;

    for (i, &file) in files.iter().enumerate() {
        let free_block = unused.get(i);
        match (file, free_block) {
            ((file_id, file_sector), Some(&free_sector)) if file_sector < free_sector => {
                result += file_id * file_sector;
            }
            ((file_id, _), Some(&free_sector)) => {
                result += file_id * free_sector;
            }
            ((file_id, file_node), None) => {
                result += file_id * file_node;
            }
        }
    }

    result
}
