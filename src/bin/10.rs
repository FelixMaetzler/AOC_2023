use std::{collections::HashSet, fmt::Debug};

use advent_of_code::{Grid, OwnIndex};

advent_of_code::solution!(10);
#[derive(PartialEq, Clone, Copy)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NorthSouth => write!(f, "|"),
            Self::EastWest => write!(f, "-"),
            Self::NorthEast => write!(f, "⌞"),
            Self::NorthWest => write!(f, "⌟"),
            Self::SouthWest => write!(f, "⌝"),
            Self::SouthEast => write!(f, "⌜"),
            Self::Ground => write!(f, "."),
            Self::Start => write!(f, "S"),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}
impl Dir {
    fn step(&self, pos: &mut (usize, usize)) {
        match self {
            Dir::North => pos.0 -= 1,
            Dir::South => pos.0 += 1,
            Dir::West => pos.1 -= 1,
            Dir::East => pos.1 += 1,
        }
    }

    fn turn(&mut self, t: &Tile) {
        use Dir::*;
        use Tile::*;
        *self = match (&self, t) {
            (_, Ground | Start) => unreachable!(),
            (North, NorthSouth) => North,
            (North, SouthEast) => East,
            (North, SouthWest) => West,

            (South, NorthSouth) => South,
            (South, NorthWest) => West,
            (South, NorthEast) => East,

            (West, EastWest) => West,
            (West, NorthEast) => North,
            (West, SouthEast) => South,

            (East, EastWest) => East,
            (East, NorthWest) => North,
            (East, SouthWest) => South,
            _ => unreachable!(),
        };
    }
}
impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NorthSouth),
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
    let (start, mut dir) = start(&grid);
    let mut curr = start;
    let mut ctr = 0;
    loop {
        dir.step(&mut curr);
        ctr += 1;
        if curr == start {
            return Some(ctr / 2);
        }
        let t = grid[curr];
        dir.turn(&t);
    }
}
fn start(grid: &Grid<Tile>) -> ((usize, usize), Dir) {
    use Tile::*;
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| t == &&Start)
        .unwrap()
        .0;
    let dir = if grid
        .get_north(start)
        .is_some_and(|(_, t)| matches!(t, SouthEast | SouthWest | NorthSouth))
    {
        Dir::North
    } else if grid
        .get_south(start)
        .is_some_and(|(_, t)| matches!(t, NorthEast | NorthWest | NorthSouth))
    {
        Dir::South
    } else if grid
        .get_west(start)
        .is_some_and(|(_, t)| matches!(t, NorthEast | EastWest | SouthEast))
    {
        Dir::West
    } else if grid
        .get_east(start)
        .is_some_and(|(_, t)| matches!(t, EastWest | NorthEast | SouthEast))
    {
        Dir::East
    } else {
        unreachable!()
    };
    (start.to_2d_index(grid), dir)
}
fn get_start_tile(grid: &Grid<Tile>) -> Tile {
    use Tile::*;
    let mut vec = vec![];
    let start = grid
        .iter()
        .enumerate()
        .find(|(_, t)| t == &&Start)
        .unwrap()
        .0;
    if grid
        .get_north(start)
        .is_some_and(|(_, t)| matches!(t, SouthEast | SouthWest | NorthSouth))
    {
        vec.push(Dir::North);
    };
    if grid
        .get_south(start)
        .is_some_and(|(_, t)| matches!(t, NorthEast | NorthWest | NorthSouth))
    {
        vec.push(Dir::South);
    };
    if grid
        .get_west(start)
        .is_some_and(|(_, t)| matches!(t, NorthEast | EastWest | SouthEast))
    {
        vec.push(Dir::West);
    };
    if grid
        .get_east(start)
        .is_some_and(|(_, t)| matches!(t, EastWest | NorthWest | SouthWest))
    {
        vec.push(Dir::East);
    };
    assert_eq!(vec.len(), 2);
    match (vec[0], vec[1]) {
        (Dir::North, Dir::South) => Tile::NorthSouth,
        (Dir::North, Dir::West) => Tile::NorthWest,
        (Dir::North, Dir::East) => Tile::NorthEast,

        (Dir::South, Dir::West) => Tile::SouthWest,
        (Dir::South, Dir::East) => Tile::SouthEast,

        (Dir::West, Dir::East) => Tile::EastWest,

        _ => unreachable!(),
    }
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let (start, mut dir) = start(&grid);
    grid[start] = get_start_tile(&grid);
    let grid = grid;
    let mut curr = start;
    let mut lop = HashSet::new();
    loop {
        dir.step(&mut curr);
        lop.insert(curr.to_flat_index(&grid));
        if curr == start {
            break;
        }
        let t = grid[curr];
        dir.turn(&t);
    }
    let width = grid.width();
    let grid = Grid::from_iter(
        grid.into_iter()
            .enumerate()
            .map(|(i, t)| if lop.contains(&i) { t } else { Tile::Ground }),
        width,
    );
    let mut sum = 0;

    for row in (0..grid.height()).map(|n| grid.get_row(n)) {
        let mut inside = false;
        for t in &row {
            if t == &Tile::Ground {
                sum += inside as usize;
            } else if matches!(t, Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest) {
                inside = !inside;
            }
        }
    }
    Some(sum)
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
        let result = part_one(
            advent_of_code::template::read_file("examples", DAY)
                .split_once("\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6_831));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let mut it = input.split("\n\n").skip(1);

        assert_eq!(part_two(it.next().unwrap()), Some(4));
        assert_eq!(part_two(it.next().unwrap()), Some(4));
        assert_eq!(part_two(it.next().unwrap()), Some(8));
        assert_eq!(part_two(it.next().unwrap()), Some(10));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(305));
    }
}
