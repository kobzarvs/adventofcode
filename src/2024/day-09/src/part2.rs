use itertools::{Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    id: usize,
    start: usize,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hole {
    start: usize,
    size: usize,
}

fn parse(input: &str) -> (Vec<File>, Vec<Hole>) {
    let mut pos = 0;
    let mut id = 0;

    let mut files: Vec<File> = vec![];
    let mut holes: Vec<Hole> = vec![];

    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .chunks(2)
        .into_iter()
        .for_each(|mut chunk| {
            let file_size = chunk.next().unwrap();
            let hole_size = chunk.next().unwrap_or(0);

            files.insert(0, File{id, start: pos, size: file_size});
            pos += file_size;
            holes.push(Hole{start: pos, size: hole_size});
            pos += hole_size;

            id += 1;
        });

    (files, holes)
}

#[tracing::instrument(level = "trace")]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut files, mut holes) = parse(input);
    let mut result = 0;

    for file in files.iter_mut() {
        for hole in holes.iter_mut() {
            // если дырка меньше файла - пропускаем
            if hole.size < file.size || file.start <= hole.start { continue; }
            // перемешаем файл на место дырки
            file.start = hole.start;
            // смещаем дырку вправо на размер файла
            hole.start = hole.start + file.size;
            // корректируем размер дырки
            hole.size -= file.size;
        }
        for i in 0..file.size {
            result += file.id * (file.start + i);
        }
    }

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
