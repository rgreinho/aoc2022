use crate::nomstr;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while},
    character::{
        complete::{self, alpha1},
        is_alphabetic, is_space,
    },
    error::Error,
    multi::separated_list1,
    sequence::preceded,
    Finish, IResult,
};
use petgraph::{
    // algo::dijkstra::dijkstra,
    dot::{Config, Dot},
    graph::NodeIndex,
    Graph,
};
use std::{collections::HashMap, fs, iter::zip, str::FromStr};

pub fn day16a() -> String {
    let data = fs::read_to_string("assets/day16.txt").expect("Could not load file");
    data
}

pub fn day16b() -> String {
    let data = fs::read_to_string("assets/day16.txt").expect("Could not load file");
    data
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Valve {
    label: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    pub fn parse(i: &str) -> IResult<&str, Valve> {
        let (i, label) = preceded(tag("Valve "), alpha1)(i)?;
        // let (i, flow_rate) = preceded(tag(" has flow rate="), complete::u32)(i)?;
        // let (i, tunnels) = preceded(
        //     alt((
        //         tag("; tunnels lead to valves "),
        //         tag("; tunnel leads to valve "),
        //     )),
        //     separated_list1(tag(", "), alpha1),
        // )(i)?;

        // Just another way to parse it by skipping characters instead.
        let (i, _) = take_while(|c| is_alphabetic(c as u8) || is_space(c as u8))(i)?;
        let (i, flow_rate) = preceded(tag("="), complete::u32)(i)?;
        let (i, _) = take_while(|c: char| !c.is_uppercase())(i)?;
        let (i, tunnels) = separated_list1(tag(", "), alpha1)(i)?;

        Ok((
            i,
            Valve {
                label: label.to_string(),
                flow_rate,
                tunnels: tunnels.iter().map(|t| t.to_string()).collect(),
            },
        ))
    }
}
nomstr!(Valve);

pub fn parse_input_a(i: &str) -> Vec<Valve> {
    i.lines().map(|l| Valve::from_str(l).unwrap()).collect_vec()
}

pub fn build_graph(valves: &[Valve]) -> Graph<(&str, u32), u32> {
    let mut graph = Graph::<(&str, u32), u32>::new();

    // Build nodes.
    let nodes: Vec<NodeIndex> = valves
        .iter()
        .map(|v| graph.add_node((v.label.as_str(), v.flow_rate)))
        .collect();

    let zipped = zip(valves, nodes)
        .map(|(v, n)| (String::from(&v.label), n))
        .collect::<HashMap<String, NodeIndex>>();

    // Build edges.
    for valve in valves {
        for tunnel in &valve.tunnels {
            graph.add_edge(
                *zipped.get(&valve.label).unwrap(),
                *zipped.get(tunnel).unwrap(),
                valve.flow_rate,
            );
        }
    }

    // Render the graph.
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    //
    graph
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_day16a_sample() {
        let valves = parse_input_a(RAW_INPUT);
        let _graph = build_graph(&valves);
    }

    #[test]
    fn test_day16b_sample() {}

    #[test]
    fn test_parse_valve() {
        let valve = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
            .parse::<Valve>()
            .unwrap();
        assert_eq!(
            valve,
            Valve {
                label: String::from("AA"),
                flow_rate: 0,
                tunnels: vec![String::from("DD"), String::from("II"), String::from("BB"),]
            }
        );
    }
}
