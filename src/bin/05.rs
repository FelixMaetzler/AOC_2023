use std::{collections::HashMap, ops::Range};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut seeds, vec) = parse(input);
    for map in vec {
        convert_vec(&mut seeds, &map);
    }
    seeds.into_iter().min()
}
fn convert_vec(vec: &mut [u64], map: &HashMap<Range<u64>, u64>) {
    for x in vec.iter_mut() {
        for (k, v) in map {
            if k.contains(x) {
                *x = (*x - k.start) + v;
                break;
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> (Vec<u64>, Vec<HashMap<Range<u64>, u64>>) {
    let (first, remainder) = input.split_once("\n\n").unwrap();
    let seeds = first
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let maps = remainder
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|l| {
                    let mut numbers = l.split_ascii_whitespace();
                    let n1: u64 = numbers.next().unwrap().parse().unwrap();
                    let n2: u64 = numbers.next().unwrap().parse().unwrap();
                    let n3: u64 = numbers.next().unwrap().parse().unwrap();
                    ((n2..n2 + n3), n1)
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(535088217));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
