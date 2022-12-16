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





// Extra Function //

/*
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Node {
    actors_movies: String,
    num_nodes: Vec<usize>,
    visited: bool,
    parent: usize,
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn init(path: &str) -> Graph {
        let mut graph = Graph { nodes: vec![] };
        let f = File::open(path).unwrap();
        let file = BufReader::new(&f);
        for (_num, line) in file.lines().enumerate() {
            let line_str = line.unwrap();
            let split_nodes: Vec<&str> = line_str.split(";").collect();
            let mut itr = graph.search(split_nodes[0]);
            if itr == <usize>::max_value() {
                graph.nodes.push(Node {
                    actors_movies: split_nodes[0].to_string(),
                    num_nodes: vec![],
                    visited: false,
                    parent: <usize>::max_value(),
                });
                itr = graph.nodes.len() - 1
            }
            let mut mind = graph.search(split_nodes[1]);
            if mind == <usize>::max_value() {
                graph.nodes.push(Node {
                    actors_movies: split_nodes[1].to_string(),
                    num_nodes: vec![],
                    visited: false,
                    parent: <usize>::max_value(),
                });
                let len = graph.nodes.len() - 1;
                graph.nodes[len].num_nodes.push(itr);
                mind = graph.nodes.len() - 1
            } else {
                graph.nodes[mind].num_nodes.push(itr);
            }
            if itr == <usize>::max_value() {
                let len = graph.nodes.len() - 1;
                graph.nodes[len].num_nodes.push(mind);
            } else {
                graph.nodes[itr].num_nodes.push(mind);
            }
        }
        return graph;
    }

    fn search(&mut self, actors_movies: &str) -> usize {
        for i in 0..self.nodes.len() {
            if self.nodes[i].actors_movies == actors_movies {
                return i;
            }
        }
        return <usize>::max_value();
    }

    fn shortest_path(&mut self, actor_1: &str, actor_2: &str) {
        let mut queue = VecDeque::new();
        let actor_1_index = self.search(actor_1);
        let mut current_index = actor_1_index;
        let actor_2_index = self.search(actor_2);
        // Mark the actor_1 node as checked and add it to the queue
        self.nodes[current_index].visited = true;
        queue.push_back(current_index);

        // Continue searching until the queue is empty
        while !queue.is_empty() {
            current_index = queue.pop_front().unwrap();
            // If we reached the actor_2 node, stop searching
            if current_index == actor_2_index {
                break;
            }
            // here I get the number of neighbors of the current node
            let num_neighbors = self.nodes[current_index].num_nodes.len();
            // then i iterate through the # of users in current_index
            for i in 0..num_neighbors {
                let neighbor_index = self.nodes[current_index].num_nodes[i];

                // If the neighbor has not been visited yet, mark it as visited
                // and add it to the queue for further exploration
                if !self.nodes[neighbor_index].visited {
                    self.nodes[neighbor_index].parent = current_index;
                    self.nodes[neighbor_index].visited = true;
                    //self.nodes[neighbor_index].parent = current_index;
                    queue.push_back(neighbor_index);
                }
            }
        }
        // Trace back from the actor_2 node to the actor_1 node, printing the names of each node along the way
        let mut node_itr = 0;
        loop {
            println!("{}", self.nodes[current_index].actors_movies);
            // If we reach the actor_1 node, break and stop printing while going back
            if current_index == actor_1_index {
                break;
            }
            // Print the name of any movies that connect the nodes in the graph
            if node_itr % 2 == 0 {
                print!("  Movie actor appeared in: ")
            }
            node_itr += 1;
            current_index = self.nodes[current_index].parent;
        }
    }
}

fn main() {
    let mut graph = Graph::init("actors.csv");
    graph.shortest_path("John Carradine", "Tom Hanks");
} */

