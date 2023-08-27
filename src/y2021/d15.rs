use crate::utils::Input;
use anyhow::Result;
use log::debug;
use petgraph::algo::dijkstra;
use petgraph::{Directed, Graph};

pub fn run(input: &Input) -> Result<(u32, usize)> {
    let mut output = (0, 0);
    let mut prev: Option<Vec<u32>> = None;
    let mut graph: Graph<(), u32, Directed, usize> = Graph::default();

    for (n, line) in input.lines().enumerate() {
        let line: Vec<u32> = line.chars().map(|c| c as u32 - 48).collect();
        if let Some(prev) = prev {
            assert_eq!(prev.len(), line.len());
            graph.extend_with_edges((0..line.len()).flat_map(|i| {
                vec![
                    (n * line.len() + i, (n - 1) * line.len() + i, prev[i]), // Going up
                    ((n - 1) * line.len() + i, n * line.len() + i, line[i]), // Going down
                ]
            }));
        };
        graph.extend_with_edges(
            line.iter()
                .enumerate()
                .zip(line.iter().enumerate().skip(1))
                .flat_map(|((i1, v1), (i2, v2))| {
                    vec![
                        (n * line.len() + i1, n * line.len() + i2, *v2), // Going right
                        (n * line.len() + i2, n * line.len() + i1, *v1), // Going left
                    ]
                }),
        );
        prev = Some(line);
    }

    let start = graph.node_indices().next().unwrap();
    let end = graph.node_indices().next_back().unwrap();
    let mut border_nodes = vec![];
    for node in graph
        .node_indices()
        .filter(|&i| graph.neighbors(i).count() < 4)
    {
        border_nodes.push(node);
    }
    debug!("{:?} {:?} - {}", start, end, border_nodes.len());

    output.0 = *dijkstra(&graph, start, Some(end), |e| *e.weight())
        .get(&end)
        .unwrap();

    Ok(output)
}
