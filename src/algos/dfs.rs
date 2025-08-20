use std::collections::{HashMap, HashSet};
use crate::core::graph::Graph;



impl Graph{
    pub fn dfs(&self, start: i32) ->Vec<i32>{
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    self.dfs_helper(start, &mut visited, &mut result);
    result
    }


    fn dfs_helper(&self, node: i32, visited: &mut HashSet<i32>,result:& mut Vec<i32>){
        visited.insert(node);
        result.push(node);

        if let Some(neighbors) = self.adj_list.get(&node){
            for &neighbor in neighbors{
                if !visited.contains(&neighbor){
                    self.dfs_helper(neighbor, visited, result);
                }
            }
        }
    }

    pub fn dfs_iterative(&self, start: i32) -> Vec<i32>{
        let mut visited= HashSet::new();
        let mut stack = vec![start];
        let mut result = Vec::new();


        while let Some(node) = stack.pop(){
            if !visited.contains(&node) {
                visited.insert(node);
                result.push(node);

                if let Some(neighbors) = self.adj_list.get(&node){

                    for &neighbor in neighbors{
                        if !visited.contains(&neighbor){
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }
        result

    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_dfs_single_node() {
        let mut graph = Graph::new(false);
        graph.add_edge(1,1);

        let result = graph.dfs(1);
        assert_eq!(result,vec![1]);

        let result_iter = graph.dfs_iterative(1);
        assert_eq!(result_iter,vec![1]);

    }
    #[test]
    fn test_dfs_isolated_node(){
        let mut graph = Graph::new(false);
        let res = graph.dfs(5);
        assert_eq!(res,vec![5]);

        let result_iter = graph.dfs_iterative(5);
        assert_eq!(result_iter,vec![5] );
    }
#[test]
fn test_dfs_lin_path(){
    let mut graph = Graph::new(false);
    graph.add_edge(1,2);

    graph.add_edge(2,3);
    graph.add_edge(3,4);

    let result = graph.dfs(1);
    assert_eq!(result.len(), 4);
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));
    assert!(result.contains(&4));
    assert_eq!(result[0], 1);
}

    #[test]
    fn test_dfs_directed_vs_undirected() {
        let mut directed_graph = Graph::new(true);
        directed_graph.add_edge(1, 2);
        directed_graph.add_edge(2, 3);

        let result_directed = directed_graph.dfs(1);
        assert_eq!(result_directed, vec![1, 2, 3]);

        // Starting from 3 in directed graph should only visit 3
        let result_from_3 = directed_graph.dfs(3);
        assert_eq!(result_from_3, vec![3]);

        let mut undirected_graph = Graph::new(false);
        undirected_graph.add_edge(1, 2);
        undirected_graph.add_edge(2, 3);

        let result_undirected = undirected_graph.dfs(3);
        assert_eq!(result_undirected.len(), 3);
        assert!(result_undirected.contains(&1));
        assert!(result_undirected.contains(&2));
        assert!(result_undirected.contains(&3));
    }

    #[test]
    fn test_dfs_cycle() {
        let mut graph = Graph::new(false);
        // Create a cycle: 1 - 2 - 3 - 1
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 1);

        let result = graph.dfs(1);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));

        // Each node should appear exactly once
        let mut counts = HashMap::new();
        for &node in &result {
            *counts.entry(node).or_insert(0) += 1;
        }
        for &count in counts.values() {
            assert_eq!(count, 1);
        }
    }

    #[test]
    fn test_dfs_tree_structure() {
        let mut graph = Graph::new(false);
        //       1
        //      / \
        //     2   3
        //    /   / \
        //   4   5   6
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 5);
        graph.add_edge(3, 6);

        let result = graph.dfs(1);
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], 1); // Should start with root

        // All nodes should be visited
        let expected_nodes = vec![1, 2, 3, 4, 5, 6];
        for node in expected_nodes {
            assert!(result.contains(&node));
        }
    }

    #[test]
    fn test_dfs_disconnected_components() {
        let mut graph = Graph::new(false);
        // Two disconnected components: (1-2) and (3-4)
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        // Starting from component 1
        let result1 = graph.dfs(1);
        assert_eq!(result1.len(), 2);
        assert!(result1.contains(&1));
        assert!(result1.contains(&2));
        assert!(!result1.contains(&3));
        assert!(!result1.contains(&4));

        // Starting from component 2
        let result2 = graph.dfs(3);
        assert_eq!(result2.len(), 2);
        assert!(result2.contains(&3));
        assert!(result2.contains(&4));
        assert!(!result2.contains(&1));
        assert!(!result2.contains(&2));
    }



    #[test]
    fn test_dfs_complete_graph() {
        let mut graph = Graph::new(false);
        // Complete graph with 4 nodes
        for i in 1..=4 {
            for j in (i+1)..=4 {
                graph.add_edge(i, j);
            }
        }

        let result = graph.dfs(1);
        assert_eq!(result.len(), 4);
        for i in 1..=4 {
            assert!(result.contains(&i));
        }
    }


    #[test]
    fn test_dfs_star_graph() {
        let mut graph = Graph::new(false);
        // Star graph: center node 1 connected to 2, 3, 4, 5
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(1, 5);

        let result = graph.dfs(1);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1); // Center should be first

        // Starting from a leaf should visit all nodes
        let result_from_leaf = graph.dfs(2);
        assert_eq!(result_from_leaf.len(), 5);
    }



    #[test]
    fn test_dfs_negative_nodes() {
        let mut graph = Graph::new(false);
        graph.add_edge(-1, -2);
        graph.add_edge(-2, -3);

        let result = graph.dfs(-1);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&-1));
        assert!(result.contains(&-2));
        assert!(result.contains(&-3));
    }

    #[test]
    fn test_dfs_large_numbers() {
        let mut graph = Graph::new(false);
        graph.add_edge(1_000_000, 2_000_000);
        graph.add_edge(2_000_000, 3_000_000);

        let result = graph.dfs(1_000_000);
        assert_eq!(result, vec![1_000_000, 2_000_000, 3_000_000]);
    }

    #[test]
    fn test_dfs_recursive_vs_iterative() {
        let mut graph = Graph::new(false);
        // Create a more complex graph
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 5);
        graph.add_edge(4, 6);
        graph.add_edge(5, 6);

        let result_recursive = graph.dfs(1);
        let result_iterative = graph.dfs_iterative(1);

        // Both should visit the same number of nodes
        assert_eq!(result_recursive.len(), result_iterative.len());

        // Both should visit all the same nodes (order might differ)
        let mut rec_set: HashSet<i32> = result_recursive.into_iter().collect();
        let mut iter_set: HashSet<i32> = result_iterative.into_iter().collect();
        assert_eq!(rec_set, iter_set);
    }

    #[test]
    fn test_dfs_performance_properties() {
        let mut graph = Graph::new(false);

        // Create a path of 1000 nodes
        for i in 0..999 {
            graph.add_edge(i, i + 1);
        }

        let result = graph.dfs(0);
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0], 0);
        assert_eq!(result[999], 999);

        // Verify all nodes are visited exactly once
        let mut visited_count = HashMap::new();
        for &node in &result {
            *visited_count.entry(node).or_insert(0) += 1;
        }

        for i in 0..1000 {
            assert_eq!(visited_count[&i], 1);
        }
    }

    #[test]
    fn test_dfs_self_loops() {
        let mut graph = Graph::new(true); // Directed graph with self-loops
        graph.add_edge(1, 1); // Self loop
        graph.add_edge(1, 2);
        graph.add_edge(2, 2); // Another self loop
        graph.add_edge(2, 3);

        let result = graph.dfs(1);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_dfs_empty_adjacency() {
        let mut graph = Graph::new(false);
        // Add a node that exists in adjacency list but has no neighbors
        graph.adj_list.insert(42, Vec::new());

        let result = graph.dfs(42);
        assert_eq!(result, vec![42]);
    }
}