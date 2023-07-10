// Main Function //

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: String) {
        self.nodes.insert(node, HashSet::new());
    }

    fn add_edge(&mut self, node1: String, node2: &String) {
        if !self.nodes.contains_key(&node1) {
            return; // node1 doesn't exist, so don't add the edge
        }

        // node1 exists, so add the edge
        self.nodes.get_mut(&node1).unwrap().insert(node2.clone());
        self.nodes
            .entry(node2.to_string())
            .or_insert(HashSet::new())
            .insert(node1);
    }

    fn get_neighbors(&self, node: &String) -> &HashSet<String> {
        self.nodes.get(node).unwrap()
    }

    fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    fn get_edge_count(&self) -> usize {
        let mut count = 0;
        for (_, neighbors) in self.nodes.iter() {
            count += neighbors.len();
        }
        count / 2
    }

    fn get_average_degree(&self) -> f64 {
        let mut sum = 0;
        for (_, neighbors) in self.nodes.iter() {
            sum += neighbors.len();
        }
        sum as f64 / self.nodes.len() as f64
    }

    fn get_degree_distribution(&self) -> HashMap<usize, usize> {
        let mut distribution = HashMap::new();
        for (_, neighbors) in self.nodes.iter() {
            let degree = neighbors.len();
            let count = distribution.entry(degree).or_insert(0);
            *count += 1;
        }
        distribution
    }

    fn get_diameter(&self) -> usize {
        let mut diameter = 0;
        for (node, _) in self.nodes.iter() {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((node, 0));
            while !queue.is_empty() {
                let (node, depth) = queue.pop_front().unwrap();
                if depth > diameter {
                    diameter = depth;
                }
                for neighbor in self.get_neighbors(node) {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back((neighbor, depth + 1));
                    }
                }
            }
        }
        diameter
    }

    fn get_average_path_length(&self) -> f64 {
        let mut sum = 0;
        let mut count = 0;
        for (node, _) in self.nodes.iter() {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((node, 0));
            while !queue.is_empty() {
                let (node, depth) = queue.pop_front().unwrap();
                sum += depth;
                count += 1;
                for neighbor in self.get_neighbors(node) {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back((neighbor, depth + 1));
                    }
                }
            }
        }
        sum as f64 / count as f64
    }
}

fn main() {
    let mut graph = Graph::new();
    let mut file = File::open("actors.csv").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    for line in buffer.lines() {
        let mut actors = line.split(";");
        let actor1 = actors.next().unwrap().to_string();
        let actor2 = actors.next().unwrap().to_string();
        graph.add_node(actor1.clone());
        graph.add_node(actor2.clone());
        graph.add_edge(actor1, &actor2);
    }
    println!("The node count is {}", graph.get_node_count());
    println!("The edge count is {}", graph.get_edge_count());
    println!("The Average degree is {}", graph.get_average_degree());
    println!(
        "The Average degree of distribution is {:?}",
        graph.get_degree_distribution()
    );
    println!("The Depth of the graph is {}", graph.get_diameter());
    println!("{}", graph.get_average_path_length());

}
