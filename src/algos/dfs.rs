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
}