use std::str::FromStr;

advent_of_code::solution!(12);
#[derive(Debug, PartialEq, Clone, Copy)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}
impl TryFrom<char> for Status {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            c => Err(c),
        }
    }
}
struct Group {
    springs: Vec<Status>,
    contiguous: Vec<u32>,
}
impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, contiguous) = s.split_once(' ').unwrap();
        let springs = springs
            .chars()
            .map(|c| Status::try_from(c).unwrap())
            .collect();
        let contiguous = contiguous.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(Self {
            springs,
            contiguous,
        })
    }
}

/// springs now has no operational at start or end and only one operational between the other variants
fn simplify(springs: &[Status]) -> Vec<Status> {
    let springs_original = springs;
    let mut springs = springs_original.to_vec();
    // Strip front
    if let Some((i, _)) = springs
        .iter()
        .enumerate()
        .find(|(_, s)| s != &&Status::Operational)
    {
        springs = springs.split_at(i).1.into();
    }
    // Strip back
    if let Some((i, _)) = springs
        .iter()
        .enumerate()
        .rfind(|(_, s)| s != &&Status::Operational)
    {
        springs = springs.split_at(i + 1).0.into();
    }
    let mut vec = vec![];
    let mut last = Status::Operational;
    for x in &springs {
        if last == Status::Operational && x == &last {
        } else {
            vec.push(*x);
        }
        last = *x;
    }
    springs = vec;

    debug_assert_ne!(
        *springs.first().unwrap_or(&Status::Damaged),
        Status::Operational
    );
    debug_assert_ne!(
        *springs.last().unwrap_or(&Status::Damaged),
        Status::Operational
    );
    debug_assert!(!springs
        .as_slice()
        .windows(2)
        .any(|w| w[0] == Status::Operational && w[1] == Status::Operational));
    springs
}
fn is_correct(springs: &[Status], contiguous: &[u32]) -> bool {
    springs == contiguous_to_springs(contiguous)
}
fn contiguous_to_springs(contiguous: &[u32]) -> Vec<Status> {
    let vec: Vec<Vec<_>> = contiguous
        .iter()
        .map(|n| {
            std::iter::repeat(Status::Damaged)
                .take(*n as usize)
                .collect()
        })
        .collect();
    vec.join(&Status::Operational)
}
pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let vec = vec
        .into_iter()
        .map(|g| (simplify(&g.springs), g.contiguous))
        .collect::<Vec<_>>();
    Some(
        vec.into_iter()
            .map(|(springs, contiguous)| solve(&springs, &contiguous))
            .sum(),
    )
}
fn solve(springs: &[Status], contiguous: &[u32]) -> u32 {
    let unknown = springs
        .iter()
        .enumerate()
        .filter(|(_, s)| s == &&Status::Unknown)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..(1 << unknown.len()) {
        let replace = unsigned_to_status(i, unknown.len());
        let mut new = vec![];
        (0..springs.len()).for_each(|j| {
            if let Ok(x) = unknown.binary_search(&j) {
                new.push(replace[x]);
            } else {
                new.push(springs[j]);
            }
        });
        new = simplify(&new);
        if is_correct(&new, contiguous) {
            count += 1;
        }
    }
    count
}
fn unsigned_to_status(x: u128, len: usize) -> Vec<Status> {
    let mut vec = vec![];
    let mut x = x;
    for _ in 0..len {
        match x % 2 {
            0 => vec.push(Status::Operational),
            1 => vec.push(Status::Damaged),
            _ => unreachable!(),
        }
        x >>= 1
    }
    vec
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> Vec<Group> {
    input
        .trim()
        .lines()
        .map(|l| Group::from_str(l).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7633));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
