use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use advent_of_code::Grid;

advent_of_code::solution!(17);
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    index: usize,
    dir: Dir,
    steps_already: usize,
    value: u32,
}

fn build_map(grid: &Grid<u32>, start: Vec<Node>) -> HashMap<Node, Vec<Node>> {
    let mut map = HashMap::new();
    let mut queue = VecDeque::from(start);
    while let Some(node) = queue.pop_front() {
        if map.contains_key(&node) {
            continue;
        }
        let neigbors = neigbors(grid, node);
        map.entry(node).or_insert(neigbors.clone());
        neigbors.into_iter().for_each(|n| queue.push_back(n));
    }
    map
}
fn neigbors(grid: &Grid<u32>, node: Node) -> Vec<Node> {
    let mut ret = vec![];
    assert!(node.steps_already <= 3);
    let dirs = match node.dir {
        Dir::North | Dir::South => vec![Dir::East, Dir::West],
        Dir::East | Dir::West => vec![Dir::North, Dir::South],
    };
    for dir in dirs {
        let n = match dir {
            Dir::North => grid.get_north(node.index),
            Dir::East => grid.get_east(node.index),
            Dir::South => grid.get_south(node.index),
            Dir::West => grid.get_west(node.index),
        };
        if let Some((i, _)) = n {
            ret.push(Node {
                index: i,
                dir,
                steps_already: 1,
                value: grid[i],
            })
        }
    }
    if node.steps_already < 3 {
        let n = match node.dir {
            Dir::North => grid.get_north(node.index),
            Dir::East => grid.get_east(node.index),
            Dir::South => grid.get_south(node.index),
            Dir::West => grid.get_west(node.index),
        };
        if let Some((i, _)) = n {
            ret.push(Node {
                index: i,
                dir: node.dir,
                steps_already: node.steps_already + 1,
                value: grid[i],
            })
        }
    }
    ret
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    solve(&grid, grid.width() * grid.height() - 1)
}
fn solve(grid: &Grid<u32>, target: usize) -> Option<u32> {
    let start = vec![
        Node {
            index: 0,
            dir: Dir::South,
            steps_already: 0,
            value: 0,
        },
        Node {
            index: 0,
            dir: Dir::East,
            steps_already: 0,
            value: 0,
        },
    ];
    let map = build_map(grid, start.clone());
    let (dist, prev) = dijkstra(&map, start);
    let min = dist
        .into_iter()
        .filter(|(k, _)| k.index == target)
        .min_by_key(|(_, j)| *j)
        .unwrap();
    let mut curr = min.0;
    while let Some(p) = prev.get(&curr) {
        curr = *p;
    }
    Some(min.1)
}
fn dijkstra(
    map: &HashMap<Node, Vec<Node>>,
    start: Vec<Node>,
) -> (HashMap<Node, u32>, HashMap<Node, Node>) {
    /*
     1  function Dijkstra(Graph, source):
     2
     3      for each vertex v in Graph.Vertices:
     4          dist[v] ← INFINITY
     5          prev[v] ← UNDEFINED
     6          add v to Q
     7      dist[source] ← 0
     8
     9      while Q is not empty:
    10          u ← vertex in Q with min dist[u]
    11          remove u from Q
    12
    13          for each neighbor v of u still in Q:
    14              alt ← dist[u] + Graph.Edges(u, v)
    15              if alt < dist[v]:
    16                  dist[v] ← alt
    17                  prev[v] ← u
    18
    19      return dist[], prev[]
    */
    let mut dist: HashMap<Node, u32> = HashMap::new();
    start.into_iter().for_each(|n| {
        dist.insert(n, 0);
    });
    let mut prev: HashMap<Node, Node> = HashMap::new();
    let mut queue = map.keys().cloned().collect::<Vec<_>>();
    while !queue.is_empty() {
        let index_with_min = queue
            .iter()
            .enumerate()
            .flat_map(|(i, node)| dist.get(node).map(|n| (*n, i)))
            .min()
            .unwrap_or_default()
            .1;
        let u = queue.swap_remove(index_with_min);
        for v in map.get(&u).unwrap() {
            if !queue.contains(v) {
                continue;
            }
            let alt = dist.get(&u).map(|n| (*n + v.value)).unwrap_or(u32::MAX);
            if alt < *dist.get(v).unwrap_or(&u32::MAX) {
                dist.insert(*v, alt);
                prev.insert(*v, u);
            }
        }
    }
    (dist, prev)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> Grid<u32> {
    Grid::from_iter_iter(
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap())),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::OwnIndex;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }
    #[test]
    #[ignore]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(907));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_part_one_1() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse(input);
        assert_eq!(solve(&grid, (2, 0).to_flat_index(&grid)), Some(6));
    }
    #[test]
    fn test_part_one_2() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse(input);
        assert_eq!(solve(&grid, (3, 0).to_flat_index(&grid)), Some(9));
    }
    #[test]
    fn test_part_one_3() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse(input);
        assert_eq!(solve(&grid, (4, 0).to_flat_index(&grid)), Some(17));
    }
}
