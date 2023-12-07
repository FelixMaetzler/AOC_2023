use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(7);
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Hand1 {
    strength: Strength,
    cards: [Card1; 5],
}
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Hand2 {
    strength: Strength,
    cards: [Card2; 5],
}
impl FromStr for Hand1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card1; 5] = s
            .chars()
            .map(|c| Card1::try_from(c).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let strength = Strength::from(cards.clone());
        Ok(Self { strength, cards })
    }
}
impl FromStr for Hand2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card2; 5] = s
            .chars()
            .map(|c| Card2::try_from(c).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let strength = from_card2(cards.clone());
        Ok(Self { strength, cards })
    }
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Ord)]
enum Card1 {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    As,
}
impl TryFrom<char> for Card1 {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::As),
            _ => Err(()),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Ord)]
enum Card2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    As,
}
impl TryFrom<char> for Card2 {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'J' => Ok(Self::Joker),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::As),
            _ => Err(()),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl<T> From<[T; 5]> for Strength
where
    T: Eq + std::hash::Hash,
{
    fn from(value: [T; 5]) -> Self {
        let map = value.into_iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        match &map.len() {
            1 => Self::FiveOfAKind,
            2 if map.values().any(|v| v == &4) => Self::FourOfAKind,
            2 if map.values().any(|v| v == &3) => Self::FullHouse,
            3 if map.values().any(|v| v == &3) => Self::ThreeOfAKind,
            3 => Self::TwoPair,
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

fn from_card2(value: [Card2; 5]) -> Strength {
    let mut map = value.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let joker_count = *map.get(&Card2::Joker).unwrap_or(&0);
    map.remove(&Card2::Joker);
    let most_card_count = *map
        .iter()
        .filter(|(c, _)| c != &&&Card2::Joker)
        .map(|(_, i)| i)
        .max()
        .unwrap_or(&0);
    match (joker_count, map.len()) {
        (5, _) => Strength::FiveOfAKind,
        (4, _) => Strength::FiveOfAKind,
        (3, 1) => Strength::FiveOfAKind,
        (3, 2) => Strength::FourOfAKind,
        (2, 1) => Strength::FiveOfAKind,
        (2, 2) => Strength::FourOfAKind,
        (2, 3) => Strength::ThreeOfAKind,
        (1, 1) => Strength::FiveOfAKind,
        (1, 2) if most_card_count == 2 => Strength::FullHouse,
        (1, 2) if most_card_count == 3 => Strength::FourOfAKind,
        (1, 3) => Strength::ThreeOfAKind,
        (1, 4) => Strength::OnePair,
        (0, _) => Strength::from(value),
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec = parse1(input);
    vec.sort();
    let vec = vec;
    Some(
        vec.into_iter()
            .enumerate()
            .map(|(i, (_, b))| b * (i as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec = parse2(input);
    vec.sort();
    let vec = vec;
    Some(
        vec.into_iter()
            .enumerate()
            .map(|(i, (_, b))| b * (i as u32 + 1))
            .sum(),
    )
}
fn parse1(input: &str) -> Vec<(Hand1, u32)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let hand = Hand1::from_str(hand).unwrap();
            let bid = bid.parse().unwrap();
            (hand, bid)
        })
        .collect()
}
fn parse2(input: &str) -> Vec<(Hand2, u32)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let hand = Hand2::from_str(hand).unwrap();
            let bid = bid.parse().unwrap();
            (hand, bid)
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
