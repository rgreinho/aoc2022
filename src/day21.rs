use crate::nomstr;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, space1},
    combinator::value,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded},
    Finish, IResult, Parser,
};
use petgraph::{
    dot::{Config, Dot},
    graph::{self, NodeIndex},
    visit::{Topo, Walker},
    Graph,
};
use std::{collections::HashMap, fs, str::FromStr};

pub fn day21a() -> String {
    let data = fs::read_to_string("assets/day21.txt").expect("Could not load file");
    let nodes = parse_input_a(&data);
    let (graph, node_map) = build_graph(&nodes);
    let root = process_input_a(&graph, &node_map);
    root.to_string()
}

pub fn day21b() -> String {
    let data = fs::read_to_string("assets/day21.txt").expect("Could not load file");
    data
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Divide,
    Multiply,
    Subtract,
}

impl Operator {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Operator::Add, tag("+")),
            value(Operator::Divide, tag("/")),
            value(Operator::Multiply, tag("*")),
            value(Operator::Subtract, tag("-")),
        ))(i)
    }
}
nomstr!(Operator);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation<'a> {
    Number(i64),
    Compute {
        lhs: &'a str,
        operator: Operator,
        rhs: &'a str,
    },
}

impl<'a> Operation<'a> {
    fn compute(i: &'a str) -> IResult<&str, Self> {
        let (i, lhs) = alpha1(i)?;
        let (i, operator) = delimited(space1, Operator::parse, space1)(i)?;
        let (i, rhs) = alpha1(i)?;
        Ok((i, Operation::Compute { lhs, operator, rhs }))
    }

    pub fn parse(i: &'a str) -> IResult<&str, Self> {
        alt((
            complete::i64.map(|n| Operation::Number(n)),
            Operation::compute,
        ))(i)
    }
}
// nomstr!(Operation);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Node<'a> {
    id: &'a str,
    operation: Operation<'a>,
}

impl<'a> Node<'a> {
    pub fn parse(i: &'a str) -> IResult<&str, Self> {
        let (i, id) = alpha1(i)?;
        let (i, operation) = preceded(tag(": "), Operation::parse)(i)?;
        Ok((i, Node { id, operation }))
    }
}
// nomstr!(Node);

pub fn parse_input_a(i: &str) -> Vec<Node> {
    let (_, nodes) = separated_list1(line_ending, Node::parse)(i).unwrap();
    nodes
}

pub fn build_graph<'a>(nodes: &'a [Node]) -> (Graph<&'a str, ()>, HashMap<NodeIndex, Node<'a>>) {
    let mut graph = Graph::<&'a str, ()>::new();

    // Add nodes.
    let mut node_id_map: HashMap<&str, NodeIndex> = HashMap::new();
    let mut node_map: HashMap<NodeIndex, Node> = HashMap::new();
    for node in nodes {
        let node_index = graph.add_node(node.id);
        node_id_map.insert(node.id, node_index);
        node_map.insert(node_index, *node);
    }

    // let node_id_map = nodes
    //     .iter()
    //     .map(|n| (n.id, graph.add_node(n.id)))
    //     .collect::<HashMap<&str, NodeIndex>>();

    // let node_map = nodes
    //     .iter()
    //     .map(|n| (n.id, *n))
    //     .collect::<HashMap<&str, Node>>();

    // Add edges.
    for node in nodes {
        if let Operation::Compute {
            lhs,
            operator: _,
            rhs,
        } = node.operation
        {
            let left = node_id_map.get(&lhs).unwrap();
            let right = node_id_map.get(&rhs).unwrap();
            let to = node_id_map.get(&node.id).unwrap();
            graph.add_edge(*left, *to, ());
            graph.add_edge(*right, *to, ());
        }
    }

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    (graph, node_map)
}

pub fn process_input_a<'a>(
    graph: &Graph<&'a str, ()>,
    node_map: &HashMap<NodeIndex, Node<'a>>,
) -> i64 {
    let mut node_values: HashMap<&str, i64> = HashMap::new();
    let topological = Topo::new(&graph);
    for node_id in topological.iter(graph) {
        let current_node = node_map.get(&node_id).unwrap();
        match current_node.operation {
            Operation::Number(n) => {
                node_values.insert(current_node.id, n);
            }
            Operation::Compute { lhs, operator, rhs } => {
                let left = node_values.get(&lhs).unwrap();
                let right = node_values.get(&rhs).unwrap();
                match operator {
                    Operator::Add => {
                        node_values.insert(current_node.id, left + right);
                    }
                    Operator::Divide => {
                        node_values.insert(current_node.id, left / right);
                    }
                    Operator::Multiply => {
                        node_values.insert(current_node.id, left * right);
                    }
                    Operator::Subtract => {
                        node_values.insert(current_node.id, left - right);
                    }
                }
            }
        }
    }
    *node_values.get("root").unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "dbpl: 5
root: pppw + sjmn
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_day21a_sample() {
        let nodes = parse_input_a(RAW_INPUT);
        let (graph, node_map) = build_graph(&nodes);
        let root = process_input_a(&graph, &node_map);
        assert_eq!(root, 152);
    }

    #[test]
    fn test_day21b_sample() {}
}
