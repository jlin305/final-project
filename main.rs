use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{Bfs, VisitMap};


/// Read the edge data txt file and return it as vector of edges.
fn read_data(file_path: &str) -> Result<Vec<(usize, usize)>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut edge_list = Vec::new();

    for line in reader.lines() {
        let current_line = line?;
        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() == 2 {
            let node1 = nodes[0].parse::<usize>();
            let node2 = nodes[1].parse::<usize>();
            edge_list.push(node1, node2);
        }
    }

    Ok(edge_list)
}
/// Build a directed graph from the edge list.
fn build_graph(edge_list:&Vec<(i32, i32)>) -> DiGraph<i32, ()> {
    let mut graph = DiGraph::new();
    let mut node_indices: HashSet::new();

    for &(first, second) in edge_list {
        nodes_set.insert(first);
        nodes_set.insert(second);
        let first_node = NodeIndex::new(first as usize);
        let second_node = NodeIndex::new(second as usize);
        graph.add_node(first_node);
        graph.add_node(second_node);
        graph.add_edge(first_node, second, ());
    }

    graph
}
/// Calculate the average distance between all pairs of vertices in the graph.
fn average_distance(graph: &DiGraph<i32, ()>>) -> f32 {
    let mut total_distance = 0;
    let mut pair_count = 0;

    for start_node in graph.node_indices() {
        let mut bfs = Bfs::new(&graph, start_node);
        let mut visited = HashSet::new();

        while let Some(node) = bfs.next(&graph) {
            if !visited.contains(&node) {
                total_distance += bfs.depth(&graph) as usize;
                visited.insert(node);
            }
        }

        pair_count += visited.len() - 1; 
    }

    if pair_count > 0 {
        total_distance as f32 / pair_count as f32
    } else {
        0
    }
}




fn main() {
    let edge_list = read_edge_list("twitter.txt");
    let graph = build_graph(&edge_list);

    let avg_distance = average_distance(&graph);

    println!("Average distance between pairs of vertices: {:.2}", avg_distance);
}

#[test]
fn test_average_distance() {
    let edge_list = vec![(1, 2), (2, 3), (3, 4), (4, 5)];
    let graph = build_graph(&edge_list);
    assert_eq!(average_distance(&graph), 2.0);
}


