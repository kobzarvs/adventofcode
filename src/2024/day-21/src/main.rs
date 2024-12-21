#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, stdout, BufRead, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Press,
}

impl Direction {
    fn get_movement(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Press => (0, 0),
        }
    }

    fn to_command(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Press => 'A',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Keyboard {
    Arrow,   // Клавиатура со стрелками
    Numeric, // Цифровая клавиатура
}

#[derive(Clone)]
struct RobotPath {
    commands: Vec<Direction>, // Команды, которые нужно выполнить
    final_pos: Position,      // Конечная позиция после выполнения команд
}

fn get_digit_position(digit: char) -> Position {
    match digit {
        '0' => Position { x: 1, y: 3 },
        '1' => Position { x: 0, y: 2 },
        '2' => Position { x: 1, y: 2 },
        '3' => Position { x: 2, y: 2 },
        '4' => Position { x: 0, y: 1 },
        '5' => Position { x: 1, y: 1 },
        '6' => Position { x: 2, y: 1 },
        '7' => Position { x: 0, y: 0 },
        '8' => Position { x: 1, y: 0 },
        '9' => Position { x: 2, y: 0 },
        'A' => Position { x: 2, y: 3 },
        _ => panic!("Неверный символ"),
    }
}

fn get_digit_by_position(pos: Position) -> char {
    match pos {
         Position { x: 1, y: 3 } => '0',
         Position { x: 0, y: 2 } => '1',
         Position { x: 1, y: 2 } => '2',
         Position { x: 2, y: 2 } => '3',
         Position { x: 0, y: 1 } => '4',
         Position { x: 1, y: 1 } => '5',
         Position { x: 2, y: 1 } => '6',
         Position { x: 0, y: 0 } => '7',
         Position { x: 1, y: 0 } => '8',
         Position { x: 2, y: 0 } => '9',
         Position { x: 2, y: 3 } => 'A',
        _ => panic!("Неверный символ"),
    }
}

// fn get_arrow_position(command: char) -> Position {
//     match command {
//         '^' => Position { x: 1, y: 0 }, // Кнопка вверх
//         'v' => Position { x: 1, y: 1 }, // Кнопка вниз
//         '<' => Position { x: 0, y: 1 }, // Кнопка влево
//         '>' => Position { x: 2, y: 1 }, // Кнопка вправо
//         'A' => Position { x: 2, y: 0 }, // Кнопка A
//         _ => panic!("Неверная команда для стрелочной клавиатуры"),
//     }
// }

fn is_valid_position(pos: &Position, keyboard: Keyboard) -> bool {
    match keyboard {
        Keyboard::Arrow => {
            // Все возможные позиции на стрелочной клавиатуре
            match (pos.x, pos.y) {
                (1, 0) | // ^
                (0, 1) | // <
                (1, 1) | // v
                (2, 1) | // >
                (2, 0)   // A
                    => true,
                _ => false
            }
        }
        Keyboard::Numeric => {
            match (pos.x, pos.y) {
                (x, y) if x >= 0 && x <= 2 && y >= 0 && y <= 2 => true, // цифры 1-9
                (1, 3) | (2, 3) => true,                                // 0 и A
                _ => false,
            }
        }
    }
}

fn get_possible_moves(pos: &Position, keyboard: Keyboard) -> Vec<Direction> {
    let mut moves = vec![];
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::Press,
    ] {
        let (dx, dy) = dir.get_movement();
        let new_pos = Position {
            x: pos.x + dx,
            y: pos.y + dy,
        };

        // Проверяем, что новая позиция валидна для данной клавиатуры
        if is_valid_position(&new_pos, keyboard) {
            moves.push(dir);
        }
    }
    moves
}

fn apply_move(pos: &Position, dir: &Direction) -> Position {
    let (dx, dy) = dir.get_movement();
    Position {
        x: pos.x + dx,
        y: pos.y + dy,
    }
}

fn find_path_to_target(start: Position, target: Position, keyboard: Keyboard) -> Option<RobotPath> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Начальное состояние: пустой путь, начальная позиция
    queue.push_back((start, Vec::new()));
    visited.insert(start);

    while let Some((pos, path)) = queue.pop_front() {
        if pos == target {
            return Some(RobotPath {
                commands: path,
                final_pos: pos,
            });
        }

        for dir in get_possible_moves(&pos, keyboard) {
            let new_pos = apply_move(&pos, &dir);
            if !visited.contains(&new_pos) && is_valid_position(&new_pos, keyboard) {
                let mut new_path = path.clone();
                new_path.push(dir);
                visited.insert(new_pos);
                queue.push_back((new_pos, new_path));
            }
        }
    }
    None
}

fn get_min_commands(target: &str) -> String {
    let mut result = String::new();
    // Все роботы начинают с позиции A (2, 0)
    let mut robot3_pos = Position { x: 2, y: 3 }; // A
    let mut robot2_pos = Position { x: 2, y: 0 }; // A
    let mut robot1_pos = Position { x: 2, y: 0 }; // A

    // Для каждой цифры в целевом числе
    for digit in target.chars() {
        // 1. Находим путь для третьего робота к текущей цифре
        let target_pos = get_digit_position(digit);
        let robot3_path = find_path_to_target(robot3_pos, target_pos, Keyboard::Numeric)
            .expect("Не удалось найти путь для третьего робота");

        // 2. Для каждой команды третьего робота находим, как второй робот должен её передать
        for &command in &robot3_path.commands {
            // Определяем, какую позицию должен занять второй робот, чтобы передать команду
            let required_pos2 = match command {
                Direction::Up => Position { x: 1, y: 0 },    // ^
                Direction::Down => Position { x: 1, y: 1 },  // v
                Direction::Left => Position { x: 0, y: 1 },  // <
                Direction::Right => Position { x: 2, y: 1 }, // >
                Direction::Press => Position { x: 2, y: 0 }, // A
            };

            // 3. Находим путь для второго робота к нужной позиции
            let robot2_path = find_path_to_target(robot2_pos, required_pos2, Keyboard::Arrow)
                .expect("Не удалось найти путь для второго робота");

            // 4. Для каждой команды второго робота находим команды первого робота
            for &command2 in &robot2_path.commands {
                // Первый робот должен выполнить те же движения
                let robot1_path = find_path_to_target(robot1_pos, required_pos2, Keyboard::Arrow)
                    .expect("Не удалось найти путь для первого робота");

                // Добавляем команды первого робота в результат
                for cmd in robot1_path.commands {
                    result.push(cmd.to_command());
                }
                result.push('A'); // Нажатие для передачи команды

                // Обновляем позиции роботов
                robot1_pos = required_pos2;
                robot2_pos = required_pos2;
            }

            // Обновляем позицию третьего робота после выполнения команды
            robot3_pos = apply_move(&robot3_pos, &command);
        }
    }

    result
}

fn draw_keyboard_frame(width: u16) -> String {
    format!("{:>width$}", "", width = width as usize)
}

fn draw_arrow_keyboard(pos: &Position, robot_name: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let frame = draw_keyboard_frame(5);

    lines.push(format!("    {}{}:", frame, robot_name));
    lines.push(format!("{}    +---+---+", frame));

    let mut line = format!("{}    |", frame);
    line.push_str(if pos.x == 1 && pos.y == 0 {
        "[^]"
    } else {
        " ^ "
    });
    line.push_str(if pos.x == 2 && pos.y == 0 {
        "|[A]|"
    } else {
        "| A |"
    });
    lines.push(line);

    lines.push(format!("{}+---+---+---+", frame));

    let mut line = format!("{}|", frame);
    line.push_str(if pos.x == 0 && pos.y == 1 {
        "[<]"
    } else {
        " < "
    });
    line.push_str(if pos.x == 1 && pos.y == 1 {
        "|[v]"
    } else {
        "| v "
    });
    line.push_str(if pos.x == 2 && pos.y == 1 {
        "|[>]|"
    } else {
        "| > |"
    });
    lines.push(line);

    lines.push(format!("{}+---+---+---+", frame));
    lines
}

fn draw_numeric_keyboard(pos: &Position, last_command: Option<Direction>) -> Vec<String> {
    let mut lines = Vec::new();
    let frame = draw_keyboard_frame(50);

    // Добавляем строку с последней полученной командой
    let command_str = match last_command {
        Some(Direction::Up) => "Последняя команда: Вверх (^)",
        Some(Direction::Down) => "Последняя команда: Вниз (v)",
        Some(Direction::Left) => "Последняя команда: Влево (<)",
        Some(Direction::Right) => "Последняя команда: Вправо (>)",
        Some(Direction::Press) => "Последняя команда: Нажатие (A)",
        None => "Ожидание команды",
    };
    lines.push(format!(
        "{}Цифровая клавиатура (Робот 3) - {}",
        frame, command_str
    ));

    lines.push(format!("{}+---+---+---+", frame));

    for y in 0..3 {
        let mut line = format!("{}|", frame);
        for x in 0..3 {
            let current = Position { x, y };
            let num = get_digit_by_position(Position {x, y});
            line.push_str(
                if current == *pos {
                    format!("[{}]", num)
                } else {
                    format!(" {} ", num)
                }
                .as_str(),
            );
            line.push('|');
        }
        lines.push(line);
        lines.push(format!("{}+---+---+---+", frame));
    }

    // lines.push(format!("{}    +---+---+", frame));
    let mut line = format!("{}    |", frame);
    line.push_str(if pos.y == 3 && pos.x == 1 {
        "[0]"
    } else {
        " 0 "
    });
    line.push_str(if pos.y == 3 && pos.x == 2 {
        "|[A]|"
    } else {
        "| A |"
    });
    lines.push(line);
    lines.push(format!("{}    +---+---+", frame));

    lines
}

fn manual_control() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = stdout().into_raw_mode()?;

    let mut human_pos = Position { x: 2, y: 0 }; // Начинаем с A
    let mut robot1_pos = Position { x: 2, y: 0 }; // Начинаем с A
    let mut robot2_pos = Position { x: 2, y: 0 }; // Начинаем с A
    let mut robot3_pos = Position { x: 2, y: 3 }; // Начинаем с A
    let mut commands = String::new();
    let mut result = String::new();

    // Текущая команда для передачи роботам
    // Флаги для отслеживания нажатий A
    let mut last_robot3_command = None; // Добавляем переменную для хранения последней команды

    write!(stdout, "{}", termion::clear::All)?;

    for c in stdin.keys() {
        // write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;

        match c? {
            Key::Up => {
                let new_pos = Position {
                    x: human_pos.x,
                    y: human_pos.y - 1,
                };
                if is_valid_position(&new_pos, Keyboard::Arrow) {
                    human_pos = new_pos;
                }
            }
            Key::Down => {
                let new_pos = Position {
                    x: human_pos.x,
                    y: human_pos.y + 1,
                };
                if is_valid_position(&new_pos, Keyboard::Arrow) {
                    human_pos = new_pos;
                }
            }
            Key::Left => {
                let new_pos = Position {
                    x: human_pos.x - 1,
                    y: human_pos.y,
                };
                if is_valid_position(&new_pos, Keyboard::Arrow) {
                    human_pos = new_pos;
                }
            }
            Key::Right => {
                let new_pos = Position {
                    x: human_pos.x + 1,
                    y: human_pos.y,
                };
                if is_valid_position(&new_pos, Keyboard::Arrow) {
                    human_pos = new_pos;
                }
            }
            Key::Char('\n') => {
                let human_cmd: char = match (human_pos.x, human_pos.y) {
                    (1, 0) => {
                        commands.push('^');
                        '^'
                    },
                    (2, 0) => {
                        commands.push('A');
                        'A'
                    },
                    (0, 1) => {
                        commands.push('<');
                        '<'
                    },
                    (1, 1) => {
                        commands.push('v');
                        'v'
                    },
                    (2, 1) => {
                        commands.push('>');
                        '>'
                    },
                    _ => ' '
                };

                if human_cmd == 'A' {
                    // Человек на кнопке A
                    // Первый робот передает команду второму роботу
                    let command_for_robot2 = match (robot1_pos.x, robot1_pos.y) {
                        (1, 0) => Direction::Up,    // ^
                        (2, 0) => Direction::Press, // A
                        (0, 1) => Direction::Left,  // <
                        (1, 1) => Direction::Down,  // v
                        (2, 1) => Direction::Right, // >
                        _ => unreachable!(),
                    };

                    match command_for_robot2 {
                        Direction::Press => {
                            let command_for_robot3 = match (robot2_pos.x, robot2_pos.y) {
                                (1, 0) => Direction::Up,    // ^
                                (1, 1) => Direction::Down,  // v
                                (0, 1) => Direction::Left,  // <
                                (2, 1) => Direction::Right, // >
                                (2, 0) => {
                                    result.push(get_digit_by_position(robot3_pos));
                                    continue;
                                },
                                _ => continue,
                            };

                            let new_pos3 = apply_move(&robot3_pos, &command_for_robot3);
                            if is_valid_position(&new_pos3, Keyboard::Numeric) {
                                robot3_pos = new_pos3;
                                last_robot3_command = Some(command_for_robot3);
                            }
                        },
                        _ => {
                            // Первый робот двигается в указанном направлении
                            let new_pos = apply_move(&robot2_pos, &command_for_robot2);
                            if is_valid_position(&new_pos, Keyboard::Arrow) {
                                robot2_pos = new_pos;
                            }
                        }
                    }
                } else {
                    // Определяем направление движения по текущей позиции человека
                    let command = match human_cmd {
                        '^' => Some(Direction::Up),    // ^
                        'v' => Some(Direction::Down),  // v
                        '<' => Some(Direction::Left),  // <
                        '>' => Some(Direction::Right), // >
                        _ => None,
                    };

                    if let Some(cmd) = command {
                        // Первый робот двигается в указанном направлении
                        let new_pos = apply_move(&robot1_pos, &cmd);
                        if is_valid_position(&new_pos, Keyboard::Arrow) {
                            robot1_pos = new_pos;
                        }
                    }
                }
            }
            Key::Ctrl('c') | Key::Char('q') => break,
            _ => {}
        }

        // Отрисовка цифровой клавиатуры
        let mut y = 5;
        for line in draw_numeric_keyboard(&robot3_pos, last_robot3_command) {
            write!(stdout, "{}{}", termion::cursor::Goto(1, y), line)?;
            y += 1;
        }
        y += 2;

        // Отрисовка стрелочных клавиатур в ряд
        let arrow_keyboards = [
            (&robot2_pos, "Робот 2"),
            (&robot1_pos, "Робот 1"),
            (&human_pos, "Человек"),
        ];

        let keyboard_lines: Vec<Vec<String>> = arrow_keyboards
            .iter()
            .map(|(pos, name)| draw_arrow_keyboard(pos, name))
            .collect();

        for row in 0..7 {
            let mut line = String::new();
            for keyboard in &keyboard_lines {
                if row < keyboard.len() {
                    line.push_str(&keyboard[row]);
                    line.push_str("   ");
                }
            }
            write!(stdout, "{}{}", termion::cursor::Goto(1, y), line)?;
            y += 1;
        }

        y += 1;
        write!(
            stdout,
            "{}Команды: {}",
            termion::cursor::Goto(1, y),
            commands
        )?;
        y += 1;
        write!(
            stdout,
            "{}Результат: {}",
            termion::cursor::Goto(1, y),
            result
        )?;

        stdout.flush()?;
    }

    write!(stdout, "{}", termion::cursor::Show)?;
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Выберите режим:");
    println!("1. Автоматическое решение");
    println!("2. Ручное управление");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => {
            let file = File::open("test.txt")?;
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                let number = line?.trim().to_string();
                if !number.is_empty() {
                    let result = get_min_commands(&number);
                    println!("Для числа {}: {}", number, result);
                }
            }
        }
        "2" => {
            manual_control()?;
        }
        _ => println!("Неверный выбор"),
    }

    Ok(())
}
