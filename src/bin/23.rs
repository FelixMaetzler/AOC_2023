use std::collections::HashSet;

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
pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
