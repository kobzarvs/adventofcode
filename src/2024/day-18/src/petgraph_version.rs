use petgraph::graphmap::UnGraphMap;
use petgraph::algo::astar;
use crate::{parse_2, read_file};

// размер сетки
const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const LIMIT: usize = 1024;
const START: (i32, i32) = (0, 0);
const END: (i32, i32) = (70, 70);

pub fn path_finding() -> Option<(i32, i32)>{
    let input = read_file();
    
    let bytes = parse_2(&input);
    
    // создаем граф
    let mut graph = UnGraphMap::new();
    for x in 0..WIDTH as i32 {
        for y in 0..HEIGHT as i32 {
            for (nx, ny) in get_neighbors(x, y) {
                // в пределах сетки добавляем ребра
                if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                    // вес ребра равен 1
                    graph.add_edge((x, y), (nx, ny), 1);
                }
            }
        }
    }

    let mut path = astar_path(&graph, START, END).expect("Начальный путь отсутствует");

    for (i, &pos) in bytes.iter().enumerate() {
        if i == LIMIT {
            let mut path = astar_path(&graph, START, END).expect("Начальный путь отсутствует");
            println!("Part I: {}", path.len() - 1);
        }

        for (nx, ny) in get_neighbors(pos.0, pos.1) {
            if let Some(_) = graph.remove_edge(pos, (nx, ny)) {}
        }

        // если обновленный путь невозможен
        if i >= LIMIT && path.contains(&pos) {
            match astar_path(&graph, START, END) {
                Some(new_path) => path = new_path,
                None => {
                    return Some(pos);
                }
            }
        }
    }
    
    None
}

fn get_neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
    ]
}

// A* враппер
fn astar_path(
    graph: &UnGraphMap<(i32, i32), i32>,
    start: (i32, i32),
    goal: (i32, i32),
) -> Option<Vec<(i32, i32)>> {
    astar(
        graph,
        start,
        |finish| finish == goal,
        |_| 1, // Все ребра одинакового веса
        |node| (goal.0 - node.0).abs() as u32 + (goal.1 - node.1).abs() as u32, // манхэттен для приоритизации
    ).map(|(_, path)| path)
}
