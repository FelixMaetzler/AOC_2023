advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let vec = parse(input);
    Some(vec.into_iter().map(|v| recurse_part_one(&v)).sum())
}
fn reduction(x: &[i32]) -> Vec<i32> {
    x.windows(2).map(|s| s[1] - s[0]).collect()
}
fn recurse_part_one(x: &[i32]) -> i32 {
    if x.iter().all(|n| n == &0) {
        return 0;
    }
    x.last().unwrap() + recurse_part_one(&reduction(x))
}
fn recurse_part_two(x: &[i32]) -> i32 {
    if x.iter().all(|n| n == &0) {
        return 0;
    }
    x.first().unwrap() - recurse_part_two(&reduction(x))
}
pub fn part_two(input: &str) -> Option<i32> {
    let vec = parse(input);
    Some(vec.into_iter().map(|v| recurse_part_two(&v)).sum())
}
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1_934_898_178));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1_129));
    }
}
