use std::collections::{HashMap, HashSet};

use crate::graph::Graph;
#[derive(Clone)]
struct CutOfThePhase {
    s: usize,
    t: usize,
    weight: i32,
}

pub struct MinCut {
    first: Graph,
    second: Graph,
    cutting_edges: Vec<(usize, usize, i32)>,
    cut_weight: i32,
}
impl MinCut {
    pub fn get_first_len(&self) -> usize {
        self.first.get_verticies().len()
    }
    pub fn get_second_len(&self) -> usize {
        self.second.get_verticies().len()
    }
}
pub fn compute_min_cut(graph: Graph) -> MinCut {
    debug_assert!(
        graph.vertex_count() >= 2,
        "Graph has to have at least 2 Verticies"
    );
    for e in graph.get_edges() {
        debug_assert!(e.2 >= 0, "All Edges has to have nonnegative weights");
    }
    let original_graph = graph.clone();
    let mut graph = graph;
    let mut current_partition: HashSet<usize> = HashSet::new();
    let mut current_best_partition: Option<HashSet<usize>> = None;
    let mut current_best_cut: Option<CutOfThePhase> = None;
    let mut got_merged_with = Vec::new();
    let mut best_got_merged_with = Vec::new();

    while graph.vertex_count() > 1 {
        //println!("Main Nodes: {}", graph.vertex_count());
        let begin = None /*
        Some(
            *graph
                .get_verticies()
                .iter()
                .map(|v| {
                    (
                        graph
                            .get_edges_from(v)
                            .into_iter()
                            .map(|e| e.2)
                            .sum::<i32>(),
                        v,
                    )
                })
                .min()
                .unwrap()
                .1,
        )*/;

        let cut_of_the_phase = maximum_adjacency_search(&graph, begin);
        if match current_best_cut {
            Some(ref x) => cut_of_the_phase.weight < x.weight,
            None => true,
        } {
            current_best_cut = Some(cut_of_the_phase.clone());
            current_best_partition = Some(current_partition.clone());

            best_got_merged_with = got_merged_with.clone();
            match current_best_partition {
                Some(ref mut x) => x.insert(cut_of_the_phase.t),
                None => unreachable!(),
            };
        }
        got_merged_with.push((cut_of_the_phase.t, cut_of_the_phase.s));
        current_partition.insert(cut_of_the_phase.t);
        graph = merge_verticies_from_cut(graph, cut_of_the_phase);
    }
    let mut queue = vec![best_got_merged_with[0].0];
    let mut set = HashSet::new();
    while let Some(next) = queue.pop() {
        set.insert(next);
        for (k, v) in &best_got_merged_with {
            if k == &next && !set.contains(v) {
                queue.push(*v);
            }
            if v == &next && !set.contains(k) {
                queue.push(*k);
            }
        }
    }
    construct_min_cut_result(original_graph, set)
}
fn maximum_adjacency_search(graph: &Graph, start: Option<usize>) -> CutOfThePhase {
    let start = match start {
        Some(x) => x,
        None => *graph.get_verticies().iter().next().unwrap(),
    };
    let mut max_adjacency_ordered_list = vec![start];
    let mut cut_weight = vec![];
    let mut candidates: HashSet<usize> = HashSet::from_iter(graph.get_verticies().iter().cloned());
    candidates.remove(&start);
    while !candidates.is_empty() {
        //println!("Canidates: {}", candidates.len());
        let mut max_next_vertex = None;
        let mut max_weight = i32::MIN;
        for next in &candidates {
            let weight_sum = max_adjacency_ordered_list
                .iter()
                .filter_map(|s| graph.get_edge(next, s))
                .map(|(_, _, w)| w)
                .sum::<i32>();
            if weight_sum > max_weight {
                max_next_vertex = Some(*next);
                max_weight = weight_sum;
            }
        }
        candidates.remove(&max_next_vertex.unwrap());
        max_adjacency_ordered_list.push(max_next_vertex.unwrap());
        cut_weight.push(max_weight);
    }
    let n = max_adjacency_ordered_list.len();
    CutOfThePhase {
        s: max_adjacency_ordered_list[n - 2],
        t: max_adjacency_ordered_list[n - 1],
        weight: cut_weight[cut_weight.len() - 1],
    }
}
fn merge_verticies_from_cut(graph: Graph, cut_of_the_phase: CutOfThePhase) -> Graph {
    let mut ret = Graph::default();
    for v in graph.get_verticies() {
        let is_s = v == &cut_of_the_phase.s;
        let is_t = v == &cut_of_the_phase.t;
        if !is_s && !is_t {
            ret.add_vertex(*v);
            for (_, dest, weight) in graph.get_edges_from(v) {
                if (dest != cut_of_the_phase.s) && (dest != cut_of_the_phase.t) {
                    ret.add_edge((*v, dest, weight));
                }
            }
        }
        if is_s {
            ret.add_vertex(*v);
            for (_, dest, weight) in graph.get_edges_from(v) {
                if dest == cut_of_the_phase.t {
                    continue;
                }
                if let Some(mergable_edge) = graph.get_edge(&cut_of_the_phase.t, &dest) {
                    ret.add_edge((*v, dest, weight + mergable_edge.2));
                    ret.add_edge((dest, *v, weight + mergable_edge.2));
                } else {
                    ret.add_edge((*v, dest, weight));
                    ret.add_edge((dest, *v, weight));
                }
            }
        }
    }
    for (_, dest, weight) in graph.get_edges_from(&cut_of_the_phase.t) {
        if dest == cut_of_the_phase.s {
            continue;
        }
        if graph.get_edge(&cut_of_the_phase.s, &dest).is_none() {
            ret.add_edge((cut_of_the_phase.s, dest, weight));
            ret.add_edge((dest, cut_of_the_phase.s, weight));
        }
    }
    ret
}
fn construct_min_cut_result(graph: Graph, partition: HashSet<usize>) -> MinCut {
    let mut first = Graph::default();
    let mut second = Graph::default();
    let mut cutting_edges = vec![];
    let mut cut_weight = 0;
    for id in graph.get_verticies() {
        if partition.contains(id) {
            first.add_vertex(*id);
        } else {
            second.add_vertex(*id);
        }
    }
    let mut edge_set = HashSet::new();
    for v in graph.get_verticies() {
        let edges = graph.get_edges_from(v);
        for (_, dest, weight) in edges {
            if first.get_verticies().contains(v) && first.get_verticies().contains(&dest) {
                first.add_edge((*v, dest, weight));
            } else if second.get_verticies().contains(v) && second.get_verticies().contains(&dest) {
                second.add_edge((*v, dest, weight));
            } else {
                cutting_edges.push((*v, dest, weight));
                if !edge_set.contains(&(*v, dest)) && !edge_set.contains(&(dest, *v)) {
                    edge_set.insert((*v, dest));
                    cut_weight += weight;
                }
            }
        }
    }
    MinCut {
        first,
        second,
        cutting_edges,
        cut_weight,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merging_from_cut_with_edge_transfer() {
        let g = min_cut_example_graph_for_edge_transfer();
        let cut_of_the_phase = CutOfThePhase {
            s: 3,
            t: 7,
            weight: 6,
        };
        let g_prime = merge_verticies_from_cut(g.clone(), cut_of_the_phase);
        assert_eq!(g.vertex_count() - 1, g_prime.vertex_count());
        assert_eq!(g.edge_count() - 2, g_prime.edge_count());
        assert!(!g_prime.get_verticies().contains(&7));
        assert_eq!(g_prime.get_edges_from(&7).len(), 0);

        assert_eq!(g_prime.get_edge(&2, &3).unwrap().2, 3);
        assert_eq!(g_prime.get_edge(&3, &2).unwrap().2, 3);
        assert_eq!(g_prime.get_edge(&6, &3).unwrap().2, 1);
        assert_eq!(g_prime.get_edge(&3, &6).unwrap().2, 1);
    }
    #[test]
    fn min_cut_happy_path_wikipedia_example_graph() {
        let g = min_cut_example_graph();
        let min_cut = compute_min_cut(g);
        assert_eq!(min_cut.first.vertex_count(), 4);
        assert_eq!(min_cut.first.edge_count(), 10);
    }
    #[test]
    fn test_max_adjacency_search_with_paper_example() {
        let g = min_cut_example_graph();
        let cut_of_the_phase = maximum_adjacency_search(&g, Some(2));
        assert_eq!(cut_of_the_phase.t, 1);
        assert_eq!(cut_of_the_phase.s, 5);
        assert_eq!(cut_of_the_phase.weight, 5);
    }
    #[test]
    fn test_merging_from_cut() {
        let g = min_cut_example_graph();
        let cut_of_the_phase = CutOfThePhase {
            s: 5,
            t: 1,
            weight: 5,
        };
        let g_prime = merge_verticies_from_cut(g.clone(), cut_of_the_phase);
        assert_eq!(g.vertex_count() - 1, g_prime.vertex_count());
        assert_eq!(g.edge_count() - 4, g_prime.edge_count());
        assert!(!g_prime.get_verticies().contains(&1));
        assert_eq!(g_prime.get_edges_from(&1).len(), 0);

        assert_eq!(g_prime.get_edge(&5, &2).unwrap().2, 4);
        assert_eq!(g_prime.get_edge(&2, &5).unwrap().2, 4);

        assert_eq!(g_prime.get_edge(&1, &2), None);
        assert_eq!(g_prime.get_edge(&1, &5), None);

        assert_eq!(g_prime.get_edge(&5, &6), g.get_edge(&5, &6));
    }
    fn min_cut_example_graph_for_edge_transfer() -> Graph {
        let mut graph = Graph::default();
        vec![1, 2, 3, 5, 6, 7].into_iter().for_each(|i| {
            graph.add_vertex(i);
        });
        graph.add_edge((1, 2, 2));
        graph.add_edge((1, 5, 3));
        //
        graph.add_edge((2, 1, 2));
        graph.add_edge((2, 3, 3));
        graph.add_edge((2, 5, 2));
        graph.add_edge((2, 6, 2));
        //
        graph.add_edge((3, 2, 3));
        graph.add_edge((3, 7, 2));
        //
        graph.add_edge((5, 1, 3));
        graph.add_edge((5, 6, 3));
        graph.add_edge((5, 2, 2));
        //
        graph.add_edge((6, 2, 2));
        graph.add_edge((6, 5, 3));
        graph.add_edge((6, 7, 1));
        //
        graph.add_edge((7, 6, 1));
        graph.add_edge((7, 3, 2));

        graph
    }
    fn min_cut_example_graph() -> Graph {
        let mut g = Graph::default();
        (1..=8).for_each(|i| {
            g.add_vertex(i);
        });
        g.add_edge((1, 2, 2));
        g.add_edge((1, 5, 3));
        //
        g.add_edge((2, 1, 2));
        g.add_edge((2, 3, 3));
        g.add_edge((2, 5, 2));
        g.add_edge((2, 6, 2));
        //
        g.add_edge((3, 2, 3));
        g.add_edge((3, 4, 4));
        g.add_edge((3, 7, 2));
        //
        g.add_edge((4, 3, 4));
        g.add_edge((4, 7, 2));
        g.add_edge((4, 8, 2));
        //
        g.add_edge((5, 1, 3));
        g.add_edge((5, 6, 3));
        g.add_edge((5, 2, 2));
        //
        g.add_edge((6, 2, 2));
        g.add_edge((6, 5, 3));
        g.add_edge((6, 7, 1));
        //
        g.add_edge((7, 6, 1));
        g.add_edge((7, 3, 2));
        g.add_edge((7, 4, 2));
        g.add_edge((7, 8, 3));
        //
        g.add_edge((8, 4, 2));
        g.add_edge((8, 7, 3));

        g
    }
}
