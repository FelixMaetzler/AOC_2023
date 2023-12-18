use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(18);
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn step(&self, x: (i32, i32)) -> (i32, i32) {
        match self {
            Dir::Up => (x.0 + 1, x.1),
            Dir::Down => (x.0 - 1, x.1),
            Dir::Left => (x.0, x.1 - 1),
            Dir::Right => (x.0, x.1 + 1),
        }
    }
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
    length: u32,
    color_code: String,
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let dir = Dir::from_str(it.next().unwrap()).unwrap();
        let length = it.next().unwrap().parse().unwrap();
        Ok(Self {
            dir,
            length,
            color_code: it.next().unwrap().to_string(),
        })
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let vec = parse(input);
    let mut map = HashSet::new();
    let mut curr = (0, 0);
    map.insert(curr);
    for instruction in &vec {
        for _ in 0..instruction.length {
            curr = instruction.dir.step(curr);
            map.insert(curr);
        }
    }
    curr = (0, 0);
    curr = vec[0].dir.step(curr);
    curr = vec[1].dir.step(curr);
    let inside = curr;
    assert!(!map.contains(&inside));
    flood_fill(&mut map, inside);
    Some(map.len())
}
fn flood_fill(map: &mut HashSet<(i32, i32)>, node: (i32, i32)) {
    let mut queue = Vec::new();
    queue.push(node);
    while let Some((y, x)) = queue.pop() {
        map.insert((y, x));
        let next = (y + 1, x);
        if !map.contains(&next) {
            queue.push(next);
        }
        let next = (y - 1, x);
        if !map.contains(&next) {
            queue.push(next);
        }
        let next = (y, x + 1);
        if !map.contains(&next) {
            queue.push(next);
        }
        let next = (y, x - 1);
        if !map.contains(&next) {
            queue.push(next);
        }
    }
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
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
        assert_eq!(result, None);
    }
}
