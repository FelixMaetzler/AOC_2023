use std::fmt::Debug;

use advent_of_code::Grid;

advent_of_code::solution!(3);
#[derive(Clone)]
enum Tile {
    Empty,
    Digit(char),
    Symbol(char),
}
impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '0'..='9' => Ok(Self::Digit(value)),
            _ => Ok(Self::Symbol(value)),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Digit(arg0) => write!(f, "{}", arg0),
            Self::Symbol(arg0) => write!(f, "{}", arg0),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let mut s = String::new();
    let mut finished = false;
    let mut valid = false;
    let mut numbers = vec![];
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            match grid.get((y, x)).unwrap() {
                Tile::Empty if s.is_empty() => {}
                Tile::Empty => finished = true,
                Tile::Digit(d) => {
                    if !valid {
                        valid = grid
                            .neighbours8((y, x))
                            .iter()
                            .any(|t| matches!(t, Tile::Symbol(_)));
                    }
                    s.push(*d);
                }

                Tile::Symbol(_) => finished = true,
            }
            if finished {
                if valid {
                    numbers.push(s.parse().unwrap());
                }
                s.clear();
                valid = false;
                finished = false;
            }
        }
    }
    Some(numbers.iter().sum())
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
