use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, map) = parse(input);
    let mut it = instructions.into_iter().cycle();
    let mut curr = &Data::from_str("AAA");
    let mut c = 0;
    while curr != &Data::from_str("ZZZ") {
        let node = map.get(curr).unwrap();
        curr = match it.next().unwrap() {
            Dir::Left => &node.left,
            Dir::Right => &node.right,
        };
        c += 1;
    }
    Some(c)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, map) = parse(input);
    let mut curr = map
        .keys()
        .filter(|s| s.pos == Pos::Start)
        .collect::<Vec<_>>();
    let mut curr_lcm = 1;

    for x in curr.iter_mut() {
        let mut it = instructions.iter().cycle();
        let mut c = 0;
        while x.pos != Pos::End {
            c += 1;
            let dir = it.next().unwrap();
            let node = map.get(*x).unwrap();
            *x = match dir {
                Dir::Left => &node.left,
                Dir::Right => &node.right,
            };
        }
        curr_lcm = lcm(curr_lcm, c as u64);
    }
    Some(curr_lcm)
}
fn lcm(x: u64, y: u64) -> u64 {
    (x * y) / gcd(x, y)
}
fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    loop {
        let h = a % b;
        a = b;
        b = h;
        if b == 0 {
            return a;
        }
    }
}
#[derive(Eq, PartialEq, Hash)]
struct Data<'a> {
    data: &'a str,
    pos: Pos,
}

impl<'a> Data<'a> {
    fn from_str(s: &'a str) -> Self {
        match s.chars().last().unwrap() {
            'A' => Self {
                data: s,
                pos: Pos::Start,
            },
            'Z' => Self {
                data: s,
                pos: Pos::End,
            },
            _ => Self {
                data: s,
                pos: Pos::Middle,
            },
        }
    }
}
struct Node<'a> {
    left: Data<'a>,
    right: Data<'a>,
}
impl<'a> Node<'a> {
    fn from_str(s: &'a str) -> Result<Self, ()> {
        let (n1, n2) = s.split_once(", ").unwrap();
        let n1 = &n1[1..];
        let n2 = &n2[..n2.len() - 1];
        Ok(Self {
            left: Data::from_str(n1),
            right: Data::from_str(n2),
        })
    }
}
fn parse(input: &str) -> (Vec<Dir>, HashMap<Data, Node>) {
    let (left, right) = input.trim().split_once("\n\n").unwrap();
    let left = left.chars().map(|c| Dir::try_from(c).unwrap()).collect();
    let map = right
        .lines()
        .map(|l| {
            let (k, rem) = l.split_once(" = ").unwrap();
            let k = Data::from_str(k);
            let data = Node::from_str(rem).unwrap();
            (k, data)
        })
        .collect();
    (left, map)
}
#[derive(Eq, PartialEq, Hash)]
enum Pos {
    Start,
    Middle,
    End,
}
#[derive(Clone)]
enum Dir {
    Left,
    Right,
}
impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(18_023));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(14_449_445_933_179));
    }
}
