use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

advent_of_code::solution!(12);
#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}
impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
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
    contiguous: Vec<usize>,
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
impl Group {
    fn quintuple(&mut self) {
        let len = self.contiguous.len();
        self.contiguous = self
            .contiguous
            .iter()
            .cloned()
            .cycle()
            .take(5 * len)
            .collect();
        let vec = vec![self.springs.clone(); 5]
            .into_iter()
            .collect::<Vec<_>>();
        self.springs = vec.as_slice().join(&Status::Unknown);
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let vec = parse(input);
    let vec = vec
        .into_iter()
        .map(|g| ((g.springs), g.contiguous))
        .collect::<VecDeque<_>>();
    Some(
        vec.into_iter()
            .map(|(springs, contiguous)| solve(&springs, &contiguous, &mut HashMap::new()))
            .sum(),
    )
}
fn solve(
    springs: &[Status],
    contiguous: &[usize],
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Memoisation
    if let Some(x) = map.get(&(springs.len(), contiguous.len())) {
        return *x;
    }
    // Base Cases
    if contiguous.is_empty() {
        return if springs.contains(&Status::Damaged) {
            // if Damaged is left but there should be none
            0
        } else {
            // if Damaged is not there and there should be none
            1
        };
    }
    if springs.len() < contiguous.iter().sum::<usize>() + contiguous.len() - 1 {
        // There are not enough springs left to match the contiguous
        return 0;
    }
    let erg = match springs[0] {
        Status::Operational => solve(&springs[1..], contiguous, map),
        Status::Damaged => damaged(springs, contiguous, map),
        Status::Unknown => {
            damaged(springs, contiguous, map) + solve(&springs[1..], contiguous, map)
        }
    };
    map.insert((springs.len(), contiguous.len()), erg);
    erg
}
fn damaged(
    springs: &[Status],
    contiguous: &[usize],
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // not enougth springs left or not the required length of springs
    if springs.len() < contiguous[0] || springs[0..contiguous[0]].contains(&Status::Operational) {
        return 0;
    }
    // If there are exactly as many springs left as required
    if springs.len() == contiguous[0] {
        return if contiguous.len() == 1 { 1 } else { 0 };
    }
    // Check the element after the block
    if springs[contiguous[0]] == Status::Damaged {
        return 0;
    }
    solve(&springs[(contiguous[0] + 1)..], &contiguous[1..], map)
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut vec = parse(input);
    vec.iter_mut().for_each(|g| g.quintuple());
    let vec = vec;
    let vec = vec
        .into_iter()
        .map(|g| (g.springs, g.contiguous))
        .collect::<Vec<_>>();
    Some(
        vec.into_iter()
            .map(|(springs, contiguous)| solve(&springs, &contiguous, &mut HashMap::new()))
            .sum(),
    )
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
        assert_eq!(result, Some(525_152));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(23_903_579_139_437));
    }

    #[test]
    fn test_part_one_1() {
        assert_eq!(part_one("???.### 1,1,3"), Some(1));
    }
    #[test]
    fn test_part_one_2() {
        assert_eq!(part_one(".??..??...?##. 1,1,3"), Some(4));
    }
    #[test]
    fn test_part_one_3() {
        assert_eq!(part_one("?#?#?#?#?#?#?#? 1,3,1,6"), Some(1));
    }
    #[test]
    fn test_part_one_4() {
        assert_eq!(part_one("????.#...#... 4,1,1"), Some(1));
    }
    #[test]
    fn test_part_one_5() {
        assert_eq!(part_one("????.######..#####. 1,6,5"), Some(4));
    }

    #[test]
    fn test_part_one_6() {
        assert_eq!(part_one("?###???????? 3,2,1"), Some(10));
    }
    #[test]
    fn test_part_one_7() {
        assert_eq!(part_one("?.?.?????.?# 1,1,3,1"), Some(5));
    }
    #[test]
    fn test_part_one_8() {
        assert_eq!(part_one("#.######.?# 1,6,1"), Some(1));
    }
    #[test]
    fn test_part_one_9() {
        assert_eq!(part_one("###.???# 3,2"), Some(1));
    }
    #[test]
    fn test_part_one_10() {
        assert_eq!(part_one("#.#????????? 1,5"), Some(1));
    }

    //-------------

    #[test]
    fn test_part_two_1() {
        assert_eq!(part_two("???.### 1,1,3"), Some(1));
    }
    #[test]
    fn test_part_two_2() {
        assert_eq!(part_two(".??..??...?##. 1,1,3"), Some(16384));
    }
    #[test]
    fn test_part_two_3() {
        assert_eq!(part_two("?#?#?#?#?#?#?#? 1,3,1,6"), Some(1));
    }
    #[test]
    fn test_part_two_4() {
        assert_eq!(part_two("????.#...#... 4,1,1"), Some(16));
    }
    #[test]
    fn test_part_two_5() {
        assert_eq!(part_two("????.######..#####. 1,6,5"), Some(2500));
    }

    #[test]
    fn test_part_two_6() {
        assert_eq!(part_two("?###???????? 3,2,1"), Some(506250));
    }
}
