use geo::{coord, Coord};

pub type Byte = (i32, Coord<i32>);

pub fn parse(input: &str) -> Vec<Byte> {
    let mut i = 0;
    input
        .lines()
        .filter_map(|line| {
            let coords: Vec<&str> = line.split(",").collect();
            if coords.len() >= 2 {
                let wall = coord! {
                    x: coords[0].parse::<i32>().ok()?,
                    y: coords[1].parse::<i32>().ok()?
                };
                let item = Some((i, wall));
                i += 1;
                return item;
            } else {
                None
            }
        })
        .collect()
}

pub fn parse_2(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            (coords[0], coords[1])
        })
        .collect()
}
