use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

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
pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = get_grid(input);
    let mut undefined = grid
        .iter()
        .enumerate()
        .filter(|(_, s)| s == &&State::Undefined)
        .map(|(i, _)| i)
        .collect::<VecDeque<_>>();

    while let Some(u) = undefined.pop_front() {
        let neigbour = grid.neighbours4(u);

        if neigbour.contains(&State::Left) {
            grid[u] = State::Left;
        } else if neigbour.contains(&State::Right) {
            grid[u] = State::Right;
        } else {
            undefined.push_back(u);
        }
    }

    let left_count = Some(
        grid.iter()
            .filter(|s| matches!(s, State::Left | State::Undefined))
            .count(),
    );
    let right_count = Some(
        grid.iter()
            .filter(|s| matches!(s, State::Right | State::Undefined))
            .count(),
    );
    if (0..grid.width()).any(|x| grid.get((0, x)).unwrap() == &State::Left) {
        return right_count;
    }
    if (0..grid.width()).any(|x| grid.get((0, x)).unwrap() == &State::Right) {
        return left_count;
    }
    if (0..grid.width()).any(|x| grid.get((grid.height() - 1, x)).unwrap() == &State::Left) {
        return right_count;
    }
    if (0..grid.width()).any(|x| grid.get((grid.height() - 1, x)).unwrap() == &State::Right) {
        return left_count;
    }
    //-------
    if (0..grid.height()).any(|y| grid.get((y, 0)).unwrap() == &State::Left) {
        return right_count;
    }
    if (0..grid.height()).any(|y| grid.get((y, 0)).unwrap() == &State::Right) {
        return left_count;
    }
    if (0..grid.height()).any(|y| grid.get((y, grid.width() - 1)).unwrap() == &State::Left) {
        return right_count;
    }
    if (0..grid.height()).any(|y| grid.get((y, grid.width() - 1)).unwrap() == &State::Right) {
        return left_count;
    }
    None
}
#[derive(PartialEq, Clone, Copy)]
enum State {
    Loop,
    Undefined,
    Left,
    Right,
}
impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Loop => write!(f, "#"),
            Self::Undefined => write!(f, "?"),
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}
fn get_grid(input: &str) -> Grid<State> {
    let grid = parse(input);
    let (start, mut dir) = start(&grid);
    let mut curr = start;
    let mut lop = HashSet::new();
    let mut dirs = vec![];

    loop {
        dir.step(&mut curr);
        lop.insert(curr);
        dirs.push(dir);
        if curr == start {
            break;
        }
        let t = grid[curr];
        dir.turn(&t);
    }
    let mut grid = Grid::from_iter(
        grid.iter().enumerate().map(|(i, _)| {
            if lop.contains(&i.to_2d_index(&grid)) {
                State::Loop
            } else {
                State::Undefined
            }
        }),
        grid.width(),
    );

    let mut curr = start;
    for dir in dirs {
        let (left, right) = match dir {
            Dir::North => (grid.get_west(curr), grid.get_east(curr)),
            Dir::South => (grid.get_east(curr), grid.get_west(curr)),
            Dir::West => (grid.get_south(curr), grid.get_north(curr)),
            Dir::East => (grid.get_north(curr), grid.get_south(curr)),
        };

        let left = left.map(|(index, s)| (index, *s));
        let right = right.map(|(index, s)| (index, *s));
        if let Some((index, s)) = left {
            if s == State::Undefined {
                grid[index] = State::Left;
            }
        }
        if let Some((index, s)) = right {
            if s == State::Undefined {
                grid[index] = State::Right;
            }
        }
        dir.step(&mut curr);
        let (left, right) = match dir {
            Dir::North => (grid.get_west(curr), grid.get_east(curr)),
            Dir::South => (grid.get_east(curr), grid.get_west(curr)),
            Dir::West => (grid.get_south(curr), grid.get_north(curr)),
            Dir::East => (grid.get_north(curr), grid.get_south(curr)),
        };

        let left = left.map(|(index, s)| (index, *s));
        let right = right.map(|(index, s)| (index, *s));
        if let Some((index, s)) = left {
            if s == State::Undefined {
                grid[index] = State::Left;
            }
        }
        if let Some((index, s)) = right {
            if s == State::Undefined {
                grid[index] = State::Right;
            }
        }
    }

    grid
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
