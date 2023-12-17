use advent_of_code::{Grid, OwnIndex};
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(16);
enum Tile {
    Space,
    Vertical,
    Horizontal,
    SouthWest,
    SouthEast,
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Space),
            '-' => Ok(Self::Horizontal),
            '|' => Ok(Self::Vertical),
            '/' => Ok(Self::SouthEast),
            '\\' => Ok(Self::SouthWest),
            val => Err(val),
        }
    }
}
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}
pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input);
    let start = (0, Dir::East);
    solve(&grid, start)
}
fn solve(grid: &Grid<Tile>, start: (usize, Dir)) -> Option<usize> {
    let mut queue = VecDeque::from(vec![start]);
    let mut visited = HashSet::new();
    while let Some(curr) = queue.pop_front() {
        if visited.contains(&curr) {
            continue;
        } else {
            visited.insert(curr);
        }
        let tile = grid.get(curr.0).unwrap();
        let next = match (tile, curr.1) {
            (Tile::Space, dir) => vec![dir],
            (Tile::Vertical, Dir::East | Dir::West) => vec![Dir::North, Dir::South],
            (Tile::Vertical, other) => vec![other],
            (Tile::Horizontal, Dir::North | Dir::South) => vec![Dir::East, Dir::West],
            (Tile::Horizontal, other) => vec![other],
            (Tile::SouthWest, Dir::North) => vec![Dir::West],
            (Tile::SouthWest, Dir::South) => vec![Dir::East],
            (Tile::SouthWest, Dir::East) => vec![Dir::South],
            (Tile::SouthWest, Dir::West) => vec![Dir::North],
            (Tile::SouthEast, Dir::North) => vec![Dir::East],
            (Tile::SouthEast, Dir::South) => vec![Dir::West],
            (Tile::SouthEast, Dir::East) => vec![Dir::North],
            (Tile::SouthEast, Dir::West) => vec![Dir::South],
        };
        for dir in next {
            let opt = match dir {
                Dir::North => grid.get_north(curr.0),
                Dir::South => grid.get_south(curr.0),
                Dir::East => grid.get_east(curr.0),
                Dir::West => grid.get_west(curr.0),
            };
            if let Some((x, _)) = opt {
                queue.push_back((x, dir))
            }
        }
    }
    let visited = visited.into_iter().map(|(x, _)| x).collect::<HashSet<_>>();
    Some(visited.len())
}
pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse(input);
    (0..grid.height())
        .flat_map(|y| {
            vec![
                ((y, 0).to_flat_index(&grid), Dir::East),
                ((y, grid.width() - 1).to_flat_index(&grid), Dir::West),
            ]
        })
        .chain((0..grid.width()).flat_map(|x| {
            vec![
                ((0, x).to_flat_index(&grid), Dir::South),
                ((grid.height() - 1, x).to_flat_index(&grid), Dir::North),
            ]
        }))
        // .par_bridge() // for rayon
        .map(|s| solve(&grid, s))
        .max()
        .unwrap()
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
        assert_eq!(result, Some(46));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7_067));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
    #[test]
    #[ignore]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7_324));
    }
}
