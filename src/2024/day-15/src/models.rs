#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::io::Read;
use std::ops::Deref;
use geo::{coord, Coord};
use phf::phf_map;


type Coord32 = Coord<i32>;

#[derive(Debug)]
pub struct Robot<'a> {
    pub map: &'a mut HashMap<Coord<i32>, String>,
    pub pos: Coord<i32>,
    pub directions: Vec<String>,
    pub curr_dir: usize,
}

static DIRECTIONS: phf::Map<&'static str, Coord<i32>> = phf_map! {
    "<" => coord! {x:-1, y:0},
    ">" => coord! {x:1, y:0},
    "^" => coord! {x:0, y:-1},
    "v" => coord! {x:0, y:1},
};

impl<'a> Robot<'a> {
    pub fn new(map: &'a mut HashMap<Coord<i32>, String>, pos: Coord<i32>, directions: &[String]) -> Self {
        Self {
            map,
            pos,
            directions: directions.to_vec(),
            curr_dir: 0,
        }
    }
    
    pub fn get_dir(&self) -> Coord<i32> {
        DIRECTIONS.get(self.directions[self.curr_dir].as_str()).cloned().unwrap()
    }
    
    pub fn try_push_boxes(&mut self, pos: &Coord32, dir: &Coord32) -> bool {
        let mut tmp = *pos;
        let mut boxes: Vec<Coord<i32>> = Vec::new();
        
        // Проверяем все позиции в направлении движения
        loop {
            match self.map.get(&tmp) {
                Some(place) => match place.as_str() {
                    "O" => boxes.push(tmp), // Собираем все камни
                    "#" => return false,    // Если встретили стену - движение невозможно
                    "." => {
                        println!("Move boxes {:?}", boxes);
                        io::stdout().flush().unwrap();
                        
                        // Нашли свободное место - двигаем все камни
                        for b in boxes.iter().rev() {
                            self.map.insert(*b, ".".to_string());
                            self.map.insert(*b + *dir, "O".to_string());
                        }
                        return true;
                    }
                    _ => return false
                },
                None => return false // Вышли за границы карты
            }
            tmp = tmp + *dir;
        }
    }

    pub fn do_move(&mut self) -> Coord<i32> {
        let dir = self.get_dir();
        // Проверяем обе позиции при горизонтальном движении
        let check_positions = if dir.x != 0 {
            vec![
                self.pos + coord! { x: dir.x.signum(), y: 0 },
                self.pos + coord! { x: dir.x.signum() * 2, y: 0 }
            ]
        } else {
            vec![self.pos + coord! { x: 0, y: dir.y }]
        };

        // Проверяем все позиции на пути
        for pos in &check_positions {
            if let Some(place) = self.map.get(pos) {
                if place == "#" {
                    println!("Wall");
                    // Увеличиваем curr_dir и возвращаемся
                    if self.curr_dir < self.directions.len() - 1 {
                        self.curr_dir += 1;
                    }
                    self.print();
                    return self.pos;
                }
            } else {
                // Если вышли за пределы карты
                println!("Wall");
                if self.curr_dir < self.directions.len() - 1 {
                    self.curr_dir += 1;
                }
                self.print();
                return self.pos;
            }
        }

        // Если дошли сюда, то путь свободен
        if let Some(place) = self.map.get(&check_positions[0]) {
            match place.as_str() {
                "[" | "]" => {
                    println!("Box");
                    if self.try_push_boxes(&check_positions[0], &dir) {
                        self.pos = self.pos + dir;
                    }
                }
                _ => {
                    println!("Free");
                    self.pos = self.pos + dir;
                }
            }
            io::stdout().flush().unwrap();
        }

        self.print();
        if self.curr_dir < self.directions.len() - 1 {
            self.curr_dir += 1;
        }
        self.pos
    }

    pub fn run(&mut self) {
        if std::env::var("MANUAL").is_ok() {
            self.manual_control();
        } else {
            self.print();
            let total_moves = self.directions.len();
            for _ in 0..total_moves {
                self.do_move();
            }
        }
    }
    
    pub fn calculate(&self) -> u64 {
        self.map.iter().fold(0, |acc, (pos, c)| {
            if c == "O" {
                acc + ((pos.y * 100 + pos.x) as u64)
            } else {
                acc
            }
        })
    }

    pub fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        if self.curr_dir > 0 {
            println!("Current position: {:?}, Curr dir: {:?}", self.pos, (self.curr_dir, &self.directions[self.curr_dir]));
        }
        if let Some((&max_coord, _)) = self.map.iter().max_by_key(|(coord, _)| (coord.y, coord.x)) {
            for y in 0..=max_coord.y {
                for x in 0..=max_coord.x {
                    let coord = coord! {x: x, y: y};
                    if let Some(c) = self.map.get(&coord) {
                        if coord == self.pos {
                            // print!("{}", if self.curr_dir > 0 {self.directions[self.curr_dir].to_string()} else {"@".to_string()});
                            print!("{}", self.directions[self.curr_dir]);
                        } else {
                            print!("{}", c);
                        }
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
        }
        println!();
        io::stdout().flush().unwrap();
        // let mut buffer = [0; 1];
        // io::stdin().read_exact(&mut buffer).unwrap();
        // std::thread::sleep(std::time::Duration::from_millis(10));
    }

    // Добавляем метод для ручного управления
    pub fn manual_control(&mut self) {
        self.print();
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut dirs: Vec<String> = self.directions.to_vec();
            let dir = match input.trim() {
                "a" | "h" => dirs.push("<".to_string()),
                "d" | "l" => dirs.push(">".to_string()),
                "w" | "k" => dirs.push("^".to_string()),
                "s" | "j" => dirs.push("v".to_string()),
                "q" => break,
                _ => continue,
            };
            self.directions = dirs;
            self.curr_dir = 0;
            self.do_move();
        }
    }

}
