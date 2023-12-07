use std::collections::HashMap;
advent_of_code::solution!(7);
#[derive(Debug)]
struct Hand {
    strength: Strength,
    card_values: [u32; 5],
    bid: u32,
}
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl From<&HashMap<char, u32>> for Strength {
    fn from(value: &HashMap<char, u32>) -> Self {
        use Strength::*;
        let mut vec: Vec<_> = value.values().collect();
        vec.sort_unstable_by(|a, b| b.cmp(a));
        let str: String = vec.iter().map(|i| i.to_string()).collect();
        match str.as_str() {
            "5" => FiveOfAKind,
            "41" => FourOfAKind,
            "32" => FullHouse,
            "311" => ThreeOfAKind,
            "221" => TwoPair,
            "2111" => OnePair,
            "11111" => HighCard,
            val => unreachable!("{val} not covered"),
        }
    }
}
fn to_hand(cards: [char; 5], bid: u32, part_1: bool) -> Hand {
    let mut map = cards.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(*c).or_insert(0) += 1;
        acc
    });
    if !part_1 {
        // Adding all Jokers to the Card that occurs the most
        // If all are Jokers, then just take these Jokers
        if let Some(j) = map.remove(&'J') {
            if let Some(max) = map.values_mut().max() {
                *max += j;
            } else {
                map.insert('J', 5);
            }
        }
    }
    let strength = Strength::from(&map);
    let card_values = cards
        .iter()
        .map(|c| match c {
            '0'..='9' => c.to_digit(10).unwrap(),
            'T' => 10,
            'J' if part_1 => 11,
            'J' if !part_1 => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    Hand {
        strength,
        bid,
        card_values,
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, false)
}
fn solve(input: &str, part_1: bool) -> Option<u32> {
    let vec = parse(input);
    let mut vec: Vec<_> = vec
        .into_iter()
        .map(|(cards, bid)| to_hand(cards, bid, part_1))
        .collect();
    vec.sort_unstable_by_key(|h| (h.strength, h.card_values));
    Some(
        vec.iter()
            .enumerate()
            .map(|(i, n)| (i + 1) as u32 * n.bid)
            .sum(),
    )
}
fn parse(input: &str) -> Vec<([char; 5], u32)> {
    input
        .trim()
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(v, n)| {
            (
                v.chars().collect::<Vec<_>>().try_into().unwrap(),
                n.parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(248_812_215));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(250_057_090));
    }
}
