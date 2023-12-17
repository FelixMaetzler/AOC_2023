use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    fmt::Debug,
};

use advent_of_code::Grid;

advent_of_code::solution!(17);
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
enum Dir {
    North,
    East,
    South,
    West,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
struct Node {
    index: usize,
    dir: Dir,
    steps_already: usize,
    value: u32,
}

fn build_map(
    grid: &Grid<u32>,
    start: Vec<Node>,
    func: impl Fn(&Grid<u32>, Node) -> Vec<Node>,
) -> HashMap<Node, Vec<Node>> {
    let mut map = HashMap::new();
    let mut queue = VecDeque::from(start);
    while let Some(node) = queue.pop_front() {
        if map.contains_key(&node) {
            continue;
        }
        let neigbors = func(grid, node);
        map.entry(node).or_insert(neigbors.clone());
        neigbors.into_iter().for_each(|n| queue.push_back(n));
    }
    map
}
fn neigbors_part_1(grid: &Grid<u32>, node: Node) -> Vec<Node> {
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
fn neigbors_part_2(grid: &Grid<u32>, node: Node) -> Vec<Node> {
    let mut ret = vec![];
    assert!(node.steps_already <= 10);
    if node.steps_already < 4 {
        let n = match node.dir {
            Dir::North => grid.get_north(node.index),
            Dir::East => grid.get_east(node.index),
            Dir::South => grid.get_south(node.index),
            Dir::West => grid.get_west(node.index),
        };
        return if let Some((i, _)) = n {
            vec![Node {
                index: i,
                dir: node.dir,
                steps_already: node.steps_already + 1,
                value: grid[i],
            }]
        } else {
            vec![]
        };
    }
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
    if node.steps_already < 10 {
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
    solve_part_1(&grid, grid.width() * grid.height() - 1)
}
fn solve_part_1(grid: &Grid<u32>, target: usize) -> Option<u32> {
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
    let map = build_map(grid, start.clone(), neigbors_part_1);
    let (dist, _) = dijkstra(&map, start);
    let min = dist
        .into_iter()
        .filter(|(k, _)| k.index == target)
        .min_by_key(|(_, j)| *j)
        .unwrap();

    Some(min.1)
}
fn solve_part_2(grid: &Grid<u32>, target: usize) -> Option<u32> {
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
    let map = build_map(grid, start.clone(), neigbors_part_2);
    let (dist, _) = dijkstra(&map, start);
    let min = dist
        .into_iter()
        .filter(|(k, _)| k.index == target && k.steps_already >= 4)
        .min_by_key(|(_, j)| *j)
        .unwrap();
    Some(min.1)
}
fn dijkstra(
    map: &HashMap<Node, Vec<Node>>,
    start: Vec<Node>,
) -> (HashMap<Node, u32>, HashMap<Node, Node>) {
    let mut dist: HashMap<Node, u32> = HashMap::new();
    let mut queue = BinaryHeap::new();
    start.into_iter().for_each(|n| {
        dist.insert(n, 0);
        queue.push((Reverse(0), n));
    });
    let mut prev: HashMap<Node, Node> = HashMap::new();

    while !queue.is_empty() {
        let (_, u) = queue.pop().unwrap();
        let next = map.get(&u).unwrap();
        for v in next {
            let alt = dist.get(&u).map(|n| (*n + v.value)).unwrap_or(u32::MAX);
            if alt < *dist.get(v).unwrap_or(&u32::MAX) {
                dist.insert(*v, alt);
                queue.push((Reverse(alt), *v));
                prev.insert(*v, u);
            }
        }
    }
    (dist, prev)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    solve_part_2(&grid, grid.width() * grid.height() - 1)
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
        let result = part_one(
            advent_of_code::template::read_file("examples", DAY)
                .split_once("\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(102));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(907));
    }

    #[test]
    fn test_part_two() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let input = binding.split_once("\n\n").unwrap();
        assert_eq!(part_two(input.0), Some(94));
        assert_eq!(part_two(input.1), Some(71));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1_057));
    }
    #[test]
    fn test_part_one_1() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let input = binding.split_once("\n\n").unwrap().0;
        let grid = parse(input);
        assert_eq!(solve_part_1(&grid, (2, 0).to_flat_index(&grid)), Some(6));
    }
    #[test]
    fn test_part_one_2() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let input = binding.split_once("\n\n").unwrap().0;

        let grid = parse(input);
        assert_eq!(solve_part_1(&grid, (3, 0).to_flat_index(&grid)), Some(9));
    }
    #[test]
    fn test_part_one_3() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let input = binding.split_once("\n\n").unwrap().0;

        let grid = parse(input);
        assert_eq!(solve_part_1(&grid, (4, 0).to_flat_index(&grid)), Some(17));
    }
}
