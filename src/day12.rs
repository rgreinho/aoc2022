use itertools::Itertools;

use petgraph::{
    algo::dijkstra::dijkstra,
    // dot::{Config, Dot},
    graph::NodeIndex,
    Graph,
};
use std::fs;

pub fn day12a() -> String {
    let data = fs::read_to_string("assets/day12.txt").expect("Could not load file");
    let squares = parse_input_a(&data);
    let (nodes, graph, start, end) = build_graph(&squares);
    let steps = process_input_a(&nodes, &graph, start, end);
    steps.to_string()
}

pub fn day12b() -> String {
    let data = fs::read_to_string("assets/day12.txt").expect("Could not load file");
    let squares = parse_input_a(&data);
    let (nodes, graph, _start, end) = build_graph(&squares);
    let steps = process_input_b(&squares, &nodes, &graph, end);
    steps.to_string()
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    label: char,
    weight: u8,
}

pub fn parse_input_a(input: &str) -> Vec<Vec<Square>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let weight = match c {
                        'S' => 'a' as u8,
                        'E' => 'z' as u8,
                        _ => c as u8,
                    };
                    Square { label: c, weight }
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn build_graph(
    squares: &Vec<Vec<Square>>,
) -> (
    Vec<Vec<NodeIndex>>,
    Graph<Square, u8>,
    (usize, usize),
    (usize, usize),
) {
    let mut graph = Graph::<Square, u8>::new();

    // Create the nodes.
    let row_count = squares.len();
    let col_count = squares[0].len();
    // dbg!(&row_count, &col_count);
    let nodes = squares
        .iter()
        .map(|row| row.iter().map(|&s| graph.add_node(s)).collect_vec())
        .collect_vec();
    // for i in 0..row_count {
    //     for j in 0..col_count {
    //         let square = squares[i][j];
    //         print!("{}", square.label);
    //     }
    //     println!();
    // }

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, square_rows) in squares.iter().enumerate() {
        for (j, square) in square_rows.iter().enumerate() {
            // Lookup for the start and finish positions.
            if square.label == 'S' {
                start = (i, j);
            }
            if square.label == 'E' {
                end = (i, j);
            }

            // Build the edges.
            let neighbors: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for neighbor in neighbors {
                let x = j as i32 + neighbor.0;
                if x < 0 || x >= col_count as i32 {
                    continue;
                }
                let y = i as i32 + neighbor.1;
                if y < 0 || y >= row_count as i32 {
                    continue;
                }
                // println!("[{i}, {j}] -> [{x}, {y}]");
                let neighbor_square = squares[y as usize][x as usize];
                if neighbor_square.weight <= square.weight
                    || neighbor_square.weight == square.weight + 1
                {
                    graph.add_edge(nodes[i][j], nodes[y as usize][x as usize], 1);
                }
            }
        }
    }

    // Render the graph.
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    dbg!(&start, &end);

    (nodes, graph, start, end)
}

pub fn build_graph_rev(
    squares: &Vec<Vec<Square>>,
) -> (
    Vec<Vec<NodeIndex>>,
    Graph<Square, u8>,
    (usize, usize),
    (usize, usize),
) {
    let mut graph = Graph::<Square, u8>::new();

    // Create the nodes.
    let row_count = squares.len();
    let col_count = squares[0].len();
    // dbg!(&row_count, &col_count);
    let nodes = squares
        .iter()
        .map(|row| row.iter().map(|&s| graph.add_node(s)).collect_vec())
        .collect_vec();
    // for i in 0..row_count {
    //     for j in 0..col_count {
    //         let square = squares[i][j];
    //         print!("{}", square.label);
    //     }
    //     println!();
    // }

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, square_rows) in squares.iter().enumerate() {
        for (j, square) in square_rows.iter().enumerate() {
            // Lookup for the start and finish positions.
            if square.label == 'S' {
                start = (i, j);
            }
            if square.label == 'E' {
                end = (i, j);
            }

            // Build the edges.
            let neighbors: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            for neighbor in neighbors {
                let x = j as i32 + neighbor.0;
                if x < 0 || x >= col_count as i32 {
                    continue;
                }
                let y = i as i32 + neighbor.1;
                if y < 0 || y >= row_count as i32 {
                    continue;
                }
                // println!("[{i}, {j}] -> [{x}, {y}]");
                let neighbor_square = squares[y as usize][x as usize];
                if neighbor_square.weight <= square.weight
                    || neighbor_square.weight == square.weight + 1
                {
                    graph.add_edge(nodes[y as usize][x as usize], nodes[i][j], 1);
                }
            }
        }
    }

    // Render the graph.
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    dbg!(&start, &end);

    (nodes, graph, start, end)
}

pub fn process_input_a(
    nodes: &Vec<Vec<NodeIndex>>,
    graph: &Graph<Square, u8>,
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    // Find the shortest path.
    let end_node = nodes[end.0][end.1];
    let shortest = dijkstra(&graph, nodes[start.0][start.1], Some(end_node), |_| 1);
    // dbg!(&shortest);
    // Why does it give me the value for all the nodes when I did specify the goal???
    shortest[&end_node]
}

pub fn process_input_b(
    squares: &Vec<Vec<Square>>,
    nodes: &Vec<Vec<NodeIndex>>,
    graph: &Graph<Square, u8>,
    start: (usize, usize),
) -> usize {
    let shortest = dijkstra(&graph, nodes[start.0][start.1], None, |_| 1);
    dbg!(&shortest);

    // Find all 'a's.
    let mut all_as: Vec<NodeIndex> = Vec::new();
    for (i, row) in squares.iter().enumerate() {
        for (j, square) in row.iter().enumerate() {
            if square.weight == 'a' as u8 {
                println!("[{i},{j}]");
                all_as.push(nodes[j][i]);
            }
        }
    }

    let res = all_as
        .iter()
        .map(|nodeindex| shortest[&nodeindex])
        .collect_vec();
    dbg!(&res);
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_day12a_sample() {
        let squares = parse_input_a(RAW_INPUT);
        let (nodes, graph, start, end) = build_graph(&squares);
        let steps = process_input_a(&nodes, &graph, start, end);
        assert_eq!(steps, 31);
    }

    #[test]
    #[ignore]
    fn test_day12b_sample() {
        let squares = parse_input_a(RAW_INPUT);
        let (nodes, graph, _start, end) = build_graph_rev(&squares);
        let steps = process_input_b(&squares, &nodes, &graph, end);
        assert_eq!(steps, 29);
    }
}
