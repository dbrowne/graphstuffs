use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WeightedGraph {
    pub ajd_list: HashMap<i32, Vec<(i32,i32)>>,
    pub is_directed: bool,
}

impl WeightedGraph {
    pub fn new(is_directed: bool) -> Self{
        WeightedGraph{
            ajd_list: HashMap::new(),
            is_directed,
        }

    }

    pub fn add_edge(&mut self, u: i32, v: i32, weight: i32) {
        self.ajd_list.entry(u).or_insert(Vec::new()).push((v,weight));
        if !self.is_directed{
            self.ajd_list.entry(v).or_insert(Vec::new()).push((u,weight));

        }
    }

}
