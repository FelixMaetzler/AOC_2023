use std::vec;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| {
                let n1 = l.find(|c: char| c.is_ascii_digit()).unwrap();
                let n2 = l.rfind(|c: char| c.is_ascii_digit()).unwrap();
                let n1 = l.chars().nth(n1).unwrap().to_digit(10).unwrap();
                let n2 = l.chars().nth(n2).unwrap().to_digit(10).unwrap();
                n1 * 10 + n2
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec = vec![];
    for line in input.trim().lines() {
        let mut n1 = None;
        let mut n2 = None;
        'outer: for i in 0..line.len() {
            for j in i..line.len().min(i + 5) {
                let slice = &line[i..=j];
                if let Some(x) = parse_number(slice) {
                    n1 = Some(x);
                    break 'outer;
                }
            }
        }

        'outer: for i in (0..line.len()).rev() {
            for j in (0.max(i.saturating_sub(5))..=i).rev() {
                let slice = &line[j..=i];
                if let Some(x) = parse_number(slice) {
                    n2 = Some(x);
                    break 'outer;
                }
            }
        }
        vec.push(n1.unwrap() * 10 + n2.unwrap());
    }
    Some(vec.into_iter().sum())
}

fn parse_number(input: &str) -> Option<u32> {
    match input {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            advent_of_code::template::read_file("examples", DAY)
                .split_once("\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(142));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(54634));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            advent_of_code::template::read_file("examples", DAY)
                .split_once("\n\n")
                .unwrap()
                .1,
        );
        assert_eq!(result, Some(281));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(53855));
    }
}
