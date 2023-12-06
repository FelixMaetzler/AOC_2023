use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

use advent_of_code::{Count, RangeExt};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut seeds, vec) = parse_part_1(input);
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

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_part_2(input);
    let mut curr = VecDeque::from(seeds);
    for map in &maps {
        let mut next = VecDeque::new();
        'outer: while let Some(x) = curr.pop_front() {
            for (k, v) in map {
                if let Some(i) = x.get_intersection(k) {
                    let a = convert(&i, k, v);
                    next.push_back(a);
                    match x.get_non_intersection(k) {
                        Count::None => {}
                        Count::Single(remainder) => curr.push_back(remainder),
                        Count::Double(rem1, rem2) => {
                            curr.push_back(rem1);
                            curr.push_back(rem2);
                        }
                    }

                    continue 'outer;
                }
            }
            next.push_back(x);
        }
        curr = next;
    }
    curr.into_iter().map(|r| r.start).min()
}
fn convert(to_be_converted: &Range<u64>, input: &Range<u64>, out: &u64) -> Range<u64> {
    (to_be_converted.start + out - input.start)..(to_be_converted.end + out - input.start)
}
fn parse_part_1(input: &str) -> (Vec<u64>, Vec<HashMap<Range<u64>, u64>>) {
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
#[allow(clippy::type_complexity)]
fn parse_part_2(input: &str) -> (Vec<Range<u64>>, Vec<HashMap<Range<u64>, u64>>) {
    let (first, remainder) = input.split_once("\n\n").unwrap();
    let seeds: Vec<_> = first
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let seeds = seeds
        .as_slice()
        .chunks_exact(2)
        .map(|s| (s[0]..(s[0] + s[1])))
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
        assert_eq!(result, Some(535_088_217));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(51_399_228));
    }
}
