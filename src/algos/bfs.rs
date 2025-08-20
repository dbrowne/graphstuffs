use std::collections::{HashSet,HashMap, VecDeque};
use crate::core::graph::Graph;


impl Graph{
    pub fn bfs(&self, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut queue  = VecDeque::new();
    let mut result = Vec::new();

    queue.push_back(start);
    visited.insert(start);

        while let Some(node) = queue.pop_front(){
            result.push(node);

            if let Some(neighbors) = self.adj_list.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        result
    }


    fn reconstruct_path(&self, parent: &HashMap<i32,i32>, start: i32, end: i32) -> Vec<i32> {
        let mut path = Vec::new();
        let mut current = end;

        while current != start {
            path.push(current);
            current = parent[&current];

        }
        path.push(start);
        path.reverse();
        path

    }

    pub fn shortest_path_bfs(&self, start: i32, end: i32) -> Option<Vec<i32>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front(){
            if node == end {
                return Some(self.reconstruct_path(&parent, start, end));
            }
            if let Some(neighbors) = self.adj_list.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        parent.insert(neighbor, node);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        None
    }


    pub fn bfs_distances(&self, start:i32) -> HashMap<i32,i32>{
    let mut distances: HashMap<i32,i32> = HashMap::new();
        let mut visited: HashSet<i32> = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);
        distances.insert(start,0);

        while let Some(node ) = queue.pop_front(){
            if let Some(neighbors) = self.adj_list.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        distances.insert(neighbor,distances[&node] + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs_single_node(){
        let graph = Graph::new(false);
        let res = graph.bfs(1);
        assert_eq!(res, vec![1]);
        // self loop
        let mut graph_self = Graph::new(false);
        graph_self.add_edge(1, 1);
        let result_self = graph_self.bfs(1);
        assert_eq!(result_self, vec![1]);

    }

    #[test]
    fn test_bfs_linear_path() {
        let mut graph = Graph::new(false);
        // 1 - 2 - 3 - 4
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let result = graph.bfs(1);
        assert_eq!(result, vec![1, 2, 3, 4]);

        // Test from different starting points
        let result_from_3 = graph.bfs(3);
        assert_eq!(result_from_3, vec![3, 2, 4, 1]);
    }


    #[test]
    fn test_bfs_level_order() {
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

        let result = graph.bfs(1);

        // BFS should visit level by level
        assert_eq!(result[0], 1);          // Level 0
        assert!(result[1..3].contains(&2) && result[1..3].contains(&3)); // Level 1

        // Level 2 (order may vary within level)
        let level_2 = &result[3..];
        assert!(level_2.contains(&4));
        assert!(level_2.contains(&5));
        assert!(level_2.contains(&6));
        assert_eq!(level_2.len(), 3);
    }

    #[test]
    fn test_bfs_directed_vs_undirected() {
        // Directed graph: 1 -> 2 -> 3
        let mut directed_graph = Graph::new(true);
        directed_graph.add_edge(1, 2);
        directed_graph.add_edge(2, 3);

        let result_directed = directed_graph.bfs(1);
        assert_eq!(result_directed, vec![1, 2, 3]);

        // Starting from 3 should only reach 3
        let result_from_3 = directed_graph.bfs(3);
        assert_eq!(result_from_3, vec![3]);

        // Undirected graph: 1 - 2 - 3
        let mut undirected_graph = Graph::new(false);
        undirected_graph.add_edge(1, 2);
        undirected_graph.add_edge(2, 3);

        let result_undirected = undirected_graph.bfs(3);
        assert_eq!(result_undirected, vec![3, 2, 1]);
    }

    #[test]
    fn test_bfs_cycle() {
        let mut graph = Graph::new(false);
        // Create a cycle: 1 - 2 - 3 - 4 - 1
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 1);

        let result = graph.bfs(1);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 1);

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
    fn test_bfs_disconnected_components() {
        let mut graph = Graph::new(false);
        // Two disconnected components: (1-2) and (3-4)
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        // BFS from component 1
        let result1 = graph.bfs(1);
        assert_eq!(result1.len(), 2);
        assert!(result1.contains(&1));
        assert!(result1.contains(&2));
        assert!(!result1.contains(&3));
        assert!(!result1.contains(&4));

        // BFS from component 2
        let result2 = graph.bfs(3);
        assert_eq!(result2.len(), 2);
        assert!(result2.contains(&3));
        assert!(result2.contains(&4));
        assert!(!result2.contains(&1));
        assert!(!result2.contains(&2));
    }

    #[test]
    fn test_bfs_complete_graph() {
        let mut graph = Graph::new(false);
        // Complete graph K4
        for i in 1..=4 {
            for j in (i+1)..=4 {
                graph.add_edge(i, j);
            }
        }

        let result = graph.bfs(1);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 1);

        // All other nodes should be at level 1 (directly connected)
        for &node in &result[1..] {
            assert!(vec![2, 3, 4].contains(&node));
        }
    }

    #[test]
    fn test_shortest_path_bfs_simple() {
        let mut graph = Graph::new(false);
        // 1 - 2 - 3 - 4
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let path = graph.shortest_path_bfs(1, 4).unwrap();
        assert_eq!(path, vec![1, 2, 3, 4]);

        let path_reverse = graph.shortest_path_bfs(4, 1).unwrap();
        assert_eq!(path_reverse, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_shortest_path_bfs_with_alternatives() {
        let mut graph = Graph::new(false);
        //   1 --- 4
        //   |     |
        //   2 --- 3
        graph.add_edge(1, 2);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let path = graph.shortest_path_bfs(1, 3).unwrap();
        // Should take direct path: either 1->2->3 or 1->4->3 (both length 3)
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], 1);
        assert_eq!(path[2], 3);
    }

    #[test]
    fn test_shortest_path_bfs_no_path() {
        let mut graph = Graph::new(false);
        // Disconnected: 1-2 and 3-4
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        let path = graph.shortest_path_bfs(1, 3);
        assert!(path.is_none());

        let path_reverse = graph.shortest_path_bfs(4, 1);
        assert!(path_reverse.is_none());
    }

    #[test]
    fn test_shortest_path_bfs_same_node() {
        let mut graph = Graph::new(false);
        graph.add_edge(1, 2);

        let path = graph.shortest_path_bfs(1, 1).unwrap();
        assert_eq!(path, vec![1]);
    }

    #[test]
    fn test_bfs_distances() {
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

        let distances = graph.bfs_distances(1);

        assert_eq!(distances[&1], 0);
        assert_eq!(distances[&2], 1);
        assert_eq!(distances[&3], 1);
        assert_eq!(distances[&4], 2);
        assert_eq!(distances[&5], 2);
        assert_eq!(distances[&6], 2);
    }

    #[test]
    fn test_bfs_star_graph() {
        let mut graph = Graph::new(false);
        // Center node 1 connected to 2, 3, 4, 5
        for i in 2..=5 {
            graph.add_edge(1, i);
        }

        let result = graph.bfs(1);
        assert_eq!(result[0], 1);
        assert_eq!(result.len(), 5);

        // All other nodes should be at distance 1
        for &node in &result[1..] {
            assert!(vec![2, 3, 4, 5].contains(&node));
        }

        // Test distances
        let distances = graph.bfs_distances(1);
        assert_eq!(distances[&1], 0);
        for i in 2..=5 {
            assert_eq!(distances[&i], 1);
        }
    }

    #[test]
    fn test_bfs_negative_nodes() {
        let mut graph = Graph::new(false);
        graph.add_edge(-1, -2);
        graph.add_edge(-2, -3);

        let result = graph.bfs(-1);
        assert_eq!(result, vec![-1, -2, -3]);

        let path = graph.shortest_path_bfs(-1, -3).unwrap();
        assert_eq!(path, vec![-1, -2, -3]);
    }

    #[test]
    fn test_bfs_large_numbers() {
        let mut graph = Graph::new(false);
        graph.add_edge(1000000, 2000000);
        graph.add_edge(2000000, 3000000);

        let result = graph.bfs(1000000);
        assert_eq!(result, vec![1000000, 2000000, 3000000]);
    }

    #[test]
    fn test_bfs_breadth_first_property() {
        let mut graph = Graph::new(false);
        // Create a graph where DFS and BFS would give different orders
        //     1
        //   / | \
        //  2  3  4
        //  |  |  |
        //  5  6  7
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 5);
        graph.add_edge(3, 6);
        graph.add_edge(4, 7);

        let result = graph.bfs(1);

        // Level 0: node 1
        assert_eq!(result[0], 1);

        // Level 1: nodes 2, 3, 4 (any order)
        let level_1: HashSet<i32> = result[1..4].iter().cloned().collect();
        assert_eq!(level_1, vec![2, 3, 4].into_iter().collect());

        // Level 2: nodes 5, 6, 7 (any order)
        let level_2: HashSet<i32> = result[4..7].iter().cloned().collect();
        assert_eq!(level_2, vec![5, 6, 7].into_iter().collect());
    }

    #[test]
    fn test_bfs_performance_large_graph() {
        let mut graph = Graph::new(false);

        // Create a binary tree of depth 10 (1023 nodes)
        for i in 1..512 {
            graph.add_edge(i, 2 * i);     // Left child
            graph.add_edge(i, 2 * i + 1); // Right child
        }

        let result = graph.bfs(1);
        assert_eq!(result.len(), 1023);
        assert_eq!(result[0], 1);

        // Check that all nodes are visited exactly once
        let mut visited_set = HashSet::new();
        for &node in &result {
            assert!(!visited_set.contains(&node), "Node {} visited twice", node);
            visited_set.insert(node);
        }
    }

    #[test]
    fn test_bfs_self_loops() {
        let mut graph = Graph::new(true);
        graph.add_edge(1, 1); // Self loop
        graph.add_edge(1, 2);
        graph.add_edge(2, 2); // Another self loop
        graph.add_edge(2, 3);

        let result = graph.bfs(1);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_shortest_path_multiple_shortest_paths() {
        let mut graph = Graph::new(false);
        // Diamond shape: multiple paths of same length
        //   1
        //  / \
        // 2   3
        //  \ /
        //   4
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        let path = graph.shortest_path_bfs(1, 4).unwrap();
        assert_eq!(path.len(), 3); // Length should be 3
        assert_eq!(path[0], 1);
        assert_eq!(path[2], 4);
        // Middle node should be either 2 or 3
        assert!(path[1] == 2 || path[1] == 3);
    }

    #[test]
    fn test_bfs_empty_graph_behavior() {
        let graph = Graph::new(false);

        // BFS on non-existent node should just return that node
        let result = graph.bfs(99);
        assert_eq!(result, vec![99]);

        // Shortest path in empty graph
        let path = graph.shortest_path_bfs(1, 2);
        assert!(path.is_none());
    }

    #[test]
    fn test_bfs_comparison_with_expected_distances() {
        let mut graph = Graph::new(false);
        // Create a more complex graph
        //   1 - 2 - 5
        //   |   |   |
        //   3 - 4 - 6
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 5);
        graph.add_edge(3, 4);
        graph.add_edge(4, 6);
        graph.add_edge(5, 6);

        let distances = graph.bfs_distances(1);

        // Verify expected distances
        assert_eq!(distances[&1], 0);
        assert_eq!(distances[&2], 1);
        assert_eq!(distances[&3], 1);
        assert_eq!(distances[&4], 2);
        assert_eq!(distances[&5], 2);
        assert_eq!(distances[&6], 3);

        // Verify shortest paths
        let path_to_6 = graph.shortest_path_bfs(1, 6).unwrap();
        assert_eq!(path_to_6.len(), 4); // Should be length 4 (distance 3 + 1)
    }
}