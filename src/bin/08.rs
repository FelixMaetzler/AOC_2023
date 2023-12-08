use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
