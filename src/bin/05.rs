use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

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
                if let Some(i) = get_intersection(&x, k) {
                    let a = convert(&i, k, v);
                    next.push_back(a);
                    if let Some(remainder) = get_non_intersection(&x, k) {
                        curr.push_back(remainder);
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
#[derive(PartialEq, Eq, Debug)]
enum Status {
    Lower,
    IntersectingLow,
    Contained,
    IntersectingHigh,
    Higher,
}
fn status<T>(a: &Range<T>, b: &Range<T>) -> Status
where
    T: PartialOrd,
{
    if a.end <= b.start {
        return Status::Lower;
    }
    if a.end > b.start && a.start < b.start {
        return Status::IntersectingLow;
    }
    if a.start >= b.start && a.end <= b.end {
        return Status::Contained;
    }
    if a.start < b.end && a.end > b.end {
        return Status::IntersectingHigh;
    }
    if a.end >= b.start {
        return Status::Higher;
    }
    unreachable!()
}
fn get_intersection<T>(a: &Range<T>, b: &Range<T>) -> Option<Range<T>>
where
    T: PartialOrd + Clone,
{
    match status(a, b) {
        Status::Lower => None,
        Status::IntersectingLow => Some(b.clone().start..a.clone().end),
        Status::Contained => Some(a.clone()),
        Status::IntersectingHigh => Some(a.clone().start..b.clone().end),
        Status::Higher => None,
    }
}
fn get_non_intersection(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    match status(a, b) {
        Status::Lower => Some(a.clone()),
        Status::IntersectingLow => Some(a.clone().start..(b.clone().start)),
        Status::Contained => None,
        Status::IntersectingHigh => Some(b.clone().end..a.clone().end),
        Status::Higher => Some(a.clone()),
    }
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
    #[test]
    fn test_lower() {
        assert_eq!(status(&(0..1), &(1..2)), Status::Lower);
        assert_eq!(get_intersection(&(0..1), &(1..2)), None);
        assert_eq!(get_non_intersection(&(0..1), &(1..2)), Some(0..1));
    }
    #[test]
    fn test_intersection_low() {
        assert_eq!(status(&(0..5), &(3..7)), Status::IntersectingLow);
        assert_eq!(get_intersection(&(0..5), &(3..7)), Some(3..5));
        assert_eq!(get_non_intersection(&(0..5), &(3..7)), Some(0..3));
    }
    #[test]
    fn test_contained() {
        assert_eq!(status(&(3..6), &(3..7)), Status::Contained);
        assert_eq!(get_intersection(&(3..6), &(3..7)), Some(3..6));
        assert_eq!(get_non_intersection(&(3..6), &(3..7)), None);
    }
    #[test]
    fn test_intersection_high() {
        assert_eq!(status(&(5..10), &(3..7)), Status::IntersectingHigh);
        assert_eq!(get_intersection(&(5..10), &(3..7)), Some(5..7));
        assert_eq!(get_non_intersection(&(5..10), &(3..7)), Some(7..10));
    }
    #[test]
    fn test_higher() {
        assert_eq!(status(&(1..2), &(0..1)), Status::Higher);
        assert_eq!(get_intersection(&(1..2), &(0..1)), None);
        assert_eq!(get_non_intersection(&(1..2), &(0..1)), Some(1..2));
    }
}
