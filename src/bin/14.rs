use std::fmt::Debug;

use advent_of_code::{Grid, OwnIndex};
#[derive(Clone, Copy, PartialEq)]
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
    let mut sum = 0;
    for (i, t) in grid.iter().enumerate() {
        let (y, _) = i.to_2d_index(&grid);
        if t == &Tile::RoundRock {
            sum += grid.height() - y;
        }
    }
    Some(sum)
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
        assert_eq!(result, None);
    }
}
