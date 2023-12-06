advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let (times, distances) = parse_part_one(input);
    Some(
        times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(t, d)| winning_count(t, d))
            .product(),
    )
}
fn winning_count(time: u64, distance: u64) -> usize {
    (0..=time).filter(|t| winnable(*t, time, distance)).count()
}
fn winnable(t: u64, max_time: u64, distance: u64) -> bool {
    let speed = t;
    let d = speed * (max_time - t);
    d > distance
}
pub fn part_two(input: &str) -> Option<usize> {
    let (time, distance) = parse_part_two(input);
    Some(winning_count(time, distance))
}
fn parse_part_one(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut it = input.trim().lines();
    let times = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();
    let distances = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();
    (times, distances)
}
fn parse_part_two(input: &str) -> (u64, u64) {
    let mut it = input.trim().lines();
    let time: String = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let distance: String = it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    (time.parse().unwrap(), distance.parse().unwrap())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
