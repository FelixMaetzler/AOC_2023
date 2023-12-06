advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let (times, distances) = parse_part_one(input);
    Some(
        times
            .into_iter()
            .zip(distances)
            .map(|(t, d)| solve(t, d))
            .product(),
    )
}
fn solve(time: u64, distance: u64) -> u64 {
    // can be solved with PQ Formula
    // d = speed * (time - t) > distance
    // speed is in this case just t
    // t * (time - t) > distance
    // t^2 - time * t + distance < 0
    // therfore the following formula
    let time = time as f64;
    let distance = distance as f64;
    let t1 = (time / 2.0) - (((time * time) / 4.0) - distance).sqrt();
    let t2 = (time / 2.0) + (((time * time) / 4.0) - distance).sqrt();
    // we know that the function is smaller than zero between the two zeros
    // so we know we have to round t1 up and round t2 down because than we are negative (< 0)
    let t1 = t1.ceil() as u64;
    let t2 = t2.floor() as u64;
    // if t1=3 and t2=4 we have two possible solutions.
    // Therefore the +1
    t2 - t1 + 1
}
pub fn part_two(input: &str) -> Option<u64> {
    let (time, distance) = parse_part_two(input);
    Some(solve(time, distance))
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
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(608_902));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71_503));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(46_173_809));
    }
}
