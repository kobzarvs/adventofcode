use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::connected_components;
use petgraph::dot::{Dot, Config};
use petgraph::prelude::*;
use ndarray::prelude::*;
use sprs::{CsMat, CsVec};

fn main() {
    // Создание графа
    let mut graph = DiGraph::<&str, &str>::new();
    let edges = [
        ("jqt", "rhn"), ("jqt", "xhk"), ("jqt", "nvd"),
        ("rhn", "xhk"), ("rhn", "bvb"), ("rhn", "hfx"),
        ("bvb", "xhk"), ("bvb", "hfx"),
        ("ntq", "jqt"), ("ntq", "hfx"), ("ntq", "bvb"), ("ntq", "xhk"),
        ("xhk", "hfx"),
        ("rzs", "qnr"), ("rzs", "cmg"), ("rzs", "lsr"), ("rzs", "rsh"),
        ("rsh", "frs"), ("rsh", "pzl"), ("rsh", "lsr"),
        ("frs", "qnr"), ("frs", "lhk"), ("frs", "lsr"),
        ("cmg", "qnr"), ("cmg", "nvd"), ("cmg", "lhk"), ("cmg", "bvb"),
        ("pzl", "lsr"), ("pzl", "hfx"), ("pzl", "nvd"),
        ("qnr", "nvd"),
        ("lsr", "lhk"),
        ("nvd", "lhk"),
    ];

    for (source, target) in edges.iter() {
        let source_node = graph.add_node(source);
        let target_node = graph.add_node(target);
        graph.add_edge(source_node, target_node, "");
    }

    // Вывод графа в формате DOT
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // Получение лапласиана графа
    let laplacian_matrix = laplacian_matrix(&graph);

    // Вычисление Fiedler вектора
    let fiedler_vector = fiedler_vector(&laplacian_matrix);

    // Находим три ребра, которые разделят граф
    let partitions = find_partitions(&graph, &fiedler_vector);

    // Создание двух подграфов
    let subgraph_1 = create_subgraph(&graph, &partitions, 1);
    let subgraph_2 = create_subgraph(&graph, &partitions, -1);

    // Подсчёт количества вершин и рёбер в каждой группе
    let nodes_count_group_1 = subgraph_1.node_count();
    let edges_count_group_1 = subgraph_1.edge_count();

    let nodes_count_group_2 = subgraph_2.node_count();
    let edges_count_group_2 = subgraph_2.edge_count();

    println!("Количество вершин в первой группе: {}", nodes_count_group_1);
    println!("Количество рёбер в первой группе: {}", edges_count_group_1);

    println!("Количество вершин во второй группе: {}", nodes_count_group_2);
    println!("Количество рёбер во второй группе: {}", edges_count_group_2);
}

// Функция для вычисления лапласиана графа
fn laplacian_matrix(graph: &DiGraph<&str, &str>) -> CsMat<f64> {
    let laplacian_dense = laplacian_dense_matrix(graph);
    let laplacian_dense_owned: Array2<f64> = laplacian_dense.map(|&x| x).to_owned();
    let laplacian_dense_view = laplacian_dense_owned.view();  // Используем метод view() для преобразования в представление ссылок
    CsMat::csr_from_dense(laplacian_dense_view, 1e-6)
}






// Функция для вычисления лапласиана графа в виде плотной матрицы
fn laplacian_dense_matrix(graph: &DiGraph<&str, &str>) -> Array2<f64> {
    let n = graph.node_count();
    let mut laplacian = Array2::zeros((n, n));

    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let source_degree = graph.neighbors_undirected(source).count() as f64;
        let target_degree = graph.neighbors_undirected(target).count() as f64;

        laplacian[[source.index(), target.index()]] = -1.0 / source_degree.sqrt();
        laplacian[[target.index(), source.index()]] = -1.0 / target_degree.sqrt();
        laplacian[[source.index(), source.index()]] += 1.0 / source_degree.sqrt();
        laplacian[[target.index(), target.index()]] += 1.0 / target_degree.sqrt();
    }

    laplacian
}

// Функция для вычисления Fiedler вектора
fn fiedler_vector(laplacian: &CsMat<f64>) -> CsVec<f64> {
    let (_, eigenvalues, eigenvectors) = laplacian.to_owned().symmetric_eigen(1, 1e-6);
    let fiedler_vector = eigenvectors.column(0).map(|&x| x);
    fiedler_vector
}

// Функция для нахождения три рёбер, которые разделят граф
fn find_partitions(graph: &DiGraph<&str, &str>, fiedler_vector: &CsVec<f64>) -> Vec<i32> {
    let mut cut_edges = Vec::new();

    let partitions: Vec<i32> = fiedler_vector
        .iter()
        .map(|&value| if value > 0.0 { 1 } else { -1 })
        .collect();

    // Находим рёбра с наименьшими весами, соединяющие вершины из разных частей графа
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let source_partition = partitions[source.index()];
        let target_partition = partitions[target.index()];

        if source_partition != target_partition {
            cut_edges.push((source.index() as i32, target.index() as i32));
            if cut_edges.len() == 3 {
                break;
            }
        }
    }

    cut_edges
}

// Функция для создания подграфа
fn create_subgraph<'a>(graph: &'a DiGraph<&'a str, &'a str>, partitions: &'a Vec<i32>, target_partition: i32) -> DiGraph<&'a str, &'a str> {
    let mut subgraph = DiGraph::<&str, &str>::new();

    for node in graph.node_indices() {
        let partition = partitions[node.index()];
        if partition == target_partition {
            subgraph.add_node(graph[node]);
        }
    }

    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let source_partition = partitions[source.index()];
        let target_partition = partitions[target.index()];

        if source_partition == target_partition && source_partition == target_partition {
            subgraph.add_edge(NodeIndex::new(source.index()), NodeIndex::new(target.index()), graph[edge]);
        }
    }

    subgraph
}
