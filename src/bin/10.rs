use std::collections::{HashMap, VecDeque};

use advent_of_code::{Grid, OwnIndex};

advent_of_code::solution!(10);
#[derive(PartialEq)]
enum Tile {
    NortSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}
enum Dir {
    North,
    South,
    West,
    East,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NortSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            val => Err(val),
        }
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, e)| e == &&Tile::Start)
        .unwrap()
        .0;
    let map = grid
        .iter()
        .enumerate()
        .filter(|(_, t)| t != &&Tile::Ground)
        .map(|(i, _)| (i, get_neighbours(i, &grid)))
        .collect::<HashMap<_, _>>();
    let mut dist = HashMap::new();
    dist.insert(start, 0);
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        let d = *dist.get(&node).unwrap();
        for n in map.get(&node).unwrap() {
            let n = n.to_flat_index(&grid);
            dist.entry(n).or_insert_with(|| {
                queue.push_back(n);
                d + 1
            });
        }
    }
    Some(*dist.values().max().unwrap())
}
fn get_neighbours(index: impl OwnIndex<Tile>, grid: &Grid<Tile>) -> Vec<impl OwnIndex<Tile>> {
    use Dir::*;
    let vec = match grid.get(index).unwrap() {
        Tile::NortSouth => vec![North, South],
        Tile::EastWest => vec![East, West],
        Tile::NorthEast => vec![North, East],
        Tile::NorthWest => vec![North, West],
        Tile::SouthWest => vec![South, West],
        Tile::SouthEast => vec![South, East],
        Tile::Start => vec![North, South, West, East],
        Tile::Ground => unreachable!(),
    };
    vec.into_iter()
        .flat_map(|dir| get_neighbour(index, dir, grid))
        .collect()
}
fn get_neighbour(
    index: impl OwnIndex<Tile>,
    dir: Dir,
    grid: &Grid<Tile>,
) -> Option<impl OwnIndex<Tile>> {
    use Tile::*;
    match dir {
        Dir::North => {
            if let Some((idx, tile)) = grid.get_north(index) {
                if matches!(tile, SouthEast | SouthWest | NortSouth) {
                    Some(idx.to_flat_index(grid))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Dir::South => {
            if let Some((idx, tile)) = grid.get_south(index) {
                if matches!(tile, NortSouth | NorthEast | NorthWest) {
                    Some(idx.to_flat_index(grid))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Dir::West => {
            if let Some((idx, tile)) = grid.get_west(index) {
                if matches!(tile, EastWest | NorthEast | SouthEast) {
                    Some(idx.to_flat_index(grid))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Dir::East => {
            if let Some((idx, tile)) = grid.get_east(index) {
                if matches!(tile, EastWest | NorthWest | SouthWest) {
                    Some(idx.to_flat_index(grid))
                } else {
                    None
                }
            } else {
                None
            }
        }
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
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6_831));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
