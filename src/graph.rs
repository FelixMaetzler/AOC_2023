use std::collections::HashSet;
type Edge = (usize, usize, i32);
#[derive(Clone, Default)]
pub struct Graph {
    verticies: HashSet<usize>,
    edges: HashSet<Edge>,
}
impl Graph {
    pub fn vertex_count(&self) -> usize {
        self.verticies.len()
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    pub fn get_edges(&self) -> &HashSet<Edge> {
        &self.edges
    }
    pub fn get_verticies(&self) -> &HashSet<usize> {
        &self.verticies
    }
    pub fn add_vertex(&mut self, v: usize) -> bool {
        self.verticies.insert(v)
    }
    pub fn add_edge(&mut self, e: Edge) -> bool {
        self.edges.insert(e)
    }
    pub fn get_edge(&self, src: &usize, dest: &usize) -> Option<Edge> {
        let ret = self
            .edges
            .iter()
            .find(|(s, d, _)| *src == *s && *dest == *d)
            .copied();
        ret
    }
    pub fn get_edges_from(&self, src: &usize) -> Vec<Edge> {
        self.edges
            .iter()
            .filter(|(s, ..)| src == s)
            .copied()
            .collect()
    }
}
