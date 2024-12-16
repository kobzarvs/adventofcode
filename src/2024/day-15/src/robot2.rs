#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use geo::{coord, Coord};
use itertools::Itertools;
use phf::phf_map;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::io::{stdout, Read};
use std::ops::Deref;

type Coord32 = Coord<i32>;

static DIRECTIONS: phf::Map<&'static str, Coord<i32>> = phf_map! {
    "<" => coord! {x:-1, y:0},
    ">" => coord! {x:1, y:0},
    "^" => coord! {x:0, y:-1},
    "v" => coord! {x:0, y:1},
};

#[derive(Debug)]
pub struct Robot2 {
    pub map: HashMap<Coord<i32>, i32>,
    pub pos: Coord<i32>,
    pub directions: Vec<String>,
    pub curr_dir: usize,
    pub boxes: HashMap<i32, (Coord<i32>, Coord<i32>)>,
}

impl Robot2 {
    pub fn new(map: &HashMap<Coord<i32>, i32>, pos: Coord<i32>, directions: &[String]) -> Self {
        let mut boxes = HashMap::new();

        // Собираем информацию о ящиках
        for (&coord, &value) in map.iter() {
            if value >= 2 && coord.x % 2 == 0 {
                // Это ящик
                boxes.insert(value, (coord, coord! {x: coord.x + 1, y: coord.y}));
            }
        }

        Self {
            map: map.clone(),
            pos: coord! { x: pos.x * 2, y: pos.y },
            directions: directions.to_vec(),
            curr_dir: 0,
            boxes,
        }
    }

    pub fn get_dir(&self) -> Coord<i32> {
        let base_dir = DIRECTIONS
            .get(self.directions[self.curr_dir].as_str())
            .unwrap();
        // Для горизонтального движения делаем шаг в 2 клетки
        coord! {
            x: base_dir.x,
            y: base_dir.y
        }
    }

    pub fn try_push_boxes(
        &mut self,
        next_pos: &Coord32,
        dir: &Coord32,
        box_id: i32,
    ) -> (bool, Vec<i32>) {
        let tbox = self.boxes.get(&box_id).unwrap();

        // Двигаем горизонтально
        if dir.y == 0 {
            let pos_after_box = *next_pos + *dir * 2;

            match self.map.get(&pos_after_box).unwrap() {
                0 => {
                    // free двигаем влево/вправо
                    // self.boxes.entry(box_id).and_modify(|(left, right)| {
                    //     *left = *left + *dir;
                    //     *right = *right + *dir;
                    // });
                    // self.map.insert(pos_after_box, box_id);
                    // self.map.insert(*next_pos, 0);
                    (true, vec![box_id])
                }
                1 => {
                    // wall
                    (false, vec![])
                }
                next_box_id => {
                    let mut boxes_for_push = vec![];

                    let (can_push, deps_boxes) =
                        self.try_push_boxes(&pos_after_box, dir, *next_box_id);
                    if !can_push {
                        return (false, vec![]);
                    }
                    boxes_for_push.extend(deps_boxes);
                    boxes_for_push.push(box_id);

                    (true, boxes_for_push)
                    // free двигаем влево/вправо
                    // self.boxes.entry(box_id).and_modify(|(left, right)| {
                    //     *left = *left + *dir;
                    //     *right = *right + *dir;
                    // });
                    // self.map.insert(pos_after_box, box_id);
                    // self.map.insert(*next_pos, 0);
                }
            }
        } else {
            // Двигаем вертикально
            let (left_branch_pos, right_branch_pos) = (tbox.0 + *dir, tbox.1 + *dir);
            let left_side_box_id = self.map.get(&left_branch_pos).unwrap().clone();
            let right_side_box_id = self.map.get(&right_branch_pos).unwrap().clone();

            match (left_side_box_id, right_side_box_id) {
                (0, 0) => (true, vec![box_id]),
                (_, 1) | (1, _) => (false, vec![]),
                (left_box_id, right_box_id) => {
                    let mut boxes_for_push = vec![];
                    let mut boxes_for_push_deps = vec![];
                    let mut can_push = false;

                    for next_box_id in [left_box_id, right_box_id] {
                        if next_box_id == 0 {
                            continue;
                        }
                        (can_push, boxes_for_push_deps) =
                            self.try_push_boxes(&coord! {x: 100, y: 100}, dir, next_box_id);
                        if !can_push {
                            return (false, vec![]);
                        }
                        boxes_for_push.extend(boxes_for_push_deps);
                    }
                    boxes_for_push.push(box_id);
                    boxes_for_push = boxes_for_push.into_iter().unique().collect();
                    (true, boxes_for_push)
                }
            }
        }
    }

    pub fn do_move(&mut self) -> Coord<i32> {
        let dir = self.get_dir();
        let next_pos = self.pos + dir;

        // Проверяем все позиции на пути
        if let Some(place) = self.map.get(&next_pos) {
            if *place == 1 {
                println!("Wall\r\n");
            } else if *place == 0 {
                println!("Free\r\n");
                self.pos = next_pos;
                stdout().flush().unwrap();
            } else {
                println!("Box\r\n");
                // Пробуем толкать только если это первая позиция
                let (can_push, boxes) = self.try_push_boxes(&next_pos, &dir, *place);
                // println!("can push: {}, Boxes: {:?}", can_push, boxes);
                if can_push {
                    for box_id in boxes {
                        // println!("move Box ID: {} {:?}", box_id, self.boxes[&box_id]);
                        self.map.insert(self.boxes[&box_id].0, 0);
                        self.map.insert(self.boxes[&box_id].1, 0);
                        self.map.insert(self.boxes[&box_id].0 + dir, box_id);
                        self.map.insert(self.boxes[&box_id].1 + dir, box_id);
                        self.boxes.entry(box_id).and_modify(|(left, right)| {
                            *left = *left + dir;
                            *right = *right + dir;
                        });
                    }

                    self.pos = next_pos;
                }
            }
        }

        self.print().unwrap();
        if self.curr_dir < self.directions.len() - 1 {
            self.curr_dir += 1;
        }
        self.pos
    }

    // ручное управление персонажем
    pub fn manual_control(&mut self) -> io::Result<()> {
        self.print()?;
        loop {
            let dir = match read()? {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => { break; }
                    KeyCode::Up => "^",
                    KeyCode::Down => "v",
                    KeyCode::Left => "<",
                    KeyCode::Right => ">",
                    _ => { continue; }
                },
                _ => { continue; }
            };
            self.directions = vec![dir.to_string()];
            self.curr_dir = 0;
            self.do_move();
        }
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        if std::env::var("MANUAL").is_ok() {
            self.manual_control()?;
        } else {
            self.print()?;
            let total_moves = self.directions.len();
            for _ in 0..total_moves {
                self.do_move();
            }
        }

        Ok(())
    }

    pub fn calculate(&self) -> u64 {
        self.boxes
            .iter()
            .fold(0, |acc, (_, pos)| acc + (pos.0.y * 100 + pos.0.x) as u64)
    }

    pub fn print(&self) -> io::Result<()> {
        disable_raw_mode()?;
        let mut stdout = stdout();
        print!("\x1B[2J\x1B[1;1H");
        println!(
            "\r\n\r\nr\nCurrent position: {:?}, Curr dir: {:?} = {:?} {:?}\r\n",
            self.pos,
            (self.curr_dir, &self.directions[self.curr_dir]),
            self.map.get(&self.pos),
            cursor::MoveToNextLine(1)
        );
        if let Some((&max_coord, _)) = self.map.iter().max_by_key(|(coord, _)| (coord.y, coord.x)) {
            for y in 0..=max_coord.y {
                for x in 0..=max_coord.x {
                    let coord = coord! {x: x, y: y};
                    if let Some(c) = self.map.get(&coord) {
                        if coord == self.pos {
                            print!("{}", self.directions[self.curr_dir]);
                        } else {
                            match *c {
                                0 => print!(" "),
                                1 => print!("#"),
                                _ => {
                                    let tbox = self.boxes.get(c).unwrap();
                                    if tbox.0 == coord {
                                        print!("[]");
                                    }
                                }
                            }
                        }
                    }
                }
                println!();
            }
        }
        println!();
        io::stdout().flush()?;

        enable_raw_mode()?;

        // пошаговый вариант
        // let mut buffer = [0; 1];
        // io::stdin().read_exact(&mut buffer).unwrap();

        // замедление визуализации
        std::thread::sleep(std::time::Duration::from_millis(60));
        Ok(())
    }
}
