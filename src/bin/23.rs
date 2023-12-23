use std::collections::{HashMap, HashSet};

use advent_of_code::{Bitmask, Grid, OwnIndex};

advent_of_code::solution!(23);
#[derive(Eq, PartialEq, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Eq, PartialEq, Clone)]
enum Tile {
    Path,
    Forrest,
    Slope(Dir),
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forrest),
            '>' => Ok(Self::Slope(Dir::Right)),
            '<' => Ok(Self::Slope(Dir::Left)),
            '^' => Ok(Self::Slope(Dir::Up)),
            'v' => Ok(Self::Slope(Dir::Down)),
            val => Err(val),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    execute(input)
}
fn get_all_neigbors(grid: &Grid<Tile>, index: usize) -> Vec<usize> {
    let n = grid.neighbours4_with_index(index);
    n.into_iter()
        .filter(|(_, t)| t != &Tile::Forrest)
        .map(|(i, _)| i.to_flat_index(grid))
        .collect()
}
fn get_neigbors(grid: &Grid<Tile>, index: usize) -> Vec<usize> {
    let curr = grid.get(index).unwrap();
    match curr {
        Tile::Path => {
            let mut ret = vec![];
            if let Some((next, tile)) = grid.get_north(index) {
                if matches!(tile, Tile::Path | Tile::Slope(Dir::Up)) {
                    ret.push(next)
                }
            }
            if let Some((next, tile)) = grid.get_south(index) {
                if matches!(tile, Tile::Path | Tile::Slope(Dir::Down)) {
                    ret.push(next)
                }
            }
            if let Some((next, tile)) = grid.get_east(index) {
                if matches!(tile, Tile::Path | Tile::Slope(Dir::Right)) {
                    ret.push(next)
                }
            }
            if let Some((next, tile)) = grid.get_west(index) {
                if matches!(tile, Tile::Path | Tile::Slope(Dir::Left)) {
                    ret.push(next)
                }
            }
            ret
        }
        Tile::Slope(dir) => {
            if let Some((next, tile)) = match dir {
                Dir::Up => grid.get_north(index),
                Dir::Down => grid.get_south(index),
                Dir::Left => grid.get_west(index),
                Dir::Right => grid.get_east(index),
            } {
                match tile {
                    Tile::Path => vec![next],
                    Tile::Forrest => vec![],
                    Tile::Slope(_) => unreachable!(),
                }
            } else {
                vec![]
            }
        }
        Tile::Forrest => unreachable!(),
    }
}
fn find_next_intersection(
    grid: &Grid<Tile>,
    start_intersection: usize,
    start_dir: usize,
    intersections: &[usize],
) -> Option<(usize, u32)> {
    let mut visited = HashSet::new();
    visited.insert(start_intersection);
    let mut ctr = 0;
    let mut curr = start_dir;
    loop {
        if intersections.contains(&curr) {
            return Some((curr, ctr + 1));
        }
        let mut n = get_neigbors(grid, curr);
        n.retain(|e| !visited.contains(e));
        match n.len() {
            0 => return None,
            1 => {
                ctr += 1;
                visited.insert(curr);
                curr = n[0]
            }
            _ => {
                unreachable!(
                    "If there are more than 2, it is a intersection and should be handled earlier"
                )
            }
        }
    }
}
fn build_graph(
    grid: &Grid<Tile>,
    start: usize,
    end: usize,
) -> HashMap<usize, HashSet<(usize, u32)>> {
    let mut map = HashMap::new();
    let intersections = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| t != &&Tile::Forrest)
        .filter(|(i, _)| get_all_neigbors(grid, *i).len() > 2)
        .map(|(i, _)| i)
        .chain(vec![start, end])
        .collect::<Vec<_>>();
    for inter in &intersections {
        let neig = get_neigbors(grid, *inter);
        for n in neig {
            if let Some(erg) = find_next_intersection(grid, *inter, n, &intersections) {
                map.entry(*inter)
                    .and_modify(|s: &mut HashSet<(usize, u32)>| {
                        s.insert(erg);
                    })
                    .or_insert(HashSet::from_iter(vec![erg]));
            }
        }
        map.entry(*inter).or_default();
    }
    map
}
fn recurse(
    tree: &Vec<Vec<(usize, u32)>>,
    curr: usize,
    end: usize,
    sum: u32,
    visited: Bitmask<usize>,
) -> Option<u32> {
    if curr == end {
        return Some(sum);
    }
    let mut neigbors = tree[curr].clone();
    neigbors.retain(|v| !visited.get(v.0));
    let mut visited = visited;
    visited.set(curr);
    neigbors
        .into_iter()
        .filter_map(|v| recurse(tree, v.0, end, sum + v.1, visited))
        .max()
}
fn execute(input: &str) -> Option<u32> {
    let grid = parse(input);
    let start = (0..grid.width())
        .map(|x| (x, grid.get((0, x)).unwrap()))
        .find(|(_, t)| t == &&Tile::Path)
        .unwrap()
        .0;
    let end = (0..grid.width())
        .map(|x| {
            (
                (grid.height() - 1, x),
                grid.get((grid.height() - 1, x)).unwrap(),
            )
        })
        .find(|(_, t)| t == &&Tile::Path)
        .unwrap()
        .0
        .to_flat_index(&grid);
    let tree = build_graph(&grid, start, end);
    let mut map = HashMap::new();
    let mut map_rev = HashMap::new();
    let mut ctr = 0;
    for v in tree.keys() {
        if map.contains_key(v) {
            continue;
        } else {
            map.insert(v, ctr);
            map_rev.insert(ctr, v);
            ctr += 1;
        }
    }
    let mut vec = Vec::with_capacity(map.len());
    for i in 0..tree.len() {
        let s = tree.get(map_rev.get(&i).unwrap()).unwrap();
        let s = s
            .iter()
            .map(|(d, val)| (*map.get(d).unwrap(), *val))
            .collect::<HashSet<_>>();
        vec.push(s);
    }
    let erg = recurse(
        &vec.into_iter()
            .map(|s| s.into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        *map.get(&start).unwrap(),
        *map.get(&end).unwrap(),
        0,
        Bitmask::default(),
    );
    Some(erg.unwrap())
}
pub fn part_two(input: &str) -> Option<u32> {
    let input = input.replace(|ch| matches!(ch, '<' | '>' | '^' | 'v'), ".");
    execute(&input)
}
fn parse(input: &str) -> Grid<Tile> {
    Grid::from_iter_iter(
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| Tile::try_from(c).unwrap())),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2_414));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6_598));
    }
}
