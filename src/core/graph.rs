use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Graph{
    pub adj_list: HashMap<i32, Vec<i32>>,
    pub is_directed: bool,
}

impl Graph {
    pub fn new(is_directed: bool) -> Self{
        Graph {
            adj_list: HashMap::new(),
            is_directed,
        }
    }

    pub fn add_edge(&mut self, u: i32, v: i32) {
        self.adj_list.entry(u).or_insert(Vec::new()).push(v);
        if !self.is_directed {
            self.adj_list.entry(v).or_insert(Vec::new()).push(u);
        }
    }
}
