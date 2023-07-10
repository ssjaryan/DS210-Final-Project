# DS210-Final-Project


# Actors and Movies Graph Analysis - Six Degrees of Separation


## Why This Project
This project was undertaken to explore the use of graph data structures in analyzing relationships between actors and movies. The graph data structure 
is a powerful tool for modeling relationships between objects, and this project provided an opportunity to apply graph analysis techniques to a real-world problem. 
Additionally, the project provided an opportunity to explore different graph algorithms and understand their applications in real-world scenarios.

## Dataset
- The dataset used in this project consists of a list of actors and movies they have appeared in from IMDB. 
- The dataset was chosen as it provided a good starting point for analyzing relationships between two data points and implements six degrees of separation.


## Overview
This project explores the use of graph data structures in analyzing relationships between actors and movies they have appeared in. 
The program has two Implementations: `main.rs` and `extra_fn.rs`. 

### Main Function
- The `Main Function` creates an undirected graph using a HashMap where the keys are nodes represented as strings and the values are
  sets of neighboring nodes represented as HashSets. The graph struct has methods for adding nodes and edges, getting the number of nodes
  and edges in the graph, calculating the average degree of the nodes, getting the degree distribution, finding the diameter of the graph,
  and calculating the average path length between all pairs of nodes in the graph. 

- The `Main Function` reads in a list of actors and movies from a file and uses this information to construct a graph of the actors and
  the movies they have appeared in. The program uses BFS in the `get_diameter` and `get_average_path_length` methods to find the shortest
  path between each pair of nodes in the graph.

### Extra Function
- The `Extra Function` represents a directed graph with nodes that have a string field `actors_movies` and a vector of integers `num_nodes` 
representing the indices of the neighbors of the node in the nodes vector of the Graph struct. The `init` method reads in a list of actors 
and movies from a file and uses this information to construct a graph of the actors and the movies they have appeared in. 

- The `shortest_path` method uses BFS to find the shortest path between two nodes (actors) in the graph. The method starts at the source node 
and uses a queue to visit its neighbors in a breadth-first fashion. The method keeps track of the parent of each node visited and updates the 
parent field of each node as it is visited. When the end node is reached, the method follows the chain of parents back to the start node to 
reconstruct the shortest path. The method outputs the shortest path as a list of the actors_movies values of the nodes in the path.


## Results
- The `Main Function` successfully outputted the minimum degree of distribution between all actors present in the data file, which was the expected result. 

- The `Extra Function` successfully outputted the shortest path between two actors, given as input by the user.

## Conclusion
This project allowed me to explore the use of graph data structures in analyzing relationships between actors and movies they have appeared in. 
It was a valuable learning experience in graph analysis techniques and their applications in real-world scenarios. The program successfully implemented 
two different graph data structures and provided useful methods for analyzing the graphs. Overall, this project demonstrated the power of graph analysis 
in modeling relationships between objects and solving real-world problems. This project showcases my ability to work with data structures, algorithms, 
and file input/output in a programming project, as well as my ability to implement different graph data structures and algorithms to solve real-world problems.
