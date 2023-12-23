use std::collections::{HashMap, HashSet};

use advent_of_code::{Grid, OwnIndex};

advent_of_code::solution!(23);
#[derive(Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Eq, PartialEq)]
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
    let erg = recurse(&grid, start, end, &HashSet::new());
    Some(erg.unwrap())
}
fn recurse(grid: &Grid<Tile>, curr: usize, end: usize, visited: &HashSet<usize>) -> Option<u32> {
    if curr == end {
        return Some(visited.len() as u32);
    }
    let mut neigbors = get_neigbors(grid, curr);
    neigbors.retain(|v| !visited.contains(v));
    let mut visited = visited.clone();
    visited.insert(curr);
    neigbors
        .into_iter()
        .filter_map(|v| recurse(grid, v, end, &visited))
        .max()
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
fn build_graph_part_2(
    grid: &Grid<Tile>,
    start: usize,
    end: usize,
) -> HashMap<usize, HashSet<(usize, u32)>> {
    let intersections = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| t != &&Tile::Forrest)
        .filter(|(i, _)| get_neigbors(grid, *i).len() > 2)
        .map(|(i, _)| i)
        .chain(vec![start, end])
        //.map(|i| i.to_2d_index(grid))
        .collect::<Vec<_>>();
    let mut map = HashMap::new();
    for i in 0..intersections.len() - 1 {
        for j in i + 1..intersections.len() {
            let dist = dist_between(
                grid,
                intersections[i],
                intersections[j],
                &intersections,
                &HashSet::new(),
            );
            if let Some(dist) = dist {
                map.insert(
                    (
                        intersections[i], //.to_2d_index(grid),
                        intersections[j], //.to_2d_index(grid),
                    ),
                    dist,
                );
            }
        }
    }
    let mut new_map = HashMap::new();
    for ((n1, n2), val) in map {
        new_map
            .entry(n1)
            .and_modify(|s: &mut HashSet<(usize, u32)>| {
                s.insert((n2, val));
            })
            .or_insert(HashSet::from_iter(vec![(n2, val)]));
        new_map
            .entry(n2)
            .and_modify(|s: &mut HashSet<(usize, u32)>| {
                s.insert((n1, val));
            })
            .or_insert(HashSet::from_iter(vec![(n1, val)]));
    }
    new_map
}
fn dist_between(
    grid: &Grid<Tile>,
    curr: usize,
    end: usize,
    tabu_list: &Vec<usize>,
    visited: &HashSet<usize>,
) -> Option<u32> {
    if curr == end {
        return Some(visited.len() as u32);
    }
    let mut neigbors = get_neigbors(grid, curr);
    neigbors.retain(|v| !visited.contains(v));
    neigbors.retain(|v| !tabu_list.contains(v) || v == &end);
    let mut visited = visited.clone();
    visited.insert(curr);
    neigbors
        .into_iter()
        .filter_map(|v| dist_between(grid, v, end, tabu_list, &visited))
        .max()
}
fn recurse2(
    tree: &HashMap<usize, HashSet<(usize, u32)>>,
    curr: usize,
    end: usize,
    sum: u32,
    visited: &HashSet<usize>,
) -> Option<u32> {
    if curr == end {
        return Some(sum);
    }
    let mut neigbors = tree.get(&curr).unwrap().clone();
    neigbors.retain(|v| !visited.contains(&v.0));
    let mut visited = visited.clone();
    visited.insert(curr);
    neigbors
        .into_iter()
        .filter_map(|v| recurse2(tree, v.0, end, sum + v.1, &visited))
        .max()
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse(input);
    grid.iter_mut().for_each(|t| {
        *t = match t {
            Tile::Path => Tile::Path,
            Tile::Slope(_) => Tile::Path,
            Tile::Forrest => Tile::Forrest,
        }
    });
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
    let tree = build_graph_part_2(&grid, start, end);
    let erg = recurse2(&tree, start, end, 0, &HashSet::new());
    Some(erg.unwrap())
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
