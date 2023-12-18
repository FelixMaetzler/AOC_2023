use std::str::FromStr;

advent_of_code::solution!(18);
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            val => Err(val.to_string()),
        }
    }
}
struct Instruction {
    dir: Dir,
    length: u64,
}
impl Instruction {
    fn part_1(s: &str) -> Instruction {
        let mut it = s.split_ascii_whitespace();
        let dir = Dir::from_str(it.next().unwrap()).unwrap();
        let length = it.next().unwrap().parse().unwrap();
        Self { dir, length }
    }
    fn part_2(s: &str) -> Instruction {
        let mut it = s.split_ascii_whitespace();

        let (length, dir) = parse_hex(it.nth(2).unwrap());
        Self { dir, length }
    }
}
fn parse_hex(input: &str) -> (u64, Dir) {
    let rem = input.trim_end_matches(')');
    let rem = rem.trim_start_matches("(#");
    debug_assert_eq!(rem.len(), 6);
    let length = u64::from_str_radix(&rem[0..5], 16).unwrap();
    let dir = match rem.chars().last().unwrap() {
        '0' => Dir::Right,
        '1' => Dir::Down,
        '2' => Dir::Left,
        '3' => Dir::Up,
        _ => unreachable!(),
    };
    (length, dir)
}
pub fn part_one(input: &str) -> Option<u64> {
    let vec = &parse_part_1(input);
    solve(vec)
}

pub fn part_two(input: &str) -> Option<u64> {
    let vec = &parse_part_2(input);
    solve(vec)
}
fn solve(vec: &[Instruction]) -> Option<u64> {
    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let shoelance = shoelance(vec);
    let perimeter = vec.iter().map(|i| i.length).sum::<u64>();
    assert_eq!(perimeter % 2, 0);

    Some(shoelance + perimeter / 2 + 1)
}
fn shoelance(vec: &[Instruction]) -> u64 {
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut curr = (0, 0);
    let mut points = Vec::with_capacity(vec.len());
    points.push(curr);
    for ins in vec {
        curr = match ins.dir {
            Dir::Up => (curr.0 + ins.length as i64, curr.1),
            Dir::Down => (curr.0 - ins.length as i64, curr.1),
            Dir::Left => (curr.0, curr.1 - ins.length as i64),
            Dir::Right => (curr.0, curr.1 + ins.length as i64),
        };
        points.push(curr);
    }
    let mut points = points.into_iter().rev().collect::<Vec<_>>();
    let n = points.len();
    points.push(points[0]);
    let mut sum = 0;
    for i in 0..n {
        sum += (points[i].0 + points[i + 1].0) * (points[i].1 - points[i + 1].1);
    }
    assert!(sum >= 0);
    assert_eq!(sum % 2, 0);
    sum as u64 / 2
}
fn parse_part_1(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(Instruction::part_1).collect()
}
fn parse_part_2(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(Instruction::part_2).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(39_194));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952_408_144_115));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(78_242_031_808_225));
    }
}
