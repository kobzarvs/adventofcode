use geo::{coord, Coord};
use std::collections::HashMap;

enum Stage {
    Map,
    Directions,
}

pub fn parse(input: &str) -> (HashMap<Coord<i32>, String>, Vec<String>, Coord<i32>) {
    let mut map: HashMap<Coord<i32>, String> = HashMap::new();
    let mut directions: Vec<String> = Vec::new();
    let mut robot_pos: Coord<i32> = Coord::zero();
    let mut stage = Stage::Map;
    
    input.lines().enumerate().for_each(|(y, line)| {
        match stage {  
            Stage::Map => {
                if line.is_empty() {
                    stage = Stage::Directions;
                    return; 
                }
                line.chars().enumerate().for_each(|(x, c)| {
                    if c == '@' {
                        robot_pos = coord! {x:x as i32, y:y as i32};
                        map.insert(coord! {x: x as i32, y: y as i32}, ".".to_string());
                    } else {
                        map.insert(coord! {x: x as i32, y: y as i32}, c.to_string());
                    }
                })
            }
            Stage::Directions => {
                if line.is_empty() { return }
                line.chars().for_each(|c| {
                    directions.push(c.to_string());
                })
            }
        } 
    });

    (map, directions, robot_pos)
}

pub fn parse_2(input: &str) -> (HashMap<Coord<i32>, i32>, Vec<String>, Coord<i32>) {
    let mut map: HashMap<Coord<i32>, i32> = HashMap::new();
    let mut directions: Vec<String> = Vec::new();
    let mut robot_pos: Coord<i32> = Coord::zero();
    let mut stage = Stage::Map;
    let mut box_id = 1;
    
    input.lines().enumerate().for_each(|(y, line)| {
        match stage {  
            Stage::Map => {
                if line.is_empty() {
                    stage = Stage::Directions;
                    return; 
                }
                line.chars().enumerate().for_each(|(x, c)| {
                    let x = (x as i32) * 2;  // Умножаем x на 2 для расширенной карты
                    let value = match c {
                        '@' => {
                            robot_pos = coord! {x: x / 2, y: y as i32};  // Сохраняем оригинальную позицию
                            map.insert(coord! {x: x + 1, y: y as i32}, 0);
                            0  // пустое место
                        }
                        '#' => {
                            // Добавляем двойную стену
                            map.insert(coord! {x: x + 1, y: y as i32}, 1);
                            1
                        }
                        'O' => {
                            // Добавляем правую часть ящика
                            box_id += 1;
                            map.insert(coord! {x: x + 1, y: y as i32}, box_id);
                            let result = box_id;
                            result
                        }
                        _ => {
                            // Добавляем пустое место справа
                            map.insert(coord! {x: x + 1, y: y as i32}, 0);
                            0
                        }
                    };
                    map.insert(coord! {x: x, y: y as i32}, value);
                })
            }
            Stage::Directions => {
                if line.is_empty() { return }
                line.chars().for_each(|c| {
                    directions.push(c.to_string());
                })
            }
        } 
    });

    (map, directions, robot_pos)
}
