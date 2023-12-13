use std::{collections::HashSet, fmt::Debug};

use advent_of_code::Grid;

advent_of_code::solution!(13);
#[derive(Clone, PartialEq)]
enum Tile {
    Rock,
    Ash,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            val => Err(val),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Ash => write!(f, "."),
        }
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let grids = parse(input);
    let mut sum_horizontal = 0;
    let mut sum_vertical = 0;
    for grid in grids {
        let erg = check_horizontal_symmetry(&grid).unwrap_or(0);
        sum_horizontal += erg;
        let erg = check_vertical_symmetry(&grid).unwrap_or(0);
        sum_vertical += erg;
    }
    Some(100 * sum_horizontal + sum_vertical)
}
fn check_horizontal_symmetry(grid: &Grid<Tile>) -> Option<usize> {
    let mut set: HashSet<usize> = HashSet::from_iter(1..grid.height());
    for i in 0..grid.width() {
        let row = grid.get_col(i);
        let erg = check_symmetry(&row, &set);
        let erg: HashSet<_> = erg.intersection(&set).cloned().collect();
        set = erg;
        if set.is_empty() {
            return None;
        }
    }
    assert_eq!(set.len(), 1);
    Some(set.into_iter().next().unwrap())
}
fn check_vertical_symmetry(grid: &Grid<Tile>) -> Option<usize> {
    let mut set: HashSet<usize> = HashSet::from_iter(1..grid.width());
    for i in 0..grid.height() {
        let row = grid.get_row(i);
        let erg = check_symmetry(&row, &set);
        let erg: HashSet<_> = erg.intersection(&set).cloned().collect();
        set = erg;
        if set.is_empty() {
            return None;
        }
    }
    assert_eq!(set.len(), 1);
    Some(set.into_iter().next().unwrap())
}
fn check_symmetry(line: &[Tile], options: &HashSet<usize>) -> HashSet<usize> {
    options
        .iter()
        .filter(|i| is_symmetic(line, **i))
        .cloned()
        .collect()
}
fn is_symmetic(line: &[Tile], i: usize) -> bool {
    let left = line.iter().take(i).rev();
    let right = line.iter().skip(i);
    let mut zip = left.zip(right);
    zip.all(|(a, b)| a == b)
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> Vec<Grid<Tile>> {
    input
        .trim()
        .split("\n\n")
        .map(|g| {
            Grid::from_iter_iter(
                g.lines()
                    .map(|l| l.chars().map(|c| Tile::try_from(c).unwrap())),
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
        assert_eq!(result, Some(405));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(33_122));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
    #[test]
    fn test_part_one_1() {
        let result = part_one(
            "
.###....#.###..
...#.##...#.#..
.#.#.#.......##
#######..#..#..
#####.###...#..
##.##......#.##
###.#.##.#...##
#####.#.###....
###..##.####...
....#...#....##
...#.######.#..
.#..#...#.###..
.###.####..#...
..#####........
####...#.#..###
####...#.#..###
..###.#........",
        );
        assert_eq!(result, Some(14));
    }
}
