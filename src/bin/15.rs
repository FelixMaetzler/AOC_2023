advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let map = vec.into_iter().map(hash_algorithm);
    Some(map.sum())
}
fn hash_algorithm(vec: &str) -> u32 {
    vec.chars().fold(0, |acc, e| ((acc + e as u32) * 17) % 256)
}
pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);
    let mut map: Vec<Option<Vec<(&str, u32)>>> = vec![None; 256];
    for s in vec {
        if let Some((label, number)) = s.split_once('=') {
            let hash = hash_algorithm(label) as usize;
            if let Some(b) = map.get_mut(hash).unwrap() {
                if let Some((i, _)) = b.iter().enumerate().find(|(_, (l, _))| l == &label) {
                    b[i].1 = number.parse().unwrap();
                } else {
                    b.push((label, number.parse().unwrap()));
                }
            } else {
                map[hash] = Some(vec![(label, number.parse().unwrap())]);
            }
        } else if let Some((label, _)) = s.split_once('-') {
            let hash = hash_algorithm(label) as usize;
            if let Some(b) = map.get_mut(hash).unwrap() {
                b.retain(|(l, _)| l != &label);
            }
        } else {
            unreachable!()
        }
    }

    Some(calc(&map))
}
fn calc(map: &[Option<Vec<(&str, u32)>>]) -> u32 {
    map.iter()
        .enumerate()
        .filter(|(_, e)| e.is_some())
        .map(|(i, e)| {
            e.as_ref()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(j, (_, n))| n * (j as u32 + 1) * (i as u32 + 1))
                .sum::<u32>()
        })
        .sum()
}
fn parse(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
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
        assert_eq!(result, Some(145));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(286_278));
    }
}
