use crate::utils::Input;
use anyhow::Result;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, VecDeque};

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut nodes: HashMap<String, NodeIndex<u8>> = HashMap::new();
    let mut graph: UnGraph<NodeType, (), u8> = UnGraph::default();

    let mut get_node = |graph: &mut UnGraph<NodeType, (), u8>, name: &str| match nodes.get(name) {
        Some(idx) => *idx,
        None => {
            let idx = graph.add_node(match name {
                "start" => NodeType::Start,
                "end" => NodeType::End,
                _ => {
                    if name.chars().next().unwrap().is_lowercase() {
                        NodeType::SmallCave
                    } else {
                        NodeType::BigCave
                    }
                }
            });
            nodes.insert(name.to_string(), idx);
            idx
        }
    };
    for line in input.lines() {
        if let Some((from, to)) = line.split_once('-') {
            let from = get_node(&mut graph, from);
            let to = get_node(&mut graph, to);
            graph.add_edge(from, to, ());
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back(Path {
        visited: Vec::with_capacity(16),
        next: get_node(&mut graph, "start"),
    });

    // Part1: enumerate all path without backtracking in small caves
    let (count1, mut backtracks) = traverse(&graph, &mut queue, true);
    // Part2: consider additional path with one backtrack only
    let (count2, _) = traverse(&graph, &mut backtracks, false);
    Ok((count1, count1 + count2))
}

#[derive(Clone)]
struct Path {
    visited: Vec<NodeIndex<u8>>,
    next: NodeIndex<u8>,
}

impl Path {
    fn contains(&self, value: NodeIndex<u8>) -> bool {
        self.next == value || self.visited.contains(&value)
    }
    fn copy_with(&self, next: NodeIndex<u8>) -> Self {
        let mut visited = self.visited.clone();
        visited.push(self.next);
        Self { visited, next }
    }
}

enum NodeType {
    Start,
    End,
    BigCave,
    SmallCave,
}

fn traverse(
    graph: &UnGraph<NodeType, (), u8>,
    queue: &mut VecDeque<Path>,
    allow_backtracks: bool,
) -> (usize, VecDeque<Path>) {
    let mut count = 0;
    let mut backtracks = VecDeque::new();
    while let Some(path) = queue.pop_front() {
        for next in graph.neighbors(path.next) {
            match graph.node_weight(next).unwrap() {
                NodeType::Start => {}
                NodeType::End => count += 1,
                NodeType::BigCave => queue.push_back(path.copy_with(next)),
                NodeType::SmallCave => {
                    if path.contains(next) {
                        if allow_backtracks {
                            backtracks.push_back(path.copy_with(next))
                        }
                    } else {
                        queue.push_back(path.copy_with(next))
                    }
                }
            }
        }
    }
    (count, backtracks)
}
