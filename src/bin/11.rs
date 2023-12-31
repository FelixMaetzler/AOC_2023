use std::{collections::HashMap, fmt::Debug};

use advent_of_code::{Grid, OwnIndex};

advent_of_code::solution!(11);
#[derive(Clone, PartialEq, Copy)]
enum Tile {
    Space,
    Galaxy,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            '#' => Ok(Self::Galaxy),
            ch => Err(ch),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space => write!(f, "."),
            Self::Galaxy => write!(f, "#"),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}
fn distance(n: ((usize, usize), (usize, usize))) -> usize {
    let (x, y) = n;
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}
fn build_combinations(vec: &[(usize, usize)]) -> Vec<((usize, usize), (usize, usize))> {
    (0..vec.len() - 1)
        .flat_map(|i| (i + 1..vec.len()).map(move |j| (vec[i], vec[j])))
        .collect()
}
pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1_000_000)
}
fn solve(input: &str, times: usize) -> Option<usize> {
    let grid = parse(input);

    let empty_rows = (0..grid.height())
        .map(|y| (y, grid.get_row(y)))
        .filter(|(_, v)| v.iter().all(|t| t == &Tile::Space))
        .map(|(y, _)| y)
        .collect::<Vec<_>>();
    let empty_cols = (0..grid.width())
        .map(|x| (x, grid.get_col(x)))
        .filter(|(_, v)| v.iter().all(|t| t == &Tile::Space))
        .map(|(x, _)| x)
        .collect::<Vec<_>>();
    let galaxies = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| t == &&Tile::Galaxy)
        .map(|(n, _)| n.to_2d_index(&grid))
        .collect::<Vec<_>>();
    let mut map_rows: HashMap<usize, usize> =
        HashMap::from_iter((0..grid.height()).map(|n| (n, n)));
    let mut map_cols: HashMap<usize, usize> = HashMap::from_iter((0..grid.width()).map(|n| (n, n)));
    empty_rows.iter().for_each(|n| {
        map_rows
            .iter_mut()
            .filter(|(k, _)| k > &n)
            .for_each(|(_, v)| *v += times - 1)
    });
    empty_cols.iter().for_each(|n| {
        map_cols
            .iter_mut()
            .filter(|(k, _)| k > &n)
            .for_each(|(_, v)| *v += times - 1)
    });
    let new_galaxies = galaxies
        .into_iter()
        .map(|(y, x)| (*map_rows.get(&y).unwrap(), *map_cols.get(&x).unwrap()))
        .collect::<Vec<_>>();
    let combinations = build_combinations(&new_galaxies);
    Some(combinations.into_iter().map(distance).sum())
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
        assert_eq!(result, Some(374));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(9_769_724));
    }

    #[test]
    fn test_part_two() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(1_030));
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(8_410));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(603_020_563_700));
    }
}
