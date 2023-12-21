use std::{collections::HashSet, str::FromStr};

use advent_of_code::{Grid, OwnIndex};

advent_of_code::solution!(21);
#[derive(Eq, PartialEq, Clone)]
enum Tile {
    Start,
    Rock,
    GardenPlot,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::GardenPlot),
            '#' => Ok(Self::Rock),
            'S' => Ok(Self::Start),
            x => Err(x),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    execute(input, 64)
}
fn execute(input: &str, n: usize) -> Option<u32> {
    let grid = parse(input);
    let mut set = HashSet::new();
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| t == &&Tile::Start)
        .unwrap()
        .0;
    set.insert(start);
    for _ in 0..n {
        let mut new_set = HashSet::new();
        for i in set {
            let n = get_neigbours(&grid, i);
            new_set.extend(n);
        }
        set = new_set;
    }
    Some(set.len() as u32)
}
fn get_neigbours(grid: &Grid<Tile>, index: usize) -> HashSet<usize> {
    let n = grid.neighbours4_with_index(index);
    n.into_iter()
        .filter(|(_, t)| t != &Tile::Rock)
        .map(|(i, _)| i.to_flat_index(grid))
        .collect()
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
        let input = &advent_of_code::template::read_file("examples", DAY);
        assert_eq!(execute(input, 1), Some(2));
        assert_eq!(execute(input, 2), Some(4));
        assert_eq!(execute(input, 3), Some(6));
        assert_eq!(execute(input, 6), Some(16));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(3_658));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
