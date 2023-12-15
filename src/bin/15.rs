advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    Some(vec.into_iter().map(|v| hash_algorithm(&v)).sum())
}
fn hash_algorithm(vec: &[u32]) -> u32 {
    vec.iter().fold(0, |acc, e| ((acc + *e) * 17) % 256)
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .split(',')
        .map(|b| b.chars().map(|c| c.into()).collect())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_320));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(498_538));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
