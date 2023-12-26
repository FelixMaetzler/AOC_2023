use std::collections::{HashMap, HashSet};

use advent_of_code::{graph::Graph, stoer_wagner};

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);
    let undirected = undirected_graph(&map);
    let mut forward = HashMap::new();
    let mut backward = HashMap::new();
    let mut graph = Graph::default();
    for (ctr, k) in undirected.keys().enumerate() {
        forward.insert(k.clone(), ctr);
        backward.insert(ctr, k.clone());
        graph.add_vertex(ctr);
    }
    for (k, v) in undirected {
        for i in v {
            graph.add_edge((*forward.get(&k).unwrap(), *forward.get(&i).unwrap(), 1));
        }
    }
    let erg = stoer_wagner::compute_min_cut(graph);

    Some(erg.get_first_len() * erg.get_second_len())
}
fn count_connected(map: &HashMap<String, HashSet<String>>, start: String) -> usize {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push(start);
    while let Some(x) = queue.pop() {
        if visited.contains(&x) {
            continue;
        }
        queue.extend(map.get(&x).unwrap_or(&HashSet::new()).iter().cloned());
        visited.insert(x);
    }
    visited.len()
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn undirected_graph(map: &HashMap<String, HashSet<String>>) -> HashMap<String, HashSet<String>> {
    let mut new: HashMap<String, HashSet<String>> = HashMap::new();
    for (k, v) in map {
        for i in v {
            new.entry(k.clone())
                .and_modify(|e| {
                    e.insert(i.clone());
                })
                .or_insert(HashSet::from_iter(vec![i.clone()].into_iter()));
            new.entry(i.clone())
                .and_modify(|e| {
                    e.insert(k.clone());
                })
                .or_insert(HashSet::from_iter(vec![k.clone()].into_iter()));
        }
    }
    new
}
/*
fn to_graph(map: &HashMap<String, Vec<String>>) -> String {
    let mut s = "".to_string();
    for (k, v) in map {
        for i in v {
            s.push_str(&format!("{k} -> {i};\n"));
        }
    }
    s
}
*/
fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    input
        .trim()
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(l, r)| {
            (
                l.parse().unwrap(),
                r.split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
    #[test]
    #[ignore]
    fn test_part_acutal() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(514_786));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
