use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, map) = parse(input);
    let mut it = instructions.chars().cycle();
    let mut curr = "AAA";
    let mut c = 0;
    while curr != "ZZZ" {
        let (n1, n2) = map.get(curr).unwrap();
        curr = match it.next().unwrap() {
            'L' => n1,
            'R' => n2,
            _ => unreachable!(),
        };
        c += 1;
    }
    Some(c)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, map) = parse(input);
    let mut curr = map.keys().filter(|s| s.ends_with('A')).collect::<Vec<_>>();
    let mut curr_lcm = 1;

    for x in curr.iter_mut() {
        let mut it = instructions.chars().cycle();
        let mut c = 0;
        loop {
            let dir = it.next().unwrap();
            let (n1, n2) = map.get(*x).unwrap();
            *x = match dir {
                'L' => n1,
                'R' => n2,
                _ => unreachable!(),
            };
            c += 1;
            if x.ends_with('Z') {
                curr_lcm = lcm(curr_lcm, c as u64);
                break;
            }
        }
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

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (left, right) = input.trim().split_once("\n\n").unwrap();
    let map = right
        .lines()
        .map(|l| {
            let (k, rem) = l.split_once(" = ").unwrap();
            let (n1, n2) = rem.split_once(", ").unwrap();
            let n1 = n1.trim_matches('(');
            let n2 = n2.trim_matches(')');
            (k, (n1, n2))
        })
        .collect();
    (left, map)
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
