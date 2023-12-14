use std::{collections::HashMap, fmt::Debug};

use advent_of_code::{Grid, OwnIndex};
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Space,
    CubeRock,
    RoundRock,
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space => write!(f, "."),
            Self::CubeRock => write!(f, "#"),
            Self::RoundRock => write!(f, "O"),
        }
    }
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            '#' => Ok(Self::CubeRock),
            'O' => Ok(Self::RoundRock),
            val => Err(val),
        }
    }
}
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    tilt_north(&mut grid);
    Some(calculate_load(&grid))
}
fn tilt_north(grid: &mut Grid<Tile>) {
    for i in 0..grid.width() {
        let mut col = grid.get_col(i);
        for j in 0..col.len() {
            if col[j] == Tile::RoundRock {
                let mut k = j;
                while k > 0 {
                    k -= 1;
                    if col[k] == Tile::Space {
                        col.swap(k, k + 1);
                    } else {
                        break;
                    }
                }
            }
        }
        grid.set_col(i, &col);
    }
}
fn tilt_west(grid: &mut Grid<Tile>) {
    for i in 0..grid.height() {
        let mut row = grid.get_row(i);
        for j in 0..row.len() {
            if row[j] == Tile::RoundRock {
                let mut k = j;
                while k > 0 {
                    k -= 1;
                    if row[k] == Tile::Space {
                        row.swap(k, k + 1);
                    } else {
                        break;
                    }
                }
            }
        }
        grid.set_row(i, &row);
    }
}
fn tilt_south(grid: &mut Grid<Tile>) {
    for i in 0..grid.width() {
        let mut col = grid.get_col(i);
        for j in (0..col.len()).rev() {
            if col[j] == Tile::RoundRock {
                let mut k = j;
                while k < grid.height() - 1 {
                    k += 1;
                    if col[k] == Tile::Space {
                        col.swap(k - 1, k);
                    } else {
                        break;
                    }
                }
            }
        }
        grid.set_col(i, &col);
    }
}
fn tilt_east(grid: &mut Grid<Tile>) {
    for i in 0..grid.width() {
        let mut row = grid.get_row(i);
        for j in (0..row.len()).rev() {
            if row[j] == Tile::RoundRock {
                let mut k = j;
                while k < grid.height() - 1 {
                    k += 1;
                    if row[k] == Tile::Space {
                        row.swap(k - 1, k);
                    } else {
                        break;
                    }
                }
            }
        }
        grid.set_row(i, &row);
    }
}
fn calculate_load(grid: &Grid<Tile>) -> usize {
    grid.iter()
        .enumerate()
        .filter(|(_, t)| t == &&Tile::RoundRock)
        .map(|(i, _)| grid.height() - i.to_2d_index(grid).0)
        .sum()
}
fn execute_one_cyle(grid: &mut Grid<Tile>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    const CYCLES: usize = 1_000_000_000;
    let mut map = HashMap::new();
    let mut start_cycle = None;
    let mut end_cycle = None;
    for i in 0..CYCLES {
        execute_one_cyle(&mut grid);
        if let Some(prev) = map.get(&grid) {
            start_cycle = Some(*prev);
            end_cycle = Some(i);
            break;
        } else {
            map.insert(grid.clone(), i);
        }
    }
    let start_cycle = start_cycle.unwrap();
    let end_cycle = end_cycle.unwrap();
    let diff = end_cycle - start_cycle;
    let remaining = (CYCLES - 1 - start_cycle) % diff;
    for _ in 0..remaining {
        execute_one_cyle(&mut grid);
    }

    Some(calculate_load(&grid))
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
        assert_eq!(result, Some(136));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(109_661));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(90_176));
    }
}
